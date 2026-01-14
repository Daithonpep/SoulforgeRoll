use actix_web::{web, HttpResponse, Responder};
use actix_files::NamedFile;
use tera::{Tera, Context};
use serde_json::json;
use serde::Deserialize;
use std::collections::HashMap;
use crate::SoulForge;
use crate::core::{ParametrosGeneracion, ParametrosConstelacion, Genero, Language};
use crate::core::i18n::TextosUI;
use rand::Rng;


// --- PAGINA DE INICIO ---

// --- PAGINA DE INICIO (API ROOT) ---

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "service": "SoulForge API",
        "version": "2.1",
        "endpoints": {
            "generate_character": "/api/v1/personaje",
            "generate_world": "/api/v1/generate-world"
        },
        "status": "online"
    }))
}


// --- API JSON ---

fn guardar_personaje(alma: &crate::core::Alma) {
    use std::fs;
    let dir = "saved_characters";
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("Error creating directory: {}", e);
        return;
    }
    
    let path = format!("{}/{}.json", dir, alma.id);
    if let Ok(json_str) = serde_json::to_string_pretty(alma) {
        if let Err(e) = fs::write(path, json_str) {
            eprintln!("Error writing file: {}", e);
        }
    }
}

pub async fn generar_personaje_json() -> impl Responder {
    let mut forge = SoulForge::nuevo();
    let personaje = forge.forjar(ParametrosGeneracion::default());
    guardar_personaje(&personaje);
    
    let sanitized_name = personaje.identidad.nombre.replace(" ", "_");
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}.json\"", sanitized_name)))
        .json(personaje)
}

pub async fn generar_personaje_custom_json(params: web::Json<ParametrosGeneracion>) -> impl Responder {
    let mut forge = SoulForge::nuevo();
    
    // Configurar semilla si viene
    if let Some(s) = params.semilla {
        forge = SoulForge::con_semilla(s);
    }
    
    let personaje = forge.forjar(params.into_inner());
    guardar_personaje(&personaje);

    let sanitized_name = personaje.identidad.nombre.replace(" ", "_");
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}.json\"", sanitized_name)))
        .json(personaje)
}

// Handler para cargar personaje desde servidor (Anti-Cheat)
pub async fn cargar_personaje(path: web::Path<String>) -> impl Responder {
    use std::fs;
    let id = path.into_inner();
    let file_path = format!("saved_characters/{}.json", id);
    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                HttpResponse::Ok().json(json)
            } else {
                HttpResponse::InternalServerError().body("Corrupted File")
            }
        },
        Err(_) => HttpResponse::NotFound().body("Character not found on server")
    }
}

// --- VISTAS HTML (Renderizado) ---

// --- VISTAS HTML (Legacy/Deprecated for Backend-Only) ---
// Estas funciones se eliminan para que el backend sea puro JSON.
// El frontend alojado en Vercel se encargará de mostrar la UI.




pub async fn descargar_personaje_json(
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
     // Si viene ID, intentar cargar desde disco
    let mut personaje: crate::core::Alma;
    
    if let Some(id) = query.get("id") {
        match std::fs::read_to_string(format!("saved_characters/{}.json", id)) {
            Ok(content) => {
                personaje = serde_json::from_str(&content).unwrap_or_else(|_| {
                     let mut f = SoulForge::nuevo();
                     f.forjar(ParametrosGeneracion::default())
                });
            },
            Err(_) => {
                 let params = parsear_params_generacion(&query);
                 let mut forge = SoulForge::nuevo();
                 personaje = forge.forjar(params);
            }
        }
    } else {
         let params = parsear_params_generacion(&query);
         let mut forge = SoulForge::nuevo();
         personaje = forge.forjar(params);
         guardar_personaje(&personaje);
    }
    
    let sanitized_name = personaje.identidad.nombre.replace(" ", "_");
    HttpResponse::Ok()
        .insert_header(("Content-Disposition", format!("attachment; filename=\"{}.json\"", sanitized_name)))
        .json(personaje)
}






