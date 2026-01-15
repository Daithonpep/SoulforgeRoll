use serde::{Deserialize, Serialize};
use crate::room::*;

// ============================================================
// MENSAJES CLIENTE -> SERVIDOR
// ============================================================

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum ClientMessage {
    // === Conexión ===
    Join {
        user_name: String,
        role: UserRole,
    },
    
    // === Ficha ===
    LoadCharacter {
        character: CharacterEssence,
    },
    
    // === Movimiento ===
    MoveToken {
        position: Position,
    },
    
    // === Chat/Comandos ===
    Chat {
        message: String,
    },
    Command {
        command: String,
        args: Vec<String>,
    },
    
    // === DM Controls ===
    StartGame,
    PauseGame,
    EndGame,
    RevealArea {
        x: f32,
        y: f32,
        radius: f32,
    },
    LoadMap {
        map_id: String,
        name: String,
        width: u32,
        height: u32,
    },
    KickPlayer {
        connection_id: u64,
    },
    
    // === Estado ===
    SetPlayerState {
        state: PlayerState,
    },
    UpdateToken {
        updates: TokenUpdate,
    },
    
    // === Sync ===
    Ping,
    RequestSync,
}

#[derive(Debug, Deserialize)]
pub struct TokenUpdate {
    pub conditions: Option<Vec<String>>,
    pub visible: Option<bool>,
    pub size: Option<f32>,
}

// ============================================================
// MENSAJES SERVIDOR -> CLIENTE
// ============================================================

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum ServerMessage {
    // === Conexión ===
    Welcome {
        connection_id: u64,
        room: RoomSnapshot,
    },
    PlayerJoined {
        participant: ParticipantSnapshot,
    },
    PlayerLeft {
        connection_id: u64,
        user_name: String,
    },
    
    // === Estado ===
    PlayerStateChanged {
        connection_id: u64,
        new_state: PlayerState,
    },
    CharacterLoaded {
        connection_id: u64,
        character_name: String,
        portrait: Option<String>,
    },
    
    // === Juego ===
    GamePhaseChanged {
        phase: GamePhase,
    },
    TokenMoved {
        connection_id: u64,
        position: Position,
    },
    TokenUpdated {
        connection_id: u64,
        token: Token,
    },
    AreaRevealed {
        area: RevealedArea,
    },
    MapLoaded {
        map_state: MapState,
    },
    
    // === Chat ===
    ChatMessage {
        from: String,
        message: String,
        is_dm: bool,
        timestamp: i64,
    },
    SystemMessage {
        message: String,
        severity: String, // "info", "warning", "error", "success"
    },
    
    // === Comandos DM ===
    DmAnnouncement {
        title: String,
        content: String,
    },
    RollResult {
        roller: String,
        dice: String,
        results: Vec<i32>,
        total: i32,
        is_public: bool,
    },
    
    // === Sync ===
    Pong {
        server_time: i64,
    },
    FullSync {
        room: RoomSnapshot,
    },
    
    // === Errores ===
    Error {
        code: String,
        message: String,
    },
}

// ============================================================
// SNAPSHOTS (versiones serializables para el cliente)
// ============================================================

#[derive(Debug, Clone, Serialize)]
pub struct RoomSnapshot {
    pub id: String,
    pub name: String,
    pub phase: GamePhase,
    pub config: RoomConfig,
    pub participants: Vec<ParticipantSnapshot>,
    pub map_state: Option<MapState>,
    pub is_dm: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParticipantSnapshot {
    pub connection_id: u64,
    pub user_name: String,
    pub role: UserRole,
    pub state: PlayerState,
    pub character_name: Option<String>,
    pub character_portrait: Option<String>,
    pub token: Option<Token>,
}

impl From<&Participant> for ParticipantSnapshot {
    fn from(p: &Participant) -> Self {
        Self {
            connection_id: p.connection_id,
            user_name: p.user_name.clone(),
            role: p.role.clone(),
            state: p.state.clone(),
            character_name: p.character.as_ref().map(|c| c.name.clone()),
            character_portrait: p.character.as_ref().and_then(|c| c.portrait.clone()),
            token: p.token.clone(),
        }
    }
}

impl Room {
    pub fn to_snapshot(&self, for_connection: u64) -> RoomSnapshot {
        let is_dm = self.dm_connection_id == Some(for_connection);
        
        RoomSnapshot {
            id: self.id.clone(),
            name: self.name.clone(),
            phase: self.phase.clone(),
            config: self.config.clone(),
            participants: self.participants.values()
                .map(ParticipantSnapshot::from)
                .collect(),
            map_state: self.map_state.clone(),
            is_dm,
        }
    }
}
