//! Núcleo del sistema de generación de personajes

mod alma;
mod capas;
mod identidad;
mod arco;
mod biografia;
mod conexiones;
pub mod gramatica;
pub mod i18n;
pub mod adapter;
pub mod procedural_text;
pub mod narrativa;
pub mod promo;
pub mod items;

pub use alma::*;
pub use capas::*;
pub use identidad::*;
pub use arco::*;
pub use biografia::*;
pub use conexiones::*;
pub use adapter::*;


use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};


/// Rol narrativo del personaje
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rol {
    Heroe,
    Villano,
    Mentor,
    Aliado,
    Catalizador,
    Sombra,
    Guardian,
    Embaucador,
    Mercenario,
    Lider,
    Marginado,
    Rebelde,
    Bufon,
    Victima,
    Profeta,
    Jugador, // For D&D Player Characters
}

impl Rol {
    pub fn all() -> Vec<Rol> {
        vec![
            Rol::Heroe, Rol::Villano, Rol::Mentor, Rol::Aliado,
            Rol::Catalizador, Rol::Sombra, Rol::Guardian, Rol::Embaucador,
            Rol::Mercenario, Rol::Lider, Rol::Marginado, Rol::Rebelde,
            Rol::Bufon, Rol::Victima, Rol::Profeta, Rol::Jugador,
        ]
    }
}
// ... (TonoMoral and Mundo enums remain unchanged)

// Add D&D Specific Enums and Structs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Raza {
    Humano,
    Elfo,
    Enano,
    Halfling,
    Dragonborn,
    Gnomo,
    Tiefling,
    Orco,
}

impl Raza {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Humano" => Some(Raza::Humano),
            "Elfo" => Some(Raza::Elfo),
            "Enano" => Some(Raza::Enano),
            "Halfling" => Some(Raza::Halfling),
            "Dragonborn" => Some(Raza::Dragonborn),
            "Gnomo" => Some(Raza::Gnomo),
            "Tiefling" => Some(Raza::Tiefling),
            "Orco" => Some(Raza::Orco),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DndStats {
    pub fuerza: u8,
    pub destreza: u8,
    pub constitucion: u8,
    pub inteligencia: u8,
    pub sabiduria: u8,
    pub carisma: u8,
    pub raza: Raza,
    pub clase: String, // Derived from Rol or random
    pub nivel: u8,
    pub hp: u16,
    pub ac: u8,
}

// ... (Existing structs) ...

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParametrosGeneracion {
    pub semilla: Option<u64>,
    pub rol: Option<Rol>,
    pub tono_moral: Option<TonoMoral>,
    pub mundo: Option<Mundo>,
    pub nivel_conflicto: Option<NivelConflicto>,
    pub genero: Option<Genero>,
    pub profundidad: Option<Profundidad>,
    pub nombre_fijo: Option<String>,
    pub edad_fija: Option<u32>,
    pub idioma: Option<Language>,
    pub raza: Option<Raza>, // New field for D&D race
}

/// Tono moral - espectro no binario
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TonoMoral {
    Luminoso,
    Claro,
    Gris,
    Oscuro,
    Abismal,
}

impl TonoMoral {
    pub fn all() -> Vec<TonoMoral> {
        vec![TonoMoral::Luminoso, TonoMoral::Claro, TonoMoral::Gris, TonoMoral::Oscuro, TonoMoral::Abismal]
    }
    