pub async fn descargar_constelacion_json(
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    let params = parsear_params_constelacion(&query);
    
    let semilla_maestra = if let Some(s_str) = query.get("semilla") {
        s_str.parse::<u64>().unwrap_or_else(|_| rand::random())
    } else {
        rand::random()
    };

    let mut forge = SoulForge::con_semilla(semilla_maestra);
    let constelacion = forge.forjar_constelacion(params);
    let filename = format!("{}_{}.json", constelacion.nombre.replace(" ", "_"), constelacion.semilla);

    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .json(constelacion)
}

#[allow(dead_code)]
fn renderizar_markdown(c: &crate::constelacion::Constelacion) -> String {
    let mut md = String::new();
    
    // HEADER
    md.push_str(&format!("# {} \n", c.nombre));
    md.push_str(&format!("*Una constelación en el mundo: {:?}*\n\n", c.mundo));
    md.push_str(&format!("**ID Constelación:** `{}`  \n", c.id));
    md.push_str(&format!("**Semilla:** `{}`\n\n", c.semilla));
    
    md.push_str("---\n\n");
    
    // EVENTO ANCLA
    md.push_str(&format!("## 🛡️ Evento Ancla: {}\n", c.evento_ancla.nombre));
    md.push_str(&format!("**Fecha:** {}\n\n", c.evento_ancla.anio));
    md.push_str(&format!("> {}\n\n", c.evento_ancla.descripcion));
    md.push_str(&format!("*Impacto Global:* {}\n\n", c.evento_ancla.impacto_global));
    
    md.push_str("---\n\n");
    
    // HISTORIA CONJUNTA
    md.push_str("## 📜 Historia Conjunta\n\n");
    md.push_str(&format!("{}\n\n", c.historia_conjunta));
    
    md.push_str("---\n\n");
    
    // DRAMATIS PERSONAE - PERFILES COMPLETOS
    md.push_str("## 👥 DRAMATIS PERSONAE\n\n");
    
    for (idx, alma) in c.almas.iter().enumerate() {
        md.push_str(&format!("### {}. {} {}\n\n", idx + 1, alma.identidad.nombre, alma.identidad.apellido.clone().unwrap_or_default()));
        md.push_str(&format!("**Rol Narrativo:** {:?}  \n", alma.rol));
        md.push_str(&format!("**Género:** {:?}  \n", alma.identidad.genero));
        md.push_str(&format!("**Edad:** {} años\n\n", alma.identidad.edad));
        
        // CAPA 1: ARQUETIPO
        md.push_str("#### 🎭 Arquetipo Junguiano\n");
        md.push_str(&format!("- **Tipo:** {:?}\n", alma.capas.arquetipo.tipo));
        md.push_str(&format!("- **Manifestación Luz:** {}\n", alma.capas.arquetipo.manifestacion_luz));
        md.push_str(&format!("- **Manifestación Sombra:** {}\n", alma.capas.arquetipo.manifestacion_sombra));
        md.push_str(&format!("- **Don Natural:** {}\n", alma.capas.arquetipo.don_natural));
        md.push_str(&format!("- **Debilidad:** {}\n\n", alma.capas.arquetipo.debilidad));
        
        // CAPA 2: HERIDA
        md.push_str("#### 💔 La Herida (The Ghost)\n");
        md.push_str(&format!("- **Tipo:** {:?}\n", alma.capas.herida.tipo));
        md.push_str(&format!("- **Edad cuando ocurrió:** {:?}\n", alma.capas.herida.edad_cuando_ocurrio));
        md.push_str(&format!("- **Causante:** {}\n", alma.capas.herida.causante));
        md.push_str(&format!("- **Circunstancia:** {}\n", alma.capas.herida.circunstancia));
        md.push_str(&format!("- **Cómo lo cambió:** {}\n", alma.capas.herida.como_lo_cambio));
        md.push_str(&format!("- **Gatillo emocional:** {}\n", alma.capas.herida.gatillo_emocional));
        md.push_str(&format!("- **Mecanismo de defensa:** {}\n\n", alma.capas.herida.mecanismo_defensa));
        
        // CAPA 3: MÁSCARA
        md.push_str("#### 🎪 La Máscara\n");
        md.push_str(&format!("- **Comportamiento público:** {}\n", alma.capas.mascara.comportamiento_publico));
        md.push_str(&format!("- **Imagen proyectada:** {}\n", alma.capas.mascara.imagen_proyectada));
        md.push_str(&format!("- **Frase típica:** *\"{}\"*\n", alma.capas.mascara.frase_tipica));
        md.push_str(&format!("- **Sentimiento oculto:** {}\n", alma.capas.mascara.sentimiento_oculto));
        md.push_str(&format!("- **Miedo central:** {}\n", alma.capas.mascara.miedo_central));
        md.push_str(&format!("- **Deseo secreto:** {}\n", alma.capas.mascara.deseo_secreto));
        md.push_str(&format!("- **Lo que rompe la máscara:** {}\n", alma.capas.mascara.trigger_que_la_rompe));
        md.push_str(&format!("- **Costo de mantenerla:** {}\n\n", alma.capas.mascara.costo_de_mantenerla));
        
        // CAPA 4: DESEO VS NECESIDAD
        md.push_str("#### ⚖️ Deseo vs Necesidad\n");
        md.push_str(&format!("- **Deseo consciente:** {}\n", alma.capas.deseo_necesidad.deseo_consciente));
        md.push_str(&format!("- **Motivación del deseo:** {}\n", alma.capas.deseo_necesidad.motivacion_del_deseo));
        md.push_str(&format!("- **Estrategia:** {}\n", alma.capas.deseo_necesidad.estrategia));
        md.push_str(&format!("- **Necesidad real:** {}\n", alma.capas.deseo_necesidad.necesidad_real));
        md.push_str(&format!("- **Por qué no la ve:** {}\n", alma.capas.deseo_necesidad.por_que_no_la_ve));
        md.push_str(&format!("- **Conflicto interno:** {}\n", alma.capas.deseo_necesidad.conflicto));
        md.push_str(&format!("- **Ironía narrativa:** {}\n\n", alma.capas.deseo_necesidad.ironia));
        
        // CAPA 5: SOMBRA
        md.push_str("#### 🌑 La Sombra\n");
        md.push_str(&format!("- **Rasgo negado:** {}\n", alma.capas.sombra.rasgo_negado));
        md.push_str("- **Cómo se filtra:** ");
        for filtro in &alma.capas.sombra.como_se_filtra {
            md.push_str(&format!("{}, ", filtro));
        }
        md.push_str("\n");
        md.push_str(&format!("- **Qué la despierta:** {}\n", alma.capas.sombra.que_la_despierta));
        md.push_str(&format!("- **Potencial si integra:** {}\n", alma.capas.sombra.potencial_integrado));
        md.push_str(&format!("- **Peligro si domina:** {}\n\n", alma.capas.sombra.peligro_si_domina));
        
        // CAPA 6: VÍNCULOS
        md.push_str("#### 🔗 Patrón Vincular\n");
        md.push_str(&format!("- **Patrón:** {:?}\n", alma.capas.vinculos.patron));
        md.push_str(&format!("- **Estilo de apego:** {:?}\n", alma.capas.vinculos.estilo_apego));
        md.push_str(&format!("- **Rol en grupos:** {}\n", alma.capas.vinculos.rol_en_grupos));
        md.push_str(&format!("- **Cómo expresa afecto:** {}\n", alma.capas.vinculos.como_expresa_afecto));
        md.push_str(&format!("- **Qué busca en otros:** {}\n", alma.capas.vinculos.que_busca_en_otros));
        md.push_str(&format!("- **Qué ofrece:** {}\n\n", alma.capas.vinculos.que_ofrece));
        
        // CAPA 7: LA MENTIRA
        md.push_str("#### 🎭 La Mentira que Cree\n");
        md.push_str(&format!("- **La mentira:** *\"{}\"*\n", alma.capas.mentira.la_mentira));
        md.push_str(&format!("- **Cómo nació:** {}\n", alma.capas.mentira.como_nacio));
        md.push_str(&format!("- **Cómo distorsiona:** {}\n", alma.capas.mentira.como_distorsiona));
        md.push_str(&format!("- **Decisiones que causa:** {}\n", alma.capas.mentira.decisiones_que_causa));
        md.push_str(&format!("- **Verdad necesaria:** {}\n", alma.capas.mentira.verdad_necesaria));
        md.push_str(&format!("- **Catalizador potencial:** {}\n", alma.capas.mentira.catalizador_potencial));
        md.push_str(&format!("- **Costo de la verdad:** {}\n\n", alma.capas.mentira.costo_de_la_verdad));
        
        // BIOGRAFÍA COMPLETA
        md.push_str("#### 📖 Biografía Completa\n\n");
        for fase in &alma.biografia.fases {
            md.push_str(&format!("**{}** ({:?})\n", fase.titulo, fase.tonalidad));
            md.push_str(&format!("{}\n\n", fase.contenido));
        }
        
        // CONFLICTOS INTERNOS
        if !alma.biografia.conflictos.is_empty() {
            md.push_str("#### ⚔️ Conflictos Internos\n");
            for conflicto in &alma.biografia.conflictos {
                md.push_str(&format!("**{}:** {} ↔ {}\n", conflicto.nombre, conflicto.polo_a, conflicto.polo_b));
                md.push_str(&format!("_{}_\n\n", conflicto.descripcion));
            }
        }
        
        // MOMENTOS DE GRACIA
        if !alma.biografia.momentos_gracia.is_empty() {
            md.push_str("#### ✨ Momentos de Gracia\n");
            for gracia in &alma.biografia.momentos_gracia {
                md.push_str(&format!("**{}:** {}\n", gracia.nombre, gracia.regalo));
            }
            md.push_str("\n");
        }
        
        md.push_str("---\n\n");
    }
    
    // TENSIONES CENTRALES
    if !c.tensiones_centrales.is_empty() {
        md.push_str("## 🔥 Tensiones Centrales\n\n");
        for tension in &c.tensiones_centrales {
            md.push_str(&format!("### {:?}\n", tension.tipo));
            md.push_str(&format!("{}\n\n", tension.descripcion));
            md.push_str(&format!("**Cómo podría estallar:** {}\n", tension.como_podria_estallar));
            md.push_str(&format!("**Lo que está en juego:** {}\n\n", tension.stakes));
        }
        md.push_str("---\n\n");
    }
    
    // RED DE RELACIONES
    md.push_str("## 🌐 Red de Relaciones Profundas\n\n");
    for con in &c.conexiones_profundas {
        md.push_str(&format!("### {} ↔ {}\n", con.nombre_a, con.nombre_b));
        md.push_str(&format!("**Tipo de conexión:** {:?}\n\n", con.tipo));
        
        md.push_str("**Momento Origen:**\n");
        md.push_str(&format!("> {}\n\n", con.momento_origen.descripcion));
        md.push_str(&format!("- **Perspectiva de {}:** {}\n", con.nombre_a, con.momento_origen.perspectiva_a));
        md.push_str(&format!("- **Perspectiva de {}:** {}\n", con.nombre_b, con.momento_origen.perspectiva_b));
        md.push_str(&format!("- **Impacto en {}:** {}\n", con.nombre_a, con.momento_origen.impacto_a));
        md.push_str(&format!("- **Impacto en {}:** {}\n\n", con.nombre_b, con.momento_origen.impacto_b));
        
        // Momentos clave
        if !con.momentos_clave.is_empty() {
            md.push_str("**Momentos Clave:**\n");
            for momento in &con.momentos_clave {
                md.push_str(&format!("- {}\n", momento.descripcion));
            }
            md.push_str("\n");
        }
        
        md.push_str(&format!("**Estado actual:** {}\n", con.estado_actual));
        md.push_str(&format!("**Tensión actual:** {}\n", con.tension_actual));
        md.push_str(&format!("**Salud de la relación:** {:.0}%  \n", con.salud * 100.0));
        md.push_str(&format!("**Profundidad:** {:.0}%\n\n", con.profundidad * 100.0));
        
        md.push_str("---\n\n");
    }
    
    // HISTORIAS ENTRELAZADAS
    if !c.historias_pares.is_empty() {
        md.push_str("## 💫 Historias Entrelazadas\n\n");
        for par in &c.historias_pares {
            md.push_str(&format!("### {} y {}\n\n", par.nombre_a, par.nombre_b));
            md.push_str(&format!("**Cómo se conocieron:** {}\n\n", par.historia.narrativa_encuentro));
            
            if !par.historia.momentos_compartidos.is_empty() {
                md.push_str("**Momentos Cruciales:**\n");
                for momento in &par.historia.momentos_compartidos {
                    md.push_str(&format!("- {}: {}\n", momento.descripcion, momento.impacto_a));
                }
                md.push_str("\n");
            }
            
            md.push_str(&format!("**Dinámica actual:** {}\n\n", par.historia.dinamica_actual));
        }
    }
    
    // FOOTER
    md.push_str("---\n\n");
    md.push_str("*Generado por SoulForge v2.4 - Sistema de Narrativa Procedural*\n");
    
    md
}

