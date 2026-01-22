use serde::{Deserialize, Serialize};
use crate::soul_parser::LivingSoul;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceViolation {
    pub severity: ViolationSeverity,
    pub violation_type: ViolationType,
    pub description: String,
    pub conflicting_element: String,
    pub source_reference: String,
    pub suggestion: String,
    pub position_hint: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Critical,   // Contradicción directa con hechos establecidos
    Warning,    // Inconsistencia probable
    Notice,     // Posible desviación del personaje
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    BiographyContradiction,  // Contradice historia pasada
    PersonalityBreach,       // Acción fuera de carácter
    ScarViolation,           // Ignora trauma sin resolución
    BondInconsistency,       // Trato inconsistente a un vínculo
    ProphecyConflict,        // Conflicto con profecía cumplida
    DeadCharacterAppears,    // Personaje muerto aparece vivo
    TimelineError,           // Error de cronología
    MoralCodeViolation,      // Viola código moral sin justificación
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceReport {
    pub is_coherent: bool,
    pub coherence_score: f32,
    pub violations: Vec<CoherenceViolation>,
    pub character_drift_warning: Option<String>,
    pub suggested_revisions: Vec<String>,
}

pub fn check_violations(soul_json: &str, new_content: &str) -> CoherenceReport {
    let soul: LivingSoul = match serde_json::from_str(soul_json) {
        Ok(s) => s,
        Err(_) => return CoherenceReport::empty(),
    };
    
    let mut violations: Vec<CoherenceViolation> = Vec::new();
    let content_lower = new_content.to_lowercase();
    
    // === VERIFICACIÓN DE CÓDIGO MORAL ===
    violations.extend(check_moral_code(&soul, &content_lower));
    
    // === VERIFICACIÓN DE CICATRICES NO SANADAS ===
    violations.extend(check_scar_consistency(&soul, &content_lower));
    
    // === VERIFICACIÓN DE VÍNCULOS ===
    violations.extend(check_bond_consistency(&soul, &content_lower));
    
    // === VERIFICACIÓN DE MIEDOS (si los enfrenta sin desarrollo) ===
    violations.extend(check_fear_handling(&soul, &content_lower));
    
    // === VERIFICACIÓN DE PROFECÍAS CUMPLIDAS ===
    violations.extend(check_prophecy_status(&soul, &content_lower));
    
    // === VERIFICACIÓN DE CRÓNICA (eventos pasados) ===
    violations.extend(check_chronicle_consistency(&soul, &content_lower));
    
    // Calcular score de coherencia
    let violation_weight: f32 = violations.iter()
        .map(|v| match v.severity {
            ViolationSeverity::Critical => 0.3,
            ViolationSeverity::Warning => 0.15,
            ViolationSeverity::Notice => 0.05,
        })
        .sum();
    
    let coherence_score = (1.0 - violation_weight).max(0.0);
    
    // Detectar deriva de personaje
    let drift_warning = detect_character_drift(&soul, &content_lower);
    
    CoherenceReport {
        is_coherent: violations.iter()
            .all(|v| !matches!(v.severity, ViolationSeverity::Critical)),
        coherence_score,
        violations,
        character_drift_warning: drift_warning,
        suggested_revisions: generate_revision_suggestions(&soul),
    }
}

fn check_moral_code(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    
    // Patrones que podrían violar códigos morales comunes
    let violation_patterns = [
        ("nunca matar", vec!["mató", "asesinó", "ejecutó", "dio muerte"]),
        ("proteger inocentes", vec!["abandonó al niño", "dejó morir", "ignoró el llanto"]),
        ("nunca mentir", vec!["mintió descaradamente", "engañó", "fabricó una historia"]),
        ("honor ante todo", vec!["huyó cobardemente", "traicionó su palabra"]),
        ("nunca robar", vec!["robó", "hurtó", "se apropió"]),
    ];
    
    for moral_rule in &soul.psyche.moral_code {
        let rule_lower = moral_rule.to_lowercase();
        
        for (pattern, triggers) in &violation_patterns {
            if rule_lower.contains(pattern) {
                for trigger in triggers {
                    if content.contains(trigger) {
                        violations.push(CoherenceViolation {
                            severity: ViolationSeverity::Critical,
                            violation_type: ViolationType::MoralCodeViolation,
                            description: format!(
                                "La acción '{}' contradice el código moral: '{}'",
                                trigger, moral_rule
                            ),
                            conflicting_element: moral_rule.clone(),
                            source_reference: "psyche.moral_code".to_string(),
                            suggestion: format!(
                                "Si {} va a romper su código, necesita una justificación \
                                narrativa poderosa o mostrar el conflicto interno.",
                                soul.name
                            ),
                            position_hint: content.find(trigger),
                        });
                    }
                }
            }
        }
    }
    
    violations
}

fn check_scar_consistency(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    
    for scar in &soul.scars {
        if !scar.healed {
            // Buscar si el personaje actúa normalmente en situaciones 
            // que deberían activar el trauma
            match scar.trauma_type {
                crate::soul_parser::TraumaType::Betrayal => {
                    if content.contains("confió ciegamente") 
                        || content.contains("sin dudar un segundo") {
                        violations.push(CoherenceViolation {
                            severity: ViolationSeverity::Warning,
                            violation_type: ViolationType::ScarViolation,
                            description: format!(
                                "{} muestra confianza ciega, pero tiene trauma \
                                de traición sin sanar: '{}'",
                                soul.name, scar.name
                            ),
                            conflicting_element: scar.name.clone(),
                            source_reference: format!("scars.{}", scar.id),
                            suggestion: "Mostrar hesitación, flashback, o conflicto interno \
                                antes de confiar.".to_string(),
                            position_hint: None,
                        });
                    }
                },
                crate::soul_parser::TraumaType::Loss => {
                    // Similar para pérdida
                },
                _ => {}
            }
        }
    }
    
    violations
}

fn check_bond_consistency(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    
    for bond in &soul.bonds {
        let entity_lower = bond.entity_name.to_lowercase();
        
        if content.contains(&entity_lower) {
            match bond.status {
                crate::soul_parser::BondStatus::Broken => {
                    // Si el vínculo está roto pero se trata calurosamente
                    if content.contains(&format!("abrazó a {}", entity_lower))
                        || content.contains(&format!("{} sonrió a", entity_lower)) {
                        violations.push(CoherenceViolation {
                            severity: ViolationSeverity::Warning,
                            violation_type: ViolationType::BondInconsistency,
                            description: format!(
                                "Interacción cálida con {}, pero el vínculo está marcado como ROTO.",
                                bond.entity_name
                            ),
                            conflicting_element: bond.entity_name.clone(),
                            source_reference: format!("bonds.{}", bond.entity_name),
                            suggestion: "Si hay reconciliación, mostrar el proceso. \
                                Si no, ajustar la interacción.".to_string(),
                            position_hint: None,
                        });
                    }
                },
                crate::soul_parser::BondStatus::Strained => {
                    // Buscar interacciones que ignoren la tensión
                },
                _ => {}
            }
        }
    }
    
    violations
}

fn check_fear_handling(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    let fear_lower = soul.psyche.core_fear.to_lowercase();
    
    if fear_lower.is_empty() {
        return violations;
    }
    
    // Si el contenido toca el miedo pero el personaje no reacciona
    let fear_keywords: Vec<&str> = fear_lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3)
        .collect();
    
    let emotional_indicators = [
        "temblor", "sudor", "pánico", "terror", "miedo", "hesitó",
        "retrocedió", "palideció", "corazón acelerado", "angustia"
    ];
    
    for keyword in fear_keywords {
        if content.contains(keyword) {
            // Verificar si hay indicadores emocionales cerca
            let has_emotional_response = emotional_indicators
                .iter()
                .any(|ind| content.contains(ind));
            
            if !has_emotional_response {
                violations.push(CoherenceViolation {
                    severity: ViolationSeverity::Notice,
                    violation_type: ViolationType::PersonalityBreach,
                    description: format!(
                        "La escena toca el miedo fundamental de {} ('{}') \
                        pero no hay respuesta emocional visible.",
                        soul.name, soul.psyche.core_fear
                    ),
                    conflicting_element: soul.psyche.core_fear.clone(),
                    source_reference: "psyche.core_fear".to_string(),
                    suggestion: "Añadir señales sutiles de incomodidad, \
                        o mostrar la supresión consciente del miedo.".to_string(),
                    position_hint: content.find(keyword),
                });
            }
        }
    }
    
    violations
}

