use actix_web::web;
use super::handlers::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Pagina Principal (Consola)
    cfg.route("/", web::get().to(index));

    // Vistas HTML Renderizadas (Deshabilitadas en modo API-Only)
    /*
    cfg.service(
        web::scope("/ver")
            .route("/personaje", web::get().to(ver_personaje_html))
            .route("/constelacion", web::get().to(ver_constelacion_html))
            .route("/juego", web::get().to(ver_juego_html))
            .route("/muro", web::get().to(ver_muro_html))
    );
    */
    
    // API JSON
    cfg.service(
        web::scope("/api/v1")
            .route("/personaje", web::get().to(generar_personaje_json))
            .route("/personaje", web::post().to(generar_personaje_custom_json))
            .route("/descargar/personaje/json", web::get().to(descargar_personaje_json))
            .route("/descargar/constelacion/json", web::get().to(descargar_constelacion_json))
            .route("/generate-world", web::post().to(generate_world_map))
            .route("/loot", web::get().to(generate_loot_json))
            .route("/forge", web::post().to(forge_item_json))
            .route("/character/{id}", web::get().to(cargar_personaje))
    );

    // Registro de Almas Routes
    super::souls::config(cfg);
}

