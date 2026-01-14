use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use tera::Tera;
use soulforge::api;

use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurar logging visual
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    imprimir_banner();

    // Cargar Templates HTML (REMOVIDO PARA API-ONLY)
    // println!("🎨 Cargando templates desde ../frontend/...");
    // let tera = match Tera::new("../frontend/**/*.html") ...

    // Puerto desde variable de entorno (Railway) o 8080 por defecto
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT debe ser un número válido");

    println!("\n🚀 SoulForge API v2.1 escuchando en http://0.0.0.0:{}", port);
    println!("   > Modo:          API JSON Only (Backend)");
    println!("   > Endpoint API:  /api/v1/personaje");
    println!("   > Presiona Ctrl+C para detener.\n");

    HttpServer::new(move || {
        // En producción, configura esto con la URL real de Vercel/Netlify
        let cors = Cors::permissive(); 

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default()) // Logger de requests
            // .app_data(web::Data::new(tera.clone())) // No se necesita Tera
            // .service(fs::Files::new("/static", "../frontend/static").show_files_listing()) // No se sirven estáticos
            .configure(api::routes::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn imprimir_banner() {
    println!(r#"
    ╔═══════════════════════════════════════════════════════════════════════╗
    ║                                                                       ║
    ║   ███████╗ ██████╗ ██╗   ██╗██╗     ███████╗ ██████╗ ██████╗  ██████╗ ║
    ║   ██╔════╝██╔═══██╗██║   ██║██║     ██╔════╝██╔═══██╗██╔══██╗██╔════╝ ║
    ║   ███████╗██║   ██║██║   ██║██║     █████╗  ██║   ██║██████╔╝██║  ███╗║
    ║   ╚════██║██║   ██║██║   ██║██║     ██╔══╝  ██║   ██║██╔══██╗██║   ██║║
    ║   ███████║╚██████╔╝╚██████╔╝███████╗██║     ╚██████╔╝██║  ██║╚██████╔╝║
    ║   ╚══════╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ║
    ║                                                                       ║
    ║                            SERVER v2.1                                ║
    ║         Generador de Personajes con Constelaciones de Almas           ║
    ║                                                                       ║
    ╚═══════════════════════════════════════════════════════════════════════╝
    "#);
}
