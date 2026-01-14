//! Sistema de arcos narrativos

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Rol, TonoMoral};
use super::capas::Mentira;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcoNarrativo {
    pub tipo: TipoArco,
    pub estado_inicial: String,
    pub punto_de_quiebre: String,
    pub climax_potencial: String,
    pub resolucion_positiva: String,
    pub resolucion_tragica: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipoArco {
    Ascenso,         // De debilidad a fortaleza
    Caida,           // De gloria a destrucción
    Plano,           // No cambia pero cambia a otros
    Redencion,       // De oscuridad a luz
    Corrupcion,      // De luz a oscuridad
    Transformacion,  // Ni bueno ni malo, solo diferente
}

impl ArcoNarrativo {
    pub fn generar(rng: &mut impl Rng, rol: &Rol, tono: &TonoMoral, mentira: &Mentira) -> Self {
        let tipo = match (rol, tono) {
            (Rol::Heroe, TonoMoral::Luminoso | TonoMoral::Claro) => TipoArco::Ascenso,
            (Rol::Villano, TonoMoral::Oscuro | TonoMoral::Abismal) => TipoArco::Caida,
            (Rol::Villano, TonoMoral::Gris) => {
                if rng.gen_bool(0.5) { TipoArco::Redencion } else { TipoArco::Caida }
            },
            (Rol::Heroe, TonoMoral::Oscuro) => TipoArco::Corrupcion,
            (Rol::Mentor, _) => TipoArco::Plano,
            _ => TipoArco::Transformacion,
        };
        
        let estado_inicial = match tipo {
            TipoArco::Ascenso => "Perdido, incompleto, sin saber su potencial",
            TipoArco::Caida => "En la cima, arrogante, ciego a sus debilidades",
            TipoArco::Plano => "Formado, resistente, existe para forjar a otros",
            TipoArco::Redencion => "Manchado, perseguido por su pasado",
            TipoArco::Corrupcion => "Puro de corazón pero ingenuo",
            TipoArco::Transformacion => "En conflicto, en el umbral entre dos mundos",
        };
        
        Self {
            tipo,
            estado_inicial: estado_inicial.to_string(),
            punto_de_quiebre: format!(
                "El momento donde {} y debe elegir", 
                mentira.catalizador_potencial.to_lowercase()
            ),
            climax_potencial: "La prueba final que definirá quién decide ser".to_string(),
            resolucion_positiva: format!(
                "Aprende que {}. Finalmente en paz, aunque con cicatrices.",
                mentira.verdad_necesaria.to_lowercase()
            ),
            resolucion_tragica: format!(
                "Nunca supera su creencia de que '{}'. Se pierde a sí mismo.",
                mentira.la_mentira.to_lowercase()
            ),
        }
    }
}
