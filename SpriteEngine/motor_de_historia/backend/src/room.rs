use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use rand::Rng;

// ============================================================
// TIPOS DE DATOS
// ============================================================

/// Identificador único de sala (ej: "SF-X9J2")
pub type RoomId = String;

/// Identificador de conexión WebSocket
pub type ConnectionId = u64;

/// Rol del usuario en la sala
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    DungeonMaster,
    Player,
    Spectator,
}

/// Estado de un jugador en la sala
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlayerState {
    Connecting,      // Acaba de unirse
    LoadingSheet,    // Cargando su ficha
    Ready,           // Ficha cargada y validada
    InGame,          // Jugando activamente
    Away,            // AFK
    Disconnected,    // Perdió conexión (puede reconectar)
}

/// Posición en el mapa
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub layer: u8,  // Capa del mapa (0 = suelo, 1 = objetos, etc.)
}

/// Token de un jugador en el mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub portrait_url: Option<String>,
    pub position: Position,
    pub size: f32,           // Radio del token
    pub color: String,       // Color asignado (#RRGGBB)
    pub visible_to_all: bool,
    pub conditions: Vec<String>,  // Estados: "herido", "oculto", etc.
}

/// Datos esenciales del personaje (extraídos del JSON de la ficha)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEssence {
    pub name: String,
    pub portrait: Option<String>,
    pub tier: String,
    pub tension_level: u8,
    pub current_wound: Option<String>,
    pub attributes: HashMap<String, i32>,
    pub special_traits: Vec<String>,
}

/// Un participante en la sala
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub connection_id: ConnectionId,
    pub user_name: String,
    pub role: UserRole,
    pub state: PlayerState,
    pub character: Option<CharacterEssence>,
    pub token: Option<Token>,
    pub joined_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Estado de la partida
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    Lobby,           // Esperando jugadores
    Preparation,     // DM configurando
    Active,          // Partida en curso
    Paused,          // Pausada
    Ended,           // Terminada
}

/// Configuración de la sala
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub max_players: usize,
    pub allow_spectators: bool,
    pub fog_of_war: bool,
    pub grid_enabled: bool,
    pub grid_size: u32,
    pub auto_save_interval_secs: u64,
}

impl Default for RoomConfig {
    fn default() -> Self {
        Self {
            max_players: 6,
            allow_spectators: true,
            fog_of_war: true,
            grid_enabled: true,
            grid_size: 64,
            auto_save_interval_secs: 300, // 5 minutos
        }
    }
}

/// Datos del mapa actual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapState {
    pub map_id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub revealed_areas: Vec<RevealedArea>,  // Áreas sin niebla
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealedArea {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

/// Una sala de juego completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub config: RoomConfig,
    pub phase: GamePhase,
    pub dm_connection_id: Option<ConnectionId>,
    pub participants: HashMap<ConnectionId, Participant>,
    pub map_state: Option<MapState>,
    pub session_log: Vec<SessionEvent>,
}

/// Evento del log de sesión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub description: String,
    pub actor: Option<String>,
}

// ============================================================
// GENERADOR DE CÓDIGOS
// ============================================================

pub struct RoomCodeGenerator;

