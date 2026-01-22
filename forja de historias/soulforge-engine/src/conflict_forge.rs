use serde::{Deserialize, Serialize};
use crate::soul_parser::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflectionPoint {
    pub id: String,
    pub title: String,
    pub description: String,
    pub conflict_type: ConflictType,
    pub intensity: f32,
    pub triggered_by: Vec<String>,      // Elementos del alma que lo generan
    pub potential_outcomes: Vec<Outcome>,
    pub narrative_hook: String,         // Frase para insertar en la historia
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    InternalStruggle,    // Conflicto consigo mismo
    NemesisConfrontation,
    ProphecyFulfillment,
    BondTest,            // Prueba de un vínculo
    FearFacing,          // Enfrentar el miedo
    MoralDilemma,
    IdentityCrisis,
    ScarReopening,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub description: String,
    pub soul_impact: SoulImpact,
    pub probability_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulImpact {
    pub affected_element: String,
    pub change_type: ChangeType,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Heal,       // Sanar una cicatriz
    Deepen,     // Profundizar un rasgo
    Break,      // Romper un vínculo
    Strengthen, // Fortalecer algo
    Transform,  // Transformar completamente
    Fulfill,    // Cumplir una profecía
}

pub fn create_inflection_points(soul_json: &str) -> Vec<InflectionPoint> {
    let soul: LivingSoul = match serde_json::from_str(soul_json) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    
    let mut points: Vec<InflectionPoint> = Vec::new();
    let mut point_id = 0;
    
    // === PUNTOS BASADOS EN MIEDO ===
    if !soul.psyche.core_fear.is_empty() {
        point_id += 1;
        points.push(InflectionPoint {
            id: format!("ip_{}", point_id),
            title: format!("La Prueba del Terror"),
            description: format!(
                "{} debe enfrentar directamente su miedo más profundo: {}. \
                No hay escape, no hay rodeo.",
                soul.name, soul.psyche.core_fear
            ),
            conflict_type: ConflictType::FearFacing,
            intensity: 0.85,
            triggered_by: vec!["psyche.core_fear".to_string()],
            potential_outcomes: vec![
                Outcome {
                    description: "Supera el miedo, gana control".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "psyche.core_fear".to_string(),
                        change_type: ChangeType::Transform,
                        new_value: Some("miedo superado -> fortaleza".to_string()),
                    },
                    probability_weight: 0.3,
                },
                Outcome {
                    description: "Colapsa, el miedo lo consume temporalmente".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "psyche.core_fear".to_string(),
                        change_type: ChangeType::Deepen,
                        new_value: None,
                    },
                    probability_weight: 0.4,
                },
                Outcome {
                    description: "Huye, pero el miedo lo perseguirá".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "scars".to_string(),
                        change_type: ChangeType::Deepen,
                        new_value: Some("Nueva cicatriz: La Huida".to_string()),
                    },
                    probability_weight: 0.3,
                },
            ],
            narrative_hook: format!(
                "Y entonces, sin previo aviso, {} se materializó ante sus ojos. \
                El único escape era atravesarlo.",
                soul.psyche.core_fear
            ),
        });
    }
    
    // === PUNTOS BASADOS EN NÉMESIS ===
    if let Some(ref nemesis) = soul.nemesis {
        point_id += 1;
        points.push(InflectionPoint {
            id: format!("ip_{}", point_id),
            title: format!("El Encuentro con {}", nemesis.name),
            description: format!(
                "El conflicto entre {} y {} alcanza su punto crítico. \
                El origen de su enemistad ({}) ya no puede ignorarse.",
                soul.name, nemesis.name, nemesis.conflict_core
            ),
            conflict_type: ConflictType::NemesisConfrontation,
            intensity: 0.95,
            triggered_by: vec!["nemesis".to_string()],
            potential_outcomes: vec![
                Outcome {
                    description: format!("Derrota a {}", nemesis.name),
                    soul_impact: SoulImpact {
                        affected_element: "nemesis".to_string(),
                        change_type: ChangeType::Transform,
                        new_value: Some("Némesis derrotada".to_string()),
                    },
                    probability_weight: 0.25,
                },
                Outcome {
                    description: "Reconciliación inesperada".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "nemesis".to_string(),
                        change_type: ChangeType::Transform,
                        new_value: Some("Antiguo enemigo -> aliado incómodo".to_string()),
                    },
                    probability_weight: 0.15,
                },
                Outcome {
                    description: format!("Pierde contra {}", nemesis.name),
                    soul_impact: SoulImpact {
                        affected_element: "scars".to_string(),
                        change_type: ChangeType::Deepen,
                        new_value: Some("Nueva cicatriz de derrota".to_string()),
                    },
                    probability_weight: 0.35,
                },
                Outcome {
                    description: "Empate que deja todo más enredado".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "tension_markers".to_string(),
                        change_type: ChangeType::Deepen,
                        new_value: None,
                    },
                    probability_weight: 0.25,
                },
            ],
            narrative_hook: format!(
                "\"{}...\" La voz de {} cortó el silencio como una cuchilla. \
                \"Sabía que este día llegaría.\"",
                soul.name, nemesis.name
            ),
        });
    }
    
    // === PUNTOS BASADOS EN PROFECÍAS ===
    for prophecy in &soul.prophecies {
        if !prophecy.fulfilled {
            point_id += 1;
            let conflict_type = match prophecy.prophecy_type {
                ProphecyType::Doom => ConflictType::ProphecyFulfillment,
                ProphecyType::Choice => ConflictType::MoralDilemma,
                _ => ConflictType::ProphecyFulfillment,
            };
            
            points.push(InflectionPoint {
                id: format!("ip_{}", point_id),
                title: format!("El Peso del Destino"),
                description: format!(
                    "La profecía '{}' comienza a manifestarse. \
                    {} debe decidir si luchar contra el destino o aceptarlo.",
                    prophecy.text, soul.name
                ),
                conflict_type,
                intensity: match prophecy.prophecy_type {
                    ProphecyType::Doom => 0.9,
                    ProphecyType::Choice => 0.75,
                    _ => 0.6,
                },
                triggered_by: vec![format!("prophecies.{}", prophecy.id)],
                potential_outcomes: vec![
                    Outcome {
                        description: "Cumple la profecía".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("prophecies.{}", prophecy.id),
                            change_type: ChangeType::Fulfill,
                            new_value: Some("fulfilled: true".to_string()),
                        },
                        probability_weight: 0.4,
                    },
                    Outcome {
                        description: "Desafía y reescribe su destino".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("prophecies.{}", prophecy.id),
                            change_type: ChangeType::Transform,
                            new_value: Some("Destino reescrito".to_string()),
                        },
                        probability_weight: 0.3,
                    },
                    Outcome {
                        description: "Pospone lo inevitable".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: "tension_markers.prophecy_pressure".to_string(),
                            change_type: ChangeType::Deepen,
                            new_value: None,
                        },
                        probability_weight: 0.3,
                    },
                ],
                narrative_hook: format!(
                    "Las palabras del antiguo oráculo resonaron en su mente: \
                    \"{}\". Y hoy, esas palabras cobraban vida.",
                    prophecy.text
                ),
            });
        }
    }
    
    // === PUNTOS BASADOS EN CICATRICES ===
    for scar in &soul.scars {
        if !scar.healed && scar.narrative_weight > 0.6 {
            point_id += 1;
            points.push(InflectionPoint {
                id: format!("ip_{}", point_id),
                title: format!("Las Heridas que Hablan"),
                description: format!(
                    "La cicatriz '{}' (origen: {}) se abre de nuevo. \
                    {} debe procesarla o será consumido.",
                    scar.name, scar.origin_event, soul.name
                ),
                conflict_type: ConflictType::ScarReopening,
                intensity: scar.narrative_weight,
                triggered_by: vec![format!("scars.{}", scar.id)],
                potential_outcomes: vec![
                    Outcome {
                        description: "Sanación a través del dolor".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("scars.{}", scar.id),
                            change_type: ChangeType::Heal,
                            new_value: Some("healed: true".to_string()),
                        },
                        probability_weight: 0.35,
                    },
                    Outcome {
                        description: "La herida se infecta más".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("scars.{}", scar.id),
                            change_type: ChangeType::Deepen,
                            new_value: Some("narrative_weight: increased".to_string()),
                        },
                        probability_weight: 0.35,
                    },
                    Outcome {
                        description: "Transforma el dolor en propósito".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("scars.{}", scar.id),
                            change_type: ChangeType::Transform,
                            new_value: Some("Cicatriz convertida en fuerza".to_string()),
                        },
                        probability_weight: 0.3,
                    },
                ],
                narrative_hook: format!(
                    "No esperaba que {} le recordara tanto a aquel día. \
                    El día de {}. Su mano tembló involuntariamente.",
                    "aquello", scar.origin_event
                ),
            });
        }
    }
    
    // === PUNTOS BASADOS EN VÍNCULOS TENSOS ===
    for bond in &soul.bonds {
        if matches!(bond.status, BondStatus::Strained) {
            point_id += 1;
            points.push(InflectionPoint {
                id: format!("ip_{}", point_id),
                title: format!("La Prueba del Vínculo"),
                description: format!(
                    "La relación entre {} y {} ({:?}) ha llegado a un punto crítico. \
                    Una decisión los unirá para siempre o los separará definitivamente.",
                    soul.name, bond.entity_name, bond.bond_type
                ),
                conflict_type: ConflictType::BondTest,
                intensity: 0.7,
                triggered_by: vec![format!("bonds.{}", bond.entity_name)],
                potential_outcomes: vec![
                    Outcome {
                        description: "Fortalecen el vínculo".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("bonds.{}", bond.entity_name),
                            change_type: ChangeType::Strengthen,
                            new_value: Some("status: Active, strength: increased".to_string()),
                        },
                        probability_weight: 0.4,
                    },
                    Outcome {
                        description: "El vínculo se rompe".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("bonds.{}", bond.entity_name),
                            change_type: ChangeType::Break,
                            new_value: Some("status: Broken".to_string()),
                        },
                        probability_weight: 0.3,
                    },
                    Outcome {
                        description: "Transformación de la relación".to_string(),
                        soul_impact: SoulImpact {
                            affected_element: format!("bonds.{}", bond.entity_name),
                            change_type: ChangeType::Transform,
                            new_value: Some("bond_type: changed".to_string()),
                        },
                        probability_weight: 0.3,
                    },
                ],
                narrative_hook: format!(
                    "\"No puedo seguir así\", dijo {}. \
                    \"Necesito saber si todavía significo algo para ti.\"",
                    bond.entity_name
                ),
            });
        }
    }
    
    // === PUNTO DE DILEMA MORAL ===
    if !soul.psyche.moral_code.is_empty() {
        point_id += 1;
        let main_rule = soul.psyche.moral_code.first().unwrap();
        points.push(InflectionPoint {
            id: format!("ip_{}", point_id),
            title: "El Peso de los Principios".to_string(),
            description: format!(
                "{} se enfrenta a una situación donde seguir su código moral ('{}') \
                tendrá consecuencias devastadoras. Romperlo también.",
                soul.name, main_rule
            ),
            conflict_type: ConflictType::MoralDilemma,
            intensity: 0.8,
            triggered_by: vec!["psyche.moral_code".to_string()],
            potential_outcomes: vec![
                Outcome {
                    description: "Mantiene su código, paga el precio".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "psyche.moral_code".to_string(),
                        change_type: ChangeType::Strengthen,
                        new_value: None,
                    },
                    probability_weight: 0.4,
                },
                Outcome {
                    description: "Rompe su código, gana pero se pierde".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "scars".to_string(),
                        change_type: ChangeType::Deepen,
                        new_value: Some("Nueva cicatriz: Principios Rotos".to_string()),
                    },
                    probability_weight: 0.35,
                },
                Outcome {
                    description: "Encuentra una tercera vía".to_string(),
                    soul_impact: SoulImpact {
                        affected_element: "psyche.moral_code".to_string(),
                        change_type: ChangeType::Transform,
                        new_value: Some("Código moral evolucionado".to_string()),
                    },
                    probability_weight: 0.25,
                },
            ],
            narrative_hook: format!(
                "La elección era imposible. Si seguía su regla de '{}', \
                aquellos que amaba sufrirían. Si la rompía, \
                ¿en qué se convertiría?",
                main_rule
            ),
        });
    }
    
    // Ordenar por intensidad
    points.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
    
    points
}
