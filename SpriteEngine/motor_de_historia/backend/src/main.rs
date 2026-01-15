mod room;
mod protocol;
mod handlers;

use std::env;
use std::sync::Arc;
use warp::Filter;
use tracing_subscriber;

use crate::room::RoomManager;
use crate::handlers::{ws_handler, create_room_handler, room_info_handler};

#[tokio::main]
async fn main() {
    // Inicializar logging
    tracing_subscriber::fmt::init();
    
    // Puerto desde variable de entorno (Railway lo proporciona)
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    
    // Estado compartido
    let room_manager = Arc::new(RoomManager::new());
    
    // Clonar para cada filtro
    let rm_ws = room_manager.clone();
    let rm_create = room_manager.clone();
    let rm_info = room_manager.clone();
    
    // CORS para Vercel
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);
    
    // === RUTAS ===
    
    // WebSocket: /ws/{room_id}
    let ws_route = warp::path("ws")
        .and(warp::path::param::<String>())
        .and(warp::ws())
        .and(warp::any().map(move || rm_ws.clone()))
        .and_then(ws_handler);
    
    // Crear sala: POST /api/rooms
    let create_route = warp::path!("api" / "rooms")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || rm_create.clone()))
        .and_then(create_room_handler);
    
    // Info de sala: GET /api/rooms/{room_id}
    let info_route = warp::path!("api" / "rooms" / String)
        .and(warp::get())
        .and(warp::any().map(move || rm_info.clone()))
        .and_then(room_info_handler);
    
    // Health check
    let health = warp::path!("health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "alive"})));
    
    let routes = ws_route
        .or(create_route)
        .or(info_route)
        .or(health)
        .with(cors);
    
    tracing::info!("SoulForge Server starting on port {}", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