    pub fn valor_numerico(&self) -> i32 {
        match self {
            TonoMoral::Luminoso => 2,
            TonoMoral::Claro => 1,
            TonoMoral::Gris => 0,
            TonoMoral::Oscuro => -1,
            TonoMoral::Abismal => -2,
        }
    }
}

/// Mundo/Género de la historia
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mundo {
    // Fantasía
    FantasiaMedieval,
    FantasiaOscura,
    FantasiaUrbana,
    // Ciencia Ficción
    SciFiSpace,
    SciFiCyberpunk,
    SciFiPostApocaliptico,
    // Histórico Occidental
    Realista,
    HistoricoMedieval,
    HistoricoAntiguo,
    HistoricoModerno,
    Victoriano,
    // Asia - Japón
    JaponFeudal,
    AnimeFantasia,
    // Asia - China
    ChinaImperial,
    Wuxia,
    // Asia - Corea
    CoreaHistorica,
    // Asia - General
    MitologiaAsiatica,
    // Géneros Occidentales
    Anime,
    Mitologico,
    MitologiaGriega,
    MitologiaNordica,
    Steampunk,
    Western,
    Noir,
    PiratasCaribe,
}

impl Mundo {
    pub fn all() -> Vec<Mundo> {
        vec![
            Mundo::FantasiaMedieval, Mundo::FantasiaOscura, Mundo::FantasiaUrbana,
            Mundo::SciFiSpace, Mundo::SciFiCyberpunk, Mundo::SciFiPostApocaliptico,
            Mundo::Realista, Mundo::HistoricoMedieval, Mundo::HistoricoAntiguo,
            Mundo::HistoricoModerno, Mundo::Victoriano, 
            Mundo::JaponFeudal, Mundo::AnimeFantasia, Mundo::ChinaImperial, Mundo::Wuxia,
            Mundo::CoreaHistorica, Mundo::MitologiaAsiatica,
            Mundo::Anime, Mundo::Mitologico, Mundo::MitologiaGriega, Mundo::MitologiaNordica,
            Mundo::Steampunk, Mundo::Western, Mundo::Noir, Mundo::PiratasCaribe,
        ]
    }
}

/// Nivel de conflicto interno
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NivelConflicto {
    Bajo,
    Medio,
    Alto,
    Extremo,
}

impl NivelConflicto {
    pub fn all() -> Vec<NivelConflicto> {
        vec![NivelConflicto::Bajo, NivelConflicto::Medio, NivelConflicto::Alto, NivelConflicto::Extremo]
    }
}

/// Idioma de generación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Espanol,
    English,
    Japanese,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s {
            "en" => Language::English,
            "jp" => Language::Japanese,
            _ => Language::Espanol,
        }
    }
}

/// Motor principal de generación
pub struct SoulForge {
    rng: ChaCha8Rng,
    config: ForgeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub profundidad_default: Profundidad,
    pub mundo_default: Mundo,
}


impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            profundidad_default: Profundidad::Completa,
            mundo_default: Mundo::FantasiaMedieval,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Profundidad {
    Minima,
    Media,
    Completa,
    Exhaustiva,
}

impl SoulForge {
    pub fn nuevo() -> Self {
        Self {
            rng: ChaCha8Rng::from_entropy(),
            config: ForgeConfig::default(),
        }
    }
    
    pub fn con_semilla(semilla: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(semilla),
            config: ForgeConfig::default(),
        }
    }
    
    /// Forja un alma individual
    pub fn forjar(&mut self, params: ParametrosGeneracion) -> Alma {
        Alma::generar(&mut self.rng, params, &self.config)
    }
    
    /// Forja múltiples almas con relaciones emergentes
    pub fn forjar_constelacion(&mut self, params: ParametrosConstelacion) -> crate::constelacion::Constelacion {
        crate::constelacion::ConstelacionBuilder::new()
            .con_params(params)
            .construir(&mut self.rng)
    }
    
    /// Calcula química entre dos almas
    pub fn calcular_quimica(&self, alma1: &Alma, alma2: &Alma) -> crate::relaciones::Quimica {
        crate::relaciones::Quimica::calcular(alma1, alma2)
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametrosConstelacion {
    pub cantidad: usize,
    pub mundo: Mundo,
    pub densidad_relaciones: DensidadRelaciones,
    pub incluir_antagonista: bool,
    pub incluir_romance: bool,
    pub estructura: EstructuraGrupo,
    pub nombres_fijos: Option<Vec<String>>,
    pub generos_fijos: Option<Vec<Genero>>,
    pub edades_fijas: Option<Vec<u32>>,
    pub tipo_villano: Option<String>, // "Envidia", "Ideologia", "PuraMaldad", "Obstaculo"
    pub modo_romance: Option<String>, // "Pareja", "Triangulo", "AmorProhibido", "Poliamor"
}

impl Default for ParametrosConstelacion {
    fn default() -> Self {
        Self {
            cantidad: 5,
            mundo: Mundo::FantasiaMedieval,
            densidad_relaciones: DensidadRelaciones::Normal,
            incluir_antagonista: true,
            incluir_romance: false,
            estructura: EstructuraGrupo::Grupo,
            nombres_fijos: None,
            generos_fijos: None,
            edades_fijas: None,
            tipo_villano: None,
            modo_romance: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DensidadRelaciones {
    Dispersa,
    Normal,
    Densa,
    Claustrofobica,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EstructuraGrupo {
    Libre,
    Familia,
    Faccion,
    Grupo,
    Corte,
    Triangulo,
    Paralelo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genero {
    Masculino,
    Femenino,
}