fn check_prophecy_status(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    
    for prophecy in &soul.prophecies {
        if prophecy.fulfilled {
            // Verificar que no se trate como pendiente
            let prophecy_keywords: Vec<&str> = prophecy.text
                .to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| w.len() > 4)
                .collect();
            
            for keyword in prophecy_keywords {
                if content.contains(keyword) 
                    && (content.contains("algún día") 
                        || content.contains("está destinado a")
                        || content.contains("la profecía dice")) {
                    violations.push(CoherenceViolation {
                        severity: ViolationSeverity::Warning,
                        violation_type: ViolationType::ProphecyConflict,
                        description: format!(
                            "Se habla de la profecía '{}' como pendiente, \
                            pero está marcada como CUMPLIDA.",
                            prophecy.text
                        ),
                        conflicting_element: prophecy.id.clone(),
                        source_reference: format!("prophecies.{}", prophecy.id),
                        suggestion: "Referirse a la profecía en pasado, \
                            o actualizar su estado.".to_string(),
                        position_hint: None,
                    });
                }
            }
        }
    }
    
    violations
}

fn check_chronicle_consistency(soul: &LivingSoul, content: &str) -> Vec<CoherenceViolation> {
    let mut violations = Vec::new();
    
    // Revisar que no se contradigan eventos pasados
    for entry in &soul.chronicle {
        for consequence in &entry.consequences {
            let cons_lower = consequence.to_lowercase();
            
            // Detectar si alguien murió y aparece vivo
            if cons_lower.contains("muerte de") || cons_lower.contains("murió") {
                // Extraer nombre del muerto
                let parts: Vec<&str> = cons_lower.split("muerte de").collect();
                if parts.len() > 1 {
                    let dead_name = parts[1].split_whitespace().next().unwrap_or("");
                    if content.contains(dead_name) 
                        && (content.contains(&format!("{} dijo", dead_name))
                            || content.contains(&format!("{} apareció", dead_name))) {
                        violations.push(CoherenceViolation {
                            severity: ViolationSeverity::Critical,
                            violation_type: ViolationType::DeadCharacterAppears,
                            description: format!(
                                "'{}' aparece activo en la narrativa, \
                                pero murió en el Capítulo {}.",
                                dead_name, entry.chapter
                            ),
                            conflicting_element: consequence.clone(),
                            source_reference: format!("chronicle.chapter_{}", entry.chapter),
                            suggestion: "Verificar si es flashback, visión, \
                                o error narrativo.".to_string(),
                            position_hint: content.find(dead_name),
                        });
                    }
                }
            }
        }
    }
    
    violations
}

