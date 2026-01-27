mod room;
mod protocol;
mod handlers;

use std::env;
use std::sync::Arc;
use std::collections::HashMap;
use warp::Filter;
use tracing_subscriber;
use serde::{Deserialize, Serialize};

use crate::room::RoomManager;
use crate::handlers::{ws_handler, create_room_handler, room_info_handler};

// Importar librería de generación
use soulforge_server::{SoulForge, ParametrosGeneracion, ParametrosConstelacion, Mundo, Rol, TonoMoral, Language};

#[derive(Debug, Deserialize)]
struct CharacterQuery {
    nombre: Option<String>,
    genero: Option<String>,
    rol: Option<String>,
    mundo: Option<String>,
    tono: Option<String>,
    edad: Option<u32>,
    lang: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ConstellationQuery {
    num_personajes: Option<usize>,
    mundo: Option<String>,
    tono: Option<String>,
    lang: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AriaRequest {
    messages: Vec<serde_json::Value>,
    system_prompt: Option<String>,
}

fn parse_mundo(s: &str) -> Mundo {
    match s.to_lowercase().as_str() {
        "fantasiamedieval" | "fantasia_medieval" | "fantasia medieval" => Mundo::FantasiaMedieval,
        "fantasiaoscura" | "fantasia_oscura" | "fantasia oscura" => Mundo::FantasiaOscura,
        "fantasiaurbana" | "fantasia_urbana" | "fantasia urbana" => Mundo::FantasiaUrbana,
        "scifispace" | "scifi" | "ciencia_ficcion" | "scifi space" => Mundo::SciFiSpace,
        "scificyberpunk" | "cyberpunk" => Mundo::SciFiCyberpunk,
        "scifipostapocaliptico" | "postapocaliptico" => Mundo::SciFiPostApocaliptico,
        "japonfeudal" | "japon_feudal" | "japon feudal" => Mundo::JaponFeudal,
        "animefantasia" | "anime fantasia" => Mundo::AnimeFantasia,
        "chinaimperial" | "china" => Mundo::ChinaImperial,
        "wuxia" => Mundo::Wuxia,
        "coreahistorica" | "corea" => Mundo::CoreaHistorica,
        "anime" => Mundo::Anime,
        "mitologiaasiatica" | "asia" => Mundo::MitologiaAsiatica,
        "mitologiagriega" | "grecia" | "mitologia griega" => Mundo::MitologiaGriega,
        "mitologianordica" | "nordica" | "mitologia nordica" => Mundo::MitologiaNordica,
        "steampunk" => Mundo::Steampunk,
        "western" => Mundo::Western,
        "noir" => Mundo::Noir,
        "piratascaribe" | "piratas" => Mundo::PiratasCaribe,
        "victoriano" => Mundo::Victoriano,
        "realista" => Mundo::Realista,
        _ => Mundo::FantasiaMedieval,
    }
}

fn parse_tono(s: &str) -> TonoMoral {
    match s.to_lowercase().as_str() {
        "luminoso" | "luminoso/bueno" => TonoMoral::Luminoso,
        "claro" => TonoMoral::Claro,
        "gris" | "neutral" => TonoMoral::Gris,
        "oscuro" => TonoMoral::Oscuro,
        "abismal" => TonoMoral::Abismal,
        _ => TonoMoral::Gris,
    }
}

fn parse_rol(s: &str) -> Rol {
    match s.to_lowercase().as_str() {
        "heroe" => Rol::Heroe,
        "villano" => Rol::Villano,
        "mentor" => Rol::Mentor,
        "aliado" => Rol::Aliado,
        "antagonista" => Rol::Villano,
        "jugador" | "jugador (d&d/rpg)" => Rol::Jugador,
        _ => Rol::Heroe,
    }
}

fn parse_genero(s: &str) -> Option<soulforge_server::Genero> {
    match s.to_lowercase().as_str() {
        "masculino" | "male" | "m" => Some(soulforge_server::Genero::Masculino),
        "femenino" | "female" | "f" => Some(soulforge_server::Genero::Femenino),
        _ => None, // random
    }
}

async fn generate_character_handler(query: CharacterQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let mut forge = SoulForge::nuevo();
    
    let params = ParametrosGeneracion {
        nombre_fijo: query.nombre,
        mundo: query.mundo.as_deref().map(parse_mundo),
        rol: query.rol.as_deref().map(parse_rol),
        tono_moral: query.tono.as_deref().map(parse_tono),
        edad_fija: query.edad,
        genero: query.genero.as_deref().and_then(parse_genero),
        idioma: query.lang.as_deref().map(Language::from_str),
        ..Default::default()
    };
    
    let alma = forge.forjar(params);

    // Convertir a serde_json::Value para pasar al enriquecedor
    let json_val = serde_json::to_value(&alma).unwrap_or_default();

    // Enriquecer con IA (si está disponible)
    use soulforge_server::core::ia_integration::enriquecer_personaje;
    let enhanced_json = enriquecer_personaje(json_val);
    
    Ok(warp::reply::json(&enhanced_json))
}

async fn generate_constellation_handler(query: ConstellationQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let mut forge = SoulForge::nuevo();
    
    let params = ParametrosConstelacion {
        cantidad: query.num_personajes.unwrap_or(4),
        mundo: query.mundo.as_deref().map(parse_mundo).unwrap_or(Mundo::FantasiaMedieval),
        ..Default::default()
    };
    
    let constelacion = forge.forjar_constelacion(params);
    
    Ok(warp::reply::json(&constelacion))
}

async fn aria_chat_handler(req: AriaRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[BACKEND] Recibida solicitud para Aria chat");
    use soulforge_server::core::ia_integration::chat_con_aria;
    let reply = chat_con_aria(req.messages, req.system_prompt)
        .unwrap_or_else(|| "Error al procesar la solicitud con Aria.".to_string());
    
    Ok(warp::reply::json(&serde_json::json!({ "reply": reply })))
}

async fn aria_status_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&serde_json::json!({ "status": "Aria is online and ready for chat (POST)" })))
}

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
    
    // === GENERACIÓN DE PERSONAJES ===
    
    // GET /api/v1/personaje?nombre=...&mundo=...
    let personaje_route = warp::path!("api" / "v1" / "personaje")
        .and(warp::get())
        .and(warp::query::<CharacterQuery>())
        .and_then(generate_character_handler);
    
    // GET /api/v1/constelacion?num_personajes=...&mundo=...
    let constelacion_route = warp::path!("api" / "v1" / "constelacion")
        .and(warp::get())
        .and(warp::query::<ConstellationQuery>())
        .and_then(generate_constellation_handler);

    // POST /api/chat & /api/v1/aria/chat (Compatibilidad)
    let aria_chat_route = warp::path("api")
        .and(
            warp::path("chat")
            .or(warp::path("v1").and(warp::path("aria")).and(warp::path("chat")))
        )
        .and(warp::post())
        .and(warp::body::json())
        .and_then(aria_chat_handler);

    // GET /api/chat (Diagnóstico)
    let aria_diag_route = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::get())
        .and_then(aria_status_handler);
    
    // Health check
    let health = warp::path!("health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "alive"})));

    let routes = aria_chat_route
        .or(aria_diag_route)
        .or(personaje_route)
        .or(constelacion_route)
        .or(ws_route)
        .or(create_route)
        .or(info_route)
        .or(health)
        .with(cors);
    
    tracing::info!("SoulForge Server starting on port {}", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
