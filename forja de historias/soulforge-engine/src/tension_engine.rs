use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::soul_parser::LivingSoul;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionAnalysis {
    pub overall_score: f32,
    pub tension_waves: Vec<TensionWave>,
    pub triggered_elements: Vec<TriggeredElement>,
    pub narrative_pressure: NarrativePressure,
    pub recommended_escalation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionWave {
    pub position: usize,         // Posición en el texto
    pub intensity: f32,          // 0.0 - 1.0
    pub wave_type: WaveType,
    pub source: String,          // Qué elemento del alma lo causó
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaveType {
    FearApproach,       // Se acerca al miedo del personaje
    DesireConflict,     // Conflicto con su deseo
    ScarResonance,      // Eco de una cicatriz pasada
    ProphecyPulse,      // Latido de una profecía
    NemesisShadow,      // Sombra de la némesis
    BondStrain,         // Tensión en un vínculo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggeredElement {
    pub element_type: String,
    pub element_id: String,
    pub trigger_phrase: String,
    pub resonance_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativePressure {
    pub building: bool,
    pub peak_predicted_at: Option<usize>,
    pub release_needed: bool,
    pub suggested_catharsis: Vec<String>,
}

pub fn calculate_tension(soul_json: &str, narrative: &str) -> TensionAnalysis {
    let soul: LivingSoul = match serde_json::from_str(soul_json) {
        Ok(s) => s,
        Err(_) => return TensionAnalysis::empty(),
    };
    
    let mut waves: Vec<TensionWave> = Vec::new();
    let mut triggered: Vec<TriggeredElement> = Vec::new();
    let narrative_lower = narrative.to_lowercase();
    
    // === ANÁLISIS DE MIEDO ===
    if !soul.psyche.core_fear.is_empty() {
        let fear_keywords = extract_keywords(&soul.psyche.core_fear);
        for keyword in &fear_keywords {
            for (pos, _) in narrative_lower.match_indices(keyword) {
                waves.push(TensionWave {
                    position: pos,
                    intensity: 0.7,
                    wave_type: WaveType::FearApproach,
                    source: format!("core_fear:{}", keyword),
                });
                triggered.push(TriggeredElement {
                    element_type: "fear".to_string(),
                    element_id: "core_fear".to_string(),
                    trigger_phrase: extract_context(narrative, pos, 30),
                    resonance_strength: 0.7,
                });
            }
        }
    }
    
    // === ANÁLISIS DE CICATRICES ===
    for scar in &soul.scars {
        let scar_keywords = extract_keywords(&scar.origin_event);
        for keyword in &scar_keywords {
            for (pos, _) in narrative_lower.match_indices(keyword) {
                waves.push(TensionWave {
                    position: pos,
                    intensity: scar.narrative_weight,
                    wave_type: WaveType::ScarResonance,
                    source: format!("scar:{}", scar.id),
                });
                triggered.push(TriggeredElement {
                    element_type: "scar".to_string(),
                    element_id: scar.id.clone(),
                    trigger_phrase: extract_context(narrative, pos, 30),
                    resonance_strength: scar.narrative_weight,
                });
            }
        }
    }
    
    // === ANÁLISIS DE PROFECÍAS ===
    for prophecy in &soul.prophecies {
        if !prophecy.fulfilled {
            let prophecy_keywords = extract_keywords(&prophecy.text);
            for keyword in &prophecy_keywords {
                for (pos, _) in narrative_lower.match_indices(keyword) {
                    let intensity = match prophecy.prophecy_type {
                        crate::soul_parser::ProphecyType::Doom => 0.9,
                        crate::soul_parser::ProphecyType::Choice => 0.6,
                        _ => 0.4,
                    };
                    waves.push(TensionWave {
                        position: pos,
                        intensity,
                        wave_type: WaveType::ProphecyPulse,
                        source: format!("prophecy:{}", prophecy.id),
                    });
                }
            }
        }
    }
    
    // === ANÁLISIS DE NÉMESIS ===
    if let Some(ref nemesis) = soul.nemesis {
        let nemesis_keywords = extract_keywords(&nemesis.name);
        nemesis_keywords.iter()
            .chain(extract_keywords(&nemesis.conflict_core).iter())
            .for_each(|keyword| {
                for (pos, _) in narrative_lower.match_indices(keyword) {
                    waves.push(TensionWave {
                        position: pos,
                        intensity: 0.85,
                        wave_type: WaveType::NemesisShadow,
                        source: format!("nemesis:{}", nemesis.name),
                    });
                }
            });
    }
    
    // === ANÁLISIS DE VÍNCULOS ===
    for bond in &soul.bonds {
        if matches!(bond.status, 
            crate::soul_parser::BondStatus::Strained | 
            crate::soul_parser::BondStatus::Broken
        ) {
            let bond_keywords = extract_keywords(&bond.entity_name);
            for keyword in &bond_keywords {
                for (pos, _) in narrative_lower.match_indices(keyword) {
                    waves.push(TensionWave {
                        position: pos,
                        intensity: 0.55,
                        wave_type: WaveType::BondStrain,
                        source: format!("bond:{}", bond.entity_name),
                    });
                }
            }
        }
    }
    
    // Ordenar ondas por posición
    waves.sort_by_key(|w| w.position);
    
    // Calcular presión narrativa
    let pressure = calculate_narrative_pressure(&waves, narrative.len());
    
    // Score general
    let overall = if waves.is_empty() {
        soul.tension_markers.overall_tension
    } else {
        let wave_avg: f32 = waves.iter().map(|w| w.intensity).sum::<f32>() 
            / waves.len() as f32;
        (wave_avg * 0.6 + soul.tension_markers.overall_tension * 0.4).min(1.0)
    };
    
    TensionAnalysis {
        overall_score: overall,
        tension_waves: waves,
        triggered_elements: triggered,
        narrative_pressure: pressure,
        recommended_escalation: suggest_escalation(&soul, overall),
    }
}

fn extract_keywords(text: &str) -> Vec<String> {
    let stopwords = ["el", "la", "de", "en", "que", "y", "a", "los", "las", 
                     "un", "una", "the", "of", "and", "to", "in", "for"];
    
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3 && !stopwords.contains(w))
        .map(|s| s.to_string())
        .collect()
}

fn extract_context(text: &str, pos: usize, radius: usize) -> String {
    let start = pos.saturating_sub(radius);
    let end = (pos + radius).min(text.len());
    text.get(start..end).unwrap_or("").to_string()
}

fn calculate_narrative_pressure(waves: &[TensionWave], text_len: usize) -> NarrativePressure {
    if waves.is_empty() {
        return NarrativePressure {
            building: false,
            peak_predicted_at: None,
            release_needed: false,
            suggested_catharsis: vec![],
        };
    }
    
    // Detectar si la tensión está subiendo
    let segments = 4;
    let segment_size = text_len / segments;
    let mut segment_tensions: Vec<f32> = vec![0.0; segments];
    
    for wave in waves {
        let seg_idx = (wave.position / segment_size.max(1)).min(segments - 1);
        segment_tensions[seg_idx] += wave.intensity;
    }
    
    let building = segment_tensions.windows(2)
        .all(|w| w[1] >= w[0]);
    
    let total_tension: f32 = waves.iter().map(|w| w.intensity).sum();
    
    NarrativePressure {
        building,
        peak_predicted_at: if building { Some(text_len + 500) } else { None },
        release_needed: total_tension > 2.0,
        suggested_catharsis: if total_tension > 2.0 {
            vec![
                "Escena de catarsis emocional".to_string(),
                "Momento de victoria menor".to_string(),
                "Revelación que alivia tensión".to_string(),
            ]
        } else {
            vec![]
        },
    }
}

fn suggest_escalation(soul: &LivingSoul, tension: f32) -> Option<String> {
    if tension < 0.4 {
        // Tensión baja: sugerir aproximación al conflicto
        if let Some(ref nemesis) = soul.nemesis {
            return Some(format!(
                "La tensión es baja. Considera introducir una señal de {}.",
                nemesis.name
            ));
        }
        if !soul.prophecies.is_empty() {
            return Some(
                "La tensión es baja. Un presagio de la profecía aumentaría el interés."
                .to_string()
            );
        }
    }
    None
}

impl TensionAnalysis {
    fn empty() -> Self {
        Self {
            overall_score: 0.0,
            tension_waves: vec![],
            triggered_elements: vec![],
            narrative_pressure: NarrativePressure {
                building: false,
                peak_predicted_at: None,
                release_needed: false,
                suggested_catharsis: vec![],
            },
            recommended_escalation: None,
        }
    }
}