impl RoomCodeGenerator {
    /// Genera un código único de sala tipo "SF-X9J2"
    pub fn generate() -> RoomId {
        let mut rng = rand::thread_rng();
        
        // 4 caracteres alfanuméricos (sin vocales para evitar palabras)
        const CHARS: &[u8] = b"BCDFGHJKLMNPQRSTVWXYZ0123456789";
        
        let code: String = (0..4)
            .map(|_| {
                let idx = rng.gen_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect();
        
        format!("SF-{}", code)
    }
}

// ============================================================
// COLORES PARA TOKENS
// ============================================================

pub struct TokenColorAssigner;

impl TokenColorAssigner {
    const COLORS: &'static [&'static str] = &[
        "#E63946", // Rojo
        "#457B9D", // Azul
        "#2A9D8F", // Verde azulado
        "#E9C46A", // Amarillo
        "#9B5DE5", // Púrpura
        "#F15BB5", // Rosa
        "#00BBF9", // Cyan
        "#F77F00", // Naranja
    ];
    
    /// Asigna un color basado en el orden de llegada
    pub fn assign(player_index: usize) -> String {
        Self::COLORS[player_index % Self::COLORS.len()].to_string()
    }
}

// ============================================================
// ROOM MANAGER
// ============================================================

pub struct RoomManager {
    rooms: DashMap<RoomId, Room>,
    connection_counter: std::sync::atomic::AtomicU64,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
            connection_counter: std::sync::atomic::AtomicU64::new(1),
        }
    }
    
    /// Genera un nuevo ID de conexión
    pub fn next_connection_id(&self) -> ConnectionId {
        self.connection_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Crea una nueva sala
    pub fn create_room(&self, name: String, config: Option<RoomConfig>) -> Room {
        let mut room_id = RoomCodeGenerator::generate();
        
        // Asegurar unicidad
        while self.rooms.contains_key(&room_id) {
            room_id = RoomCodeGenerator::generate();
        }
        
        let room = Room {
            id: room_id.clone(),
            name,
            created_at: Utc::now(),
            config: config.unwrap_or_default(),
            phase: GamePhase::Lobby,
            dm_connection_id: None,
            participants: HashMap::new(),
            map_state: None,
            session_log: vec![SessionEvent {
                timestamp: Utc::now(),
                event_type: "room_created".to_string(),
                description: "La sala ha sido creada".to_string(),
                actor: None,
            }],
        };
        
        self.rooms.insert(room_id.clone(), room.clone());
        
        tracing::info!("Room created: {}", room_id);
        
        room
    }
    
    /// Obtiene una sala por ID
    pub fn get_room(&self, room_id: &str) -> Option<Room> {
        self.rooms.get(room_id).map(|r| r.clone())
    }
    
    /// Verifica si una sala existe
    pub fn room_exists(&self, room_id: &str) -> bool {
        self.rooms.contains_key(room_id)
    }
    
    /// Añade un participante a la sala
    pub fn join_room(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        user_name: String,
        role: UserRole,
    ) -> Result<Participant, &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        // Verificar capacidad
        let player_count = room.participants.values()
            .filter(|p| p.role == UserRole::Player)
            .count();
        
        if role == UserRole::Player && player_count >= room.config.max_players {
            return Err("Room is full");
        }
        
        if role == UserRole::Spectator && !room.config.allow_spectators {
            return Err("Spectators not allowed");
        }
        
        // Asignar color al token
        let color = TokenColorAssigner::assign(room.participants.len());
        
        let participant = Participant {
            connection_id,
            user_name: user_name.clone(),
            role: role.clone(),
            state: PlayerState::Connecting,
            character: None,
            token: Some(Token {
                id: format!("token_{}", connection_id),
                name: user_name.clone(),
                portrait_url: None,
                position: Position::default(),
                size: 1.0,
                color,
                visible_to_all: true,
                conditions: vec![],
            }),
            joined_at: Utc::now(),
            last_activity: Utc::now(),
        };
        
        // Si es DM, registrarlo
        if role == UserRole::DungeonMaster {
            room.dm_connection_id = Some(connection_id);
        }
        
        room.participants.insert(connection_id, participant.clone());
        
        // Log del evento
        room.session_log.push(SessionEvent {
            timestamp: Utc::now(),
            event_type: "player_joined".to_string(),
            description: format!("{} se ha unido como {:?}", user_name, role),
            actor: Some(user_name),
        });
        
        tracing::info!("Player {} joined room {} as {:?}", connection_id, room_id, role);
        
        Ok(participant)
    }
    
    /// Remueve un participante de la sala
    pub fn leave_room(&self, room_id: &str, connection_id: ConnectionId) -> Option<Participant> {
        let mut room = self.rooms.get_mut(room_id)?;
        
        let participant = room.participants.remove(&connection_id)?;
        
        // Si era el DM, limpiar referencia
        if room.dm_connection_id == Some(connection_id) {
            room.dm_connection_id = None;
        }
        
        room.session_log.push(SessionEvent {
            timestamp: Utc::now(),
            event_type: "player_left".to_string(),
            description: format!("{} ha abandonado la sala", participant.user_name),
            actor: Some(participant.user_name.clone()),
        });
        
        // Si no quedan participantes, marcar para limpieza
        if room.participants.is_empty() {
            room.phase = GamePhase::Ended;
        }
        
        Some(participant)
    }
    
    /// Actualiza el estado de un participante
    pub fn update_participant_state(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        new_state: PlayerState,
    ) -> Result<(), &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        let participant = room.participants.get_mut(&connection_id)
            .ok_or("Participant not found")?;
        
        participant.state = new_state;
        participant.last_activity = Utc::now();
        
        Ok(())
    }
    
    /// Carga los datos del personaje de un participante
    pub fn load_character(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        character: CharacterEssence,
    ) -> Result<(), &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        let participant = room.participants.get_mut(&connection_id)
            .ok_or("Participant not found")?;
        
        // Actualizar token con datos del personaje
        if let Some(ref mut token) = participant.token {
            token.name = character.name.clone();
            token.portrait_url = character.portrait.clone();
        }
        
        participant.character = Some(character);
        participant.state = PlayerState::Ready;
        participant.last_activity = Utc::now();
        
        room.session_log.push(SessionEvent {
            timestamp: Utc::now(),
            event_type: "character_loaded".to_string(),
            description: format!("{} ha cargado su personaje", participant.user_name),
            actor: Some(participant.user_name.clone()),
        });
        
        Ok(())
    }
    
    /// Mueve un token
    pub fn move_token(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        new_position: Position,
    ) -> Result<Position, &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        let participant = room.participants.get_mut(&connection_id)
            .ok_or("Participant not found")?;
        
        let token = participant.token.as_mut()
            .ok_or("No token assigned")?;
        
        token.position = new_position.clone();
        participant.last_activity = Utc::now();
        
        Ok(new_position)
    }
    
    /// Cambia la fase del juego (solo DM)
    pub fn set_game_phase(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        new_phase: GamePhase,
    ) -> Result<(), &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        // Verificar que es el DM
        if room.dm_connection_id != Some(connection_id) {
            return Err("Only DM can change game phase");
        }
        
        room.phase = new_phase.clone();
        
        room.session_log.push(SessionEvent {
            timestamp: Utc::now(),
            event_type: "phase_changed".to_string(),
            description: format!("La partida ha pasado a fase: {:?}", new_phase),
            actor: Some("DM".to_string()),
        });
        
        Ok(())
    }
    
    /// Revela un área del mapa (quita niebla)
    pub fn reveal_area(
        &self,
        room_id: &str,
        connection_id: ConnectionId,
        area: RevealedArea,
    ) -> Result<(), &'static str> {
        let mut room = self.rooms.get_mut(room_id)
            .ok_or("Room not found")?;
        
        // Verificar que es el DM
        if room.dm_connection_id != Some(connection_id) {
            return Err("Only DM can reveal areas");
        }
        
        if let Some(ref mut map) = room.map_state {
            map.revealed_areas.push(area);
        }
        
        Ok(())
    }
    
    /// Obtiene lista de participantes listos
    pub fn get_ready_participants(&self, room_id: &str) -> Vec<Participant> {
        self.rooms.get(room_id)
            .map(|room| {
                room.participants.values()
                    .filter(|p| p.state == PlayerState::Ready || p.state == PlayerState::InGame)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Obtiene todos los tokens visibles
    pub fn get_visible_tokens(&self, room_id: &str) -> Vec<Token> {
        self.rooms.get(room_id)
            .map(|room| {
                room.participants.values()
                    .filter_map(|p| p.token.clone())
                    .filter(|t| t.visible_to_all)
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Limpia salas viejas (llamar periódicamente)
    pub fn cleanup_stale_rooms(&self, max_age_hours: i64) {
        let cutoff = Utc::now() - chrono::Duration::hours(max_age_hours);
        
        let stale_rooms: Vec<RoomId> = self.rooms.iter()
            .filter(|entry| {
                entry.value().phase == GamePhase::Ended ||
                entry.value().participants.is_empty() ||
                entry.value().created_at < cutoff
            })
            .map(|entry| entry.key().clone())
            .collect();
        
        for room_id in stale_rooms {
            self.rooms.remove(&room_id);
            tracing::info!("Cleaned up stale room: {}", room_id);
        }
    }
}
