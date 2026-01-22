use serde::{Deserialize, Serialize};
use crate::soul_parser::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    pub id: String,
    pub name: String,
    pub creator: String,
    pub world_laws: Vec<WorldLaw>,
    pub factions: Vec<Faction>,
    pub active_conflicts: Vec<GlobalConflict>,
    pub environmental_rules: EnvironmentalRules,
    pub narrative_tone: NarrativeTone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldLaw {
    pub id: String,
    pub name: String,
    pub description: String,
    pub affects_archetypes: Vec<String>,
    pub effect: LawEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LawEffect {
    Buff { stat: String, modifier: f32 },
    Debuff { stat: String, modifier: f32 },
    Restriction { action: String },
    Enhancement { ability: String },
    SocialModifier { reputation_change: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    pub alignment: String,
    pub accepts_archetypes: Vec<String>,
    pub rejects_archetypes: Vec<String>,
    pub benefits: Vec<String>,
    pub obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConflict {
    pub name: String,
    pub sides: Vec<String>,
    pub stakes: String,
    pub how_souls_are_affected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRules {
    pub magic_level: MagicLevel,
    pub technology_level: TechLevel,
    pub danger_level: f32,
    pub special_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MagicLevel {
    None, Low, Medium, High, Mythical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechLevel {
    Primitive, Medieval, Renaissance, Industrial, Modern, Futuristic, Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTone {
    pub darkness: f32,      // 0.0 = light, 1.0 = grimdark
    pub hope: f32,          // 0.0 = hopeless, 1.0 = hopeful
    pub realism: f32,       // 0.0 = fantastical, 1.0 = gritty realistic
    pub pace: String,       // "slow", "medium", "fast", "chaotic"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSoulInteraction {
    pub soul_id: String,
    pub world_id: String,
    pub active_effects: Vec<ActiveWorldEffect>,
    pub faction_standing: HashMap<String, i32>,
    pub world_specific_traits: Vec<String>,
    pub narrative_hooks_in_world: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWorldEffect {
    pub source: String,
    pub effect_description: String,
    pub mechanical_impact: String,
}

/// Integra un Alma en un Mundo Semilla
pub fn integrate_soul_into_world(soul: &LivingSoul, world: &WorldSeed) -> WorldSoulInteraction {
    let mut effects: Vec<ActiveWorldEffect> = Vec::new();
    let mut faction_standing: HashMap<String, i32> = HashMap::new();
    let mut world_traits: Vec<String> = Vec::new();
    let mut hooks: Vec<String> = Vec::new();
    
    // === APLICAR LEYES DEL MUNDO ===
    for law in &world.world_laws {
        if law.affects_archetypes.iter()
            .any(|a| soul.archetype.to_lowercase().contains(&a.to_lowercase()))
            || law.affects_archetypes.contains(&"*".to_string()) {
            
            let effect_desc = match &law.effect {
                LawEffect::Buff { stat, modifier } => {
                    format!("✨ Bonificación: {} +{:.0}%", stat, modifier * 100.0)
                },
                LawEffect::Debuff { stat, modifier } => {
                    format!("⚠️ Penalización: {} -{:.0}%", stat, modifier * 100.0)
                },
                LawEffect::Restriction { action } => {
                    format!("🚫 Restricción: {} está prohibido/limitado", action)
                },
                LawEffect::Enhancement { ability } => {
                    format!("⚡ Mejora: {} potenciado", ability)
                },
                LawEffect::SocialModifier { reputation_change } => {
                    let direction = if *reputation_change > 0 { "mejorada" } else { "dañada" };
                    format!("👥 Reputación {} por ley del mundo", direction)
                },
            };
            
            effects.push(ActiveWorldEffect {
                source: format!("Ley: {}", law.name),
                effect_description: law.description.clone(),
                mechanical_impact: effect_desc,
            });
        }
    }
    
    // === CALCULAR AFINIDAD CON FACCIONES ===
    for faction in &world.factions {
        let mut standing = 0i32;
        
        // Check if archetype is accepted
        if faction.accepts_archetypes.iter()
            .any(|a| soul.archetype.to_lowercase().contains(&a.to_lowercase())) {
            standing += 20;
        }
        
        // Check if archetype is rejected
        if faction.rejects_archetypes.iter()
            .any(|a| soul.archetype.to_lowercase().contains(&a.to_lowercase())) {
            standing -= 30;
        }
        
        // Check moral alignment
        for code in &soul.psyche.moral_code {
            if faction.alignment.to_lowercase().contains("luz") 
                && code.to_lowercase().contains("proteger") {
                standing += 10;
            }
            if faction.alignment.to_lowercase().contains("oscuro") 
                && code.to_lowercase().contains("proteger") {
                standing -= 10;
            }
        }
        
        faction_standing.insert(faction.name.clone(), standing);
        
        // Generar hook si hay tensión
        if standing < -10 {
            hooks.push(format!(
                "La facción '{}' ve a {} con hostilidad. \
                Su mera presencia genera conflicto.",
                faction.name, soul.name
            ));
        } else if standing > 20 {
            hooks.push(format!(
                "La facción '{}' podría ofrecer refugio o alianza a {}.",
                faction.name, soul.name
            ));
        }
    }
    
    // === APLICAR CONFLICTOS GLOBALES ===
    for conflict in &world.active_conflicts {
        hooks.push(format!(
            "El conflicto '{}' afecta a {}: {}",
            conflict.name, soul.name, conflict.how_souls_are_affected
        ));
    }
    
    // === ADAPTAR AL TONO NARRATIVO ===
    if world.narrative_tone.darkness > 0.7 {
        world_traits.push("Las cicatrices pesan más aquí".to_string());
        if !soul.scars.is_empty() {
            effects.push(ActiveWorldEffect {
                source: "Tono Oscuro del Mundo".to_string(),
                effect_description: "En este mundo, los traumas resuenan con más fuerza".to_string(),
                mechanical_impact: "Cicatrices tienen +20% peso narrativo".to_string(),
            });
        }
    }
    
    if world.narrative_tone.hope > 0.7 {
        world_traits.push("La redención es posible aquí".to_string());
        effects.push(ActiveWorldEffect {
            source: "Tono Esperanzador del Mundo".to_string(),
            effect_description: "Hay luz incluso en la oscuridad".to_string(),
            mechanical_impact: "Cicatrices pueden sanarse con más facilidad".to_string(),
        });
    }
    
    // === CRUZAR PROFECÍAS CON MUNDO ===
    for prophecy in &soul.prophecies {
        if !prophecy.fulfilled {
            hooks.push(format!(
                "La profecía '{}' podría manifestarse de forma única en {}.",
                prophecy.text, world.name
            ));
        }
    }
    
    // === NÉMESIS EN NUEVO MUNDO ===
    if let Some(ref nemesis) = soul.nemesis {
        hooks.push(format!(
            "¿Existe {} en {}? ¿O encontrará un nuevo enemigo que encarne el mismo conflicto?",
            nemesis.name, world.name
        ));
    }
    
    WorldSoulInteraction {
        soul_id: soul.name.clone(),
        world_id: world.id.clone(),
        active_effects: effects,
        faction_standing,
        world_specific_traits: world_traits,
        narrative_hooks_in_world: hooks,
    }
}

/// Parser de Mundo desde HTML
pub fn parse_world_seed(html: &str) -> WorldSeed {
    use scraper::{Html, Selector};
    
    let doc = Html::parse_document(html);
    
    // Selectores para mundo
    let name_sel = Selector::parse("[data-world-name], .world-name").unwrap();
    let law_sel = Selector::parse("[data-world-law], .world-law").unwrap();
    let faction_sel = Selector::parse("[data-faction], .world-faction").unwrap();
    
    let name = doc.select(&name_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_else(|| "Mundo Sin Nombre".to_string());
    
    let mut laws = Vec::new();
    for (idx, el) in doc.select(&law_sel).enumerate() {
        laws.push(WorldLaw {
            id: format!("law_{}", idx),
            name: el.value().attr("data-law-name")
                .unwrap_or("Ley Anónima")
                .to_string(),
            description: el.text().collect::<String>().trim().to_string(),
            affects_archetypes: el.value().attr("data-affects")
                .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
                .unwrap_or_else(|| vec!["*".to_string()]),
            effect: parse_law_effect(el.value().attr("data-effect")),
        });
    }
    
    let mut factions = Vec::new();
    for el in doc.select(&faction_sel) {
        factions.push(Faction {
            name: el.value().attr("data-faction-name")
                .unwrap_or("Facción")
                .to_string(),
            alignment: el.value().attr("data-alignment")
                .unwrap_or("neutral")
                .to_string(),
            accepts_archetypes: el.value().attr("data-accepts")
                .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
                .unwrap_or_default(),
            rejects_archetypes: el.value().attr("data-rejects")
                .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
                .unwrap_or_default(),
            benefits: vec![],
            obligations: vec![],
        });
    }
    
    WorldSeed {
        id: format!("world_{}", name.to_lowercase().replace(' ', "_")),
        name,
        creator: "Unknown".to_string(),
        world_laws: laws,
        factions,
        active_conflicts: vec![],
        environmental_rules: EnvironmentalRules {
            magic_level: MagicLevel::Medium,
            technology_level: TechLevel::Medieval,
            danger_level: 0.5,
            special_conditions: vec![],
        },
        narrative_tone: NarrativeTone {
            darkness: 0.5,
            hope: 0.5,
            realism: 0.5,
            pace: "medium".to_string(),
        },
    }
}

fn parse_law_effect(effect_str: Option<&str>) -> LawEffect {
    match effect_str {
        Some(s) if s.starts_with("buff:") => {
            let parts: Vec<&str> = s[5..].split(':').collect();
            LawEffect::Buff {
                stat: parts.get(0).unwrap_or(&"unknown").to_string(),
                modifier: parts.get(1).and_then(|x| x.parse().ok()).unwrap_or(0.1),
            }
        },
        Some(s) if s.starts_with("debuff:") => {
            let parts: Vec<&str> = s[7..].split(':').collect();
            LawEffect::Debuff {
                stat: parts.get(0).unwrap_or(&"unknown").to_string(),
                modifier: parts.get(1).and_then(|x| x.parse().ok()).unwrap_or(0.1),
            }
        },
        Some(s) if s.starts_with("restrict:") => {
            LawEffect::Restriction { action: s[9..].to_string() }
        },
        _ => LawEffect::SocialModifier { reputation_change: 0 },
    }
}
