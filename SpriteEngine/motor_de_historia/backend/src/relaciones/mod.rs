//! Sistema de Relaciones y Química entre personajes
//! 
//! La innovación clave: los conflictos EMERGEN de las psicologías de los personajes,
//! no se asignan arbitrariamente.

mod quimica;
mod vinculos;

pub use quimica::*;
pub use vinculos::*;

use serde::{Deserialize, Serialize};
use crate::core::Alma;

/// Tipo de relación entre dos personajes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoRelacion {
    // Positivas
    Alianza,
    Amistad,
    Romance,
    Mentor,
    Protector,
    Hermandad,
    
    // Negativas
    Rivalidad,
    Enemistad,
    Traicion,
    Desconfianza,
    
    // Complejas
    Ambivalente,     // Amor-odio
    Codependencia,
    Espejo,          // Se ven reflejados
    Catalizador,     // Provocan cambio mutuo
}

/// Una relación concreta entre dos personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vinculo {
    pub id_persona_a: uuid::Uuid,
    pub id_persona_b: uuid::Uuid,
    pub tipo: TipoRelacion,
    pub intensidad: f32,  // 0.0 a 1.0
    pub quimica: Quimica,
    pub historia_compartida: Option<String>,
    pub tension_activa: Option<String>,
    pub potencial_narrativo: Vec<String>,
}

impl Vinculo {
    pub fn crear(alma_a: &Alma, alma_b: &Alma) -> Self {
        let quimica = Quimica::calcular(alma_a, alma_b);
        let tipo = Self::determinar_tipo(&quimica, alma_a, alma_b);
        let intensidad = quimica.intensidad_total();
        
        Self {
            id_persona_a: alma_a.id,
            id_persona_b: alma_b.id,
            tipo,
            intensidad,
            historia_compartida: Self::generar_historia(&quimica, alma_a, alma_b),
            tension_activa: Self::generar_tension(&quimica, alma_a, alma_b),
            potencial_narrativo: Self::generar_potencial(&quimica, &tipo),
            quimica,
        }
    }
    
    fn determinar_tipo(quimica: &Quimica, _a: &Alma, _b: &Alma) -> TipoRelacion {
        // El tipo emerge de la química psicológica
        
        if quimica.compatibilidad_heridas > 0.7 && quimica.atraccion_arquetipos > 0.6 {
            // Alta compatibilidad de heridas + atracción = potencial romance o amistad profunda
            if quimica.conflicto_sombras < 0.3 {
                TipoRelacion::Romance
            } else {
                TipoRelacion::Ambivalente
            }
        } else if quimica.conflicto_sombras > 0.7 {
            // Alto conflicto de sombras = rivalidad o enemistad
            if quimica.deseo_necesidad_tension > 0.5 {
                TipoRelacion::Enemistad
            } else {
                TipoRelacion::Rivalidad
            }
        } else if quimica.complementariedad > 0.6 {
            // Se complementan bien
            if quimica.potencial_catalitico > 0.5 {
                TipoRelacion::Catalizador
            } else {
                TipoRelacion::Alianza
            }
        } else if quimica.espejo > 0.7 {
            // Se ven reflejados uno en el otro
            TipoRelacion::Espejo
        } else if quimica.compatibilidad_heridas > 0.5 {
            TipoRelacion::Amistad
        } else {
            TipoRelacion::Desconfianza
        }
    }
    
    fn generar_historia(quimica: &Quimica, a: &Alma, b: &Alma) -> Option<String> {
        if quimica.intensidad_total() > 0.5 {
            let historias = vec![
                format!(
                    "Se conocieron cuando {} necesitaba ayuda, y {} fue el único que respondió.",
                    a.identidad.nombre, b.identidad.nombre
                ),
                format!(
                    "Un encuentro fortuito reveló que ambos comparten {} - un secreto que nadie más conoce.",
                    if quimica.compatibilidad_heridas > 0.5 { "un pasado similar" } else { "un objetivo" }
                ),
                format!(
                    "Estuvieron en lados opuestos de un conflicto, hasta que descubrieron la verdad.",
                ),
            ];
            Some(historias[rand::random::<usize>() % historias.len()].clone())
        } else {
            None
        }
    }
    
    fn generar_tension(quimica: &Quimica, a: &Alma, b: &Alma) -> Option<String> {
        if quimica.conflicto_sombras > 0.4 || quimica.deseo_necesidad_tension > 0.4 {
            let tensiones = vec![
                format!(
                    "{} ve en {} todo lo que niega de sí mismo - y no puede ignorarlo.",
                    a.identidad.nombre, b.identidad.nombre
                ),
                format!(
                    "Ambos quieren lo mismo, pero solo uno puede tenerlo.",
                ),
                format!(
                    "{} sabe un secreto sobre {} que podría destruirlo.",
                    b.identidad.nombre, a.identidad.nombre
                ),
                format!(
                    "La lealtad de {} está dividida, y {} lo sabe.",
                    a.identidad.nombre, b.identidad.nombre
                ),
            ];
            Some(tensiones[rand::random::<usize>() % tensiones.len()].clone())
        } else {
            None
        }
    }
    
    fn generar_potencial(quimica: &Quimica, tipo: &TipoRelacion) -> Vec<String> {
        let mut potencial = Vec::new();
        
        match tipo {
            TipoRelacion::Ambivalente => {
                potencial.push("La tensión podría estallar en conflicto abierto".to_string());
                potencial.push("O podría transformarse en el vínculo más profundo".to_string());
            },
            TipoRelacion::Rivalidad => {
                potencial.push("Podrían convertirse en los peores enemigos".to_string());
                potencial.push("O reconocer que son más similares de lo que admiten".to_string());
            },
            TipoRelacion::Espejo => {
                potencial.push("Verse reflejados podría ser devastador".to_string());
                potencial.push("O podría ser el camino a la auto-aceptación".to_string());
            },
            TipoRelacion::Catalizador => {
                potencial.push("Juntos podrían alcanzar su potencial".to_string());
                potencial.push("O destruirse mutuamente en el proceso".to_string());
            },
            _ => {
                potencial.push("La relación evolucionará con los eventos".to_string());
            }
        }
        
        if quimica.potencial_catalitico > 0.6 {
            potencial.push("Uno podría ser la clave para que el otro supere su mentira".to_string());
        }
        
        potencial
    }
}
