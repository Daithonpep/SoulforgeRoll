use std::sync::Arc;
use warp::ws::{Message, WebSocket};
use warp::Reply;
use futures::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use serde::Deserialize;

use crate::room::*;
use crate::protocol::*;

// ============================================================
// HTTP HANDLERS
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub config: Option<RoomConfig>,
}

pub async fn create_room_handler(
    body: CreateRoomRequest,
    room_manager: Arc<RoomManager>,
) -> Result<impl Reply, warp::Rejection> {
    let room = room_manager.create_room(body.name, body.config);
    
    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "room_id": room.id,
        "room_name": room.name,
        "created_at": room.created_at.to_rfc3339(),
    })))
}

pub async fn room_info_handler(
    room_id: String,
    room_manager: Arc<RoomManager>,
) -> Result<impl Reply, warp::Rejection> {
    match room_manager.get_room(&room_id) {
        Some(room) => Ok(warp::reply::json(&serde_json::json!({
            "exists": true,
            "name": room.name,
            "phase": room.phase,
            "player_count": room.participants.len(),
            "max_players": room.config.max_players,
            "has_dm": room.dm_connection_id.is_some(),
        }))),
        None => Ok(warp::reply::json(&serde_json::json!({
            "exists": false,
        }))),
    }
}

// ============================================================
// WEBSOCKET HANDLER
// ============================================================

pub async fn ws_handler(
    room_id: String,
    ws: warp::ws::Ws,
    room_manager: Arc<RoomManager>,
) -> Result<impl Reply, warp::Rejection> {
    // Verificar que la sala existe
    if !room_manager.room_exists(&room_id) {
        return Ok(ws.on_upgrade(|websocket| async {
            let (mut tx, _) = websocket.split();
            let _ = tx.send(Message::text(
                serde_json::to_string(&ServerMessage::Error {
                    code: "room_not_found".to_string(),
                    message: "La sala no existe".to_string(),
                }).unwrap()
            )).await;
        }));
    }
    
    Ok(ws.on_upgrade(move |socket| {
        handle_connection(socket, room_id, room_manager)
    }))
}

async fn handle_connection(
    ws: WebSocket,
    room_id: String,
    room_manager: Arc<RoomManager>,
) {
    let connection_id = room_manager.next_connection_id();
    let (mut ws_tx, mut ws_rx) = ws.split();
    
    // Canal para mensajes salientes
    let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);
    
    // Task para enviar mensajes
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_tx.send(Message::text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Procesar mensajes entrantes
    let mut user_name: Option<String> = None;
    
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                        let response = process_message(
                            &room_manager,
                            &room_id,
                            connection_id,
                            &mut user_name,
                            client_msg,
                        ).await;
                        
                        if let Some(server_msg) = response {
                            let _ = tx.send(server_msg).await;
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }
    
    // Limpiar al desconectar
    if user_name.is_some() {
        room_manager.leave_room(&room_id, connection_id);
        tracing::info!("Connection {} disconnected from room {}", connection_id, room_id);
    }
    
    send_task.abort();
}

async fn process_message(
    room_manager: &RoomManager,
    room_id: &str,
    connection_id: ConnectionId,
    user_name: &mut Option<String>,
    message: ClientMessage,
) -> Option<ServerMessage> {
    match message {
        ClientMessage::Join { user_name: name, role } => {
            match room_manager.join_room(room_id, connection_id, name.clone(), role) {
                Ok(participant) => {
                    *user_name = Some(name);
                    
                    let room = room_manager.get_room(room_id)?;
                    
                    Some(ServerMessage::Welcome {
                        connection_id,
                        room: room.to_snapshot(connection_id),
                    })
                }
                Err(e) => Some(ServerMessage::Error {
                    code: "join_failed".to_string(),
                    message: e.to_string(),
                }),
            }
        }
        
        ClientMessage::LoadCharacter { character } => {
            match room_manager.load_character(room_id, connection_id, character.clone()) {
                Ok(_) => Some(ServerMessage::CharacterLoaded {
                    connection_id,
                    character_name: character.name,
                    portrait: character.portrait,
                }),
                Err(e) => Some(ServerMessage::Error {
                    code: "load_failed".to_string(),
                    message: e.to_string(),
                }),
            }
        }
        
        ClientMessage::MoveToken { position } => {
            match room_manager.move_token(room_id, connection_id, position.clone()) {
                Ok(_) => Some(ServerMessage::TokenMoved {
                    connection_id,
                    position,
                }),
                Err(e) => Some(ServerMessage::Error {
                    code: "move_failed".to_string(),
                    message: e.to_string(),
                }),
            }
        }
        
        ClientMessage::StartGame => {
            match room_manager.set_game_phase(room_id, connection_id, GamePhase::Active) {
                Ok(_) => Some(ServerMessage::GamePhaseChanged {
                    phase: GamePhase::Active,
                }),
                Err(e) => Some(ServerMessage::Error {
                    code: "phase_change_failed".to_string(),
                    message: e.to_string(),
                }),
            }
        }
        
        ClientMessage::RevealArea { x, y, radius } => {
            let area = RevealedArea { x, y, radius };
            match room_manager.reveal_area(room_id, connection_id, area.clone()) {
                Ok(_) => Some(ServerMessage::AreaRevealed { area }),
                Err(e) => Some(ServerMessage::Error {
                    code: "reveal_failed".to_string(),
                    message: e.to_string(),
                }),
            }
        }
        
        ClientMessage::Chat { message } => {
            let is_dm = room_manager.get_room(room_id)
                .map(|r| r.dm_connection_id == Some(connection_id))
                .unwrap_or(false);
            
            Some(ServerMessage::ChatMessage {
                from: user_name.clone().unwrap_or_else(|| "???".to_string()),
                message,
                is_dm,
                timestamp: chrono::Utc::now().timestamp_millis(),
            })
        }
        
        ClientMessage::Ping => Some(ServerMessage::Pong {
            server_time: chrono::Utc::now().timestamp_millis(),
        }),
        
        ClientMessage::RequestSync => {
            room_manager.get_room(room_id)
                .map(|room| ServerMessage::FullSync {
                    room: room.to_snapshot(connection_id),
                })
        }
        
        _ => None,
    }
}
