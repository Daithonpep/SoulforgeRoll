// src/api/handlers.rs
// Adding new view handlers for Game and Wall of Fallen
use actix_files::NamedFile;
use std::path::PathBuf;

// ... existing imports ... (Assuming they are available in scope or adding necessary ones)

pub async fn ver_juego_html() -> impl Responder {
    // Assuming templates are in "templates/" relative to binary
    let path: PathBuf = "./templates/juego_en_linea.html".parse().unwrap();
    NamedFile::open(path)
}

pub async fn ver_muro_html() -> impl Responder {
    let path: PathBuf = "./templates/muro_caidos.html".parse().unwrap();
    NamedFile::open(path)
}