// --- Helpers de Parseo (De Strings web a Enums Rust) ---

fn parsear_params_generacion(query: &HashMap<String, String>) -> ParametrosGeneracion {
    let mut p = ParametrosGeneracion::default();
    
    // Semilla
    if let Some(s) = query.get("semilla").filter(|s| !s.is_empty()) {
        if let Ok(val) = s.parse::<u64>() {
            p.semilla = Some(val);
        }
    }

    // Rol
    if let Some(s) = query.get("rol").filter(|s| !s.is_empty()) {
        // Mapeo simple basado en nombres del HTML
        p.rol = serde_json::from_value(json!(s)).ok();
    }
    
    // Tono Moral
    if let Some(s) = query.get("tono_moral").filter(|s| !s.is_empty()) {
        p.tono_moral = serde_json::from_value(json!(s)).ok();
    }
    
    // Mundo
    if let Some(s) = query.get("mundo").filter(|s| !s.is_empty()) {
        p.mundo = serde_json::from_value(json!(s)).ok();
    }

    // Idioma
    if let Some(s) = query.get("lang").filter(|s| !s.is_empty()) {
        p.idioma = Some(Language::from_str(s));
    }

    p
}

fn parsear_params_constelacion(query: &HashMap<String, String>) -> ParametrosConstelacion {
    let mut p = ParametrosConstelacion::default();
    
    if let Some(s) = query.get("cantidad").filter(|s| !s.is_empty()) {
        if let Ok(val) = s.parse::<usize>() {
            p.cantidad = val;
        }
    }
    
    if let Some(s) = query.get("mundo").filter(|s| !s.is_empty()) {
        if let Ok(val) = serde_json::from_value(json!(s)) {
            p.mundo = val;
        }
    }
    
    if let Some(s) = query.get("estructura").filter(|s| !s.is_empty()) {
        if let Ok(val) = serde_json::from_value(json!(s)) {
            p.estructura = val;
        }
    }
    
    if let Some(s) = query.get("densidad_relaciones").filter(|s| !s.is_empty()) {
        if let Ok(val) = serde_json::from_value(json!(s)) {
            p.densidad_relaciones = val;
        }
    }
    
    // Checkboxes envían "true" o no envían nada
    p.incluir_antagonista = query.get("incluir_antagonista").map(|s| s == "true").unwrap_or(false);
    p.incluir_romance = query.get("incluir_romance").map(|s| s == "true").unwrap_or(false);
    
    // Nombres fijos separados por coma
    if let Some(s) = query.get("nombres").filter(|s| !s.is_empty()) {
        let lista: Vec<String> = s.split(',').map(|n| n.trim().to_string()).filter(|n| !n.is_empty()).collect();
        if !lista.is_empty() {
            p.nombres_fijos = Some(lista);
        }
    }
    
    // Géneros fijos separados por coma (M, F o vacío)
    if let Some(s) = query.get("generos").filter(|s| !s.is_empty()) {
        let lista: Vec<Genero> = s.split(',').map(|g| {
            match g.trim() {
                "M" | "Masculino" => Genero::Masculino,
                "F" | "Femenino" => Genero::Femenino,
                _ => Genero::Masculino, // Default
            }
        }).collect();
        // Solo guardar si al menos uno tiene valor significativo
        p.generos_fijos = Some(lista);
    }
    
    // Edades fijas separadas por coma
    if let Some(s) = query.get("edades").filter(|s| !s.is_empty()) {
        let lista: Vec<u32> = s.split(',').map(|edad_str| {
            edad_str.trim().parse::<u32>().unwrap_or_else(|_| rand::thread_rng().gen_range(18..55))
        }).collect();
        p.edades_fijas = Some(lista);
    }
    
    // Parámetros avanzados
    if let Some(s) = query.get("tipo_villano").filter(|s| !s.is_empty() && *s != "None") {
        p.tipo_villano = Some(s.to_string());
        p.incluir_antagonista = true; // Forzar antagonista si hay tipo
    }
    
    if let Some(s) = query.get("modo_romance").filter(|s| !s.is_empty() && *s != "None") {
        p.modo_romance = Some(s.to_string());
        p.incluir_romance = true; // Forzar romance si hay modo
    }
    
    p
}