fn detect_character_drift(soul: &LivingSoul, content: &str) -> Option<String> {
    // Detectar si el personaje se está alejando mucho de su esencia
    let archetype_lower = soul.archetype.to_lowercase();
    
    // Definir comportamientos esperados vs contradictorios por arquetipo
    let archetype_conflicts: Vec<(&str, Vec<&str>)> = vec![
        ("héroe", vec!["huyó abandonando", "sacrificó al inocente"]),
        ("sabio", vec!["actuó sin pensar", "ignoró la evidencia"]),
        ("rebelde", vec!["obedeció sin cuestionar", "siguió las reglas"]),
        ("cuidador", vec!["abandonó al necesitado", "priorizó su beneficio"]),
    ];
    
    for (archetype, conflicts) in archetype_conflicts {
        if archetype_lower.contains(archetype) {
            for conflict in conflicts {
                if content.contains(conflict) {
                    return Some(format!(
                        "⚠️ Deriva de Personaje: {} tiene arquetipo de '{}' \
                        pero la acción '{}' sugiere un cambio fundamental. \
                        ¿Es intencional?",
                        soul.name, soul.archetype, conflict
                    ));
                }
            }
        }
    }
    
    None
}

fn generate_revision_suggestions(soul: &LivingSoul) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    // Sugerencias basadas en elementos del alma
    if !soul.psyche.core_fear.is_empty() {
        suggestions.push(format!(
            "Recuerda: el miedo fundamental de {} es '{}'. \
            Las escenas de tensión deberían resonar con esto.",
            soul.name, soul.psyche.core_fear
        ));
    }
    
    if let Some(ref nemesis) = soul.nemesis {
        suggestions.push(format!(
            "La némesis {} siempre debe sentirse como una amenaza latente, \
            incluso cuando no aparece directamente.",
            nemesis.name
        ));
    }
    
    suggestions
}

impl CoherenceReport {
    fn empty() -> Self {
        Self {
            is_coherent: true,
            coherence_score: 1.0,
            violations: vec![],
            character_drift_warning: None,
            suggested_revisions: vec![],
        }
    }
}