// --- VISTAS NUEVAS (Game & Wall) ---



// --- MAP GENERATION ---
use crate::cartographer::{parser::BackstoryParser, world_generator::{WorldGenerator, WorldConfig}};

#[derive(Deserialize)]
pub struct WorldGenRequest {
    pub characters: Vec<serde_json::Value>,
    pub config: Option<WorldConfigDTO>,
}

#[derive(Deserialize)]
pub struct WorldConfigDTO {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub seed: Option<u64>,
}

pub async fn generate_world_map(
    req: web::Json<WorldGenRequest>,
) -> impl Responder {
    let parser = BackstoryParser::new();
    
    // Parsear todos los personajes
    let geographies: Vec<_> = req.characters.iter()
        .map(|c| parser.parse_character(c))
        .collect();
        
    // Configuración
    let mut config = WorldConfig::default();
    if let Some(c) = &req.config {
        if let Some(w) = c.width { config.width = w; }
        if let Some(h) = c.height { config.height = h; }
        if let Some(s) = c.seed { config.seed = s; }
    }
    
    // Generar mundo
    let mut generator = WorldGenerator::new(config);
    let world = generator.generate_from_characters(geographies);
    
    HttpResponse::Ok().json(world)
}

// --- LOOT & FORGE ---
use crate::core::items::{ItemGenerator, ItemType};

pub async fn generate_loot_json() -> impl Responder {
    let item = ItemGenerator::generate_loot(None);
    HttpResponse::Ok().json(item)
}

#[derive(Deserialize)]
pub struct ForgeRequest {
    pub item_type: String, // "weapon", "armor", etc.
    pub quality: u8,
    pub skill: u8,
}

pub async fn forge_item_json(req: web::Json<ForgeRequest>) -> impl Responder {
    let i_type = match req.item_type.to_lowercase().as_str() {
        "weapon" => ItemType::Weapon,
        "armor" => ItemType::Armor,
        "trinket" => ItemType::Trinket,
        "potion" => ItemType::Potion,
        _ => ItemType::Material,
    };
    
    let item = ItemGenerator::forge(i_type, req.quality, req.skill);
    HttpResponse::Ok().json(item)
}
