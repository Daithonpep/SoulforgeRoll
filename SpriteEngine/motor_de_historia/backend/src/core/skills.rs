//! Sistema de Habilidades Procedurales SoulForge
//! 
//! Genera habilidades únicas basadas en:
//! - Tier del Alma (Eco → Mítica) - Determina cantidad y rareza
//! - Edad del personaje - Determina poder y experiencia
//! - Clase/Trabajo - Define el árbol de habilidades
//! - Historia/Traumas - Genera pasivas únicas (Habilidades de Firma)
//!
//! Ver SKILL_SYSTEM_DESIGN.txt para documentación completa.

use rand::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================
// TIER DEL ALMA
// ============================================================

/// Tier del Alma - Sistema de rareza del personaje
/// Afecta la cantidad y calidad de las habilidades
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoulTier {
    Eco,        // Común: 1 habilidad básica
    Murmullo,   // Poco común: 2 habilidades
    Sombra,     // Raro: 3 habilidades + pasiva
    Voz,        // Épico: 3 habilidades + social
    Alma,       // Legendario: 4 habilidades + mística
    Ancestral,  // Mítico: 4 habilidades + legado
    Primordial, // Ultra-raro: 5 habilidades + elemental
    Legendaria, // Extremo: 6 habilidades + inspiración
    Mitica,     // Divino: 8 habilidades máximo
}

impl SoulTier {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "eco" => SoulTier::Eco,
            "murmullo" => SoulTier::Murmullo,
            "sombra" => SoulTier::Sombra,
            "voz" => SoulTier::Voz,
            "alma" => SoulTier::Alma,
            "ancestral" => SoulTier::Ancestral,
            "primordial" => SoulTier::Primordial,
            "legendaria" => SoulTier::Legendaria,
            "mitica" | "mítica" => SoulTier::Mitica,
            _ => SoulTier::Eco,
        }
    }
    
    /// Número base de habilidades por tier
    pub fn skill_slots(&self) -> usize {
        match self {
            SoulTier::Eco => 1,
            SoulTier::Murmullo => 2,
            SoulTier::Sombra => 3,
            SoulTier::Voz => 3,
            SoulTier::Alma => 4,
            SoulTier::Ancestral => 4,
            SoulTier::Primordial => 5,
            SoulTier::Legendaria => 6,
            SoulTier::Mitica => 8,
        }
    }
    
    /// ¿Desbloquea habilidades de firma basadas en trauma?
    pub fn unlocks_signature(&self) -> bool {
        matches!(self, SoulTier::Sombra | SoulTier::Voz | SoulTier::Alma | 
                       SoulTier::Ancestral | SoulTier::Primordial | 
                       SoulTier::Legendaria | SoulTier::Mitica)
    }
    
    /// ¿Desbloquea habilidades ultimate?
    pub fn unlocks_ultimate(&self) -> bool {
        matches!(self, SoulTier::Alma | SoulTier::Ancestral | 
                       SoulTier::Primordial | SoulTier::Legendaria | SoulTier::Mitica)
    }
    
    /// Rango de poder base para este Tier (SOLO TIER determina poder)
    /// Sistema Gacha: Tier es la ÚNICA variable que afecta poder
    pub fn power_range(&self) -> (u8, u8) {
        match self {
            SoulTier::Eco => (3, 4),        // Común
            SoulTier::Murmullo => (4, 5),   // Poco común
            SoulTier::Sombra => (5, 6),     // Raro
            SoulTier::Voz => (5, 7),        // Épico
            SoulTier::Alma => (6, 8),       // Legendario
            SoulTier::Ancestral => (7, 8),  // Mítico
            SoulTier::Primordial => (7, 9), // Ultra-raro
            SoulTier::Legendaria => (8, 9), // Extremo
            SoulTier::Mitica => (9, 10),    // Divino
        }
    }
    
    /// Color del tier para UI
    pub fn color(&self) -> &'static str {
        match self {
            SoulTier::Eco => "#808080",
            SoulTier::Murmullo => "#FFFFFF",
            SoulTier::Sombra => "#32CD32",
            SoulTier::Voz => "#4169E1",
            SoulTier::Alma => "#9932CC",
            SoulTier::Ancestral => "#FFD700",
            SoulTier::Primordial => "#FF4500",
            SoulTier::Legendaria => "#FF1493",
            SoulTier::Mitica => "#00FFFF",
        }
    }
    
    /// Nombre en español
    pub fn name(&self) -> &'static str {
        match self {
            SoulTier::Eco => "Eco",
            SoulTier::Murmullo => "Murmullo",
            SoulTier::Sombra => "Sombra",
            SoulTier::Voz => "Voz",
            SoulTier::Alma => "Alma",
            SoulTier::Ancestral => "Ancestral",
            SoulTier::Primordial => "Primordial",
            SoulTier::Legendaria => "Legendaria",
            SoulTier::Mitica => "Mítica",
        }
    }
}

// ============================================================
// EDAD (Solo afecta NARRATIVA, NO poder de habilidades)
// ============================================================

/// Devuelve una descripción del rango de edad para narrativas
/// NOTA: La edad NO afecta el poder de las habilidades (sistema Gacha)
/// Solo se usa para generar texto narrativo de unlock_reason
pub fn age_description(age: u32) -> &'static str {
    match age {
        0..=20 => "joven inexperto",
        21..=30 => "adulto joven",
        31..=45 => "en su prime",
        46..=60 => "veterano experimentado",
        61..=80 => "maestro",
        _ => "anciano sabio",
    }
}

// ============================================================
// CATEGORÍAS DE HABILIDAD
// ============================================================

/// Categoría de habilidad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillCategory {
    Active,      // Requiere activación consciente
    Passive,     // Siempre activa automáticamente  
    Ultimate,    // Poderosa, uso limitado
    Reaction,    // Se activa en respuesta a eventos
    Signature,   // Única del personaje (basada en su trauma/historia)
}

impl SkillCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            SkillCategory::Active => "⚔️",
            SkillCategory::Passive => "🛡️",
            SkillCategory::Ultimate => "⭐",
            SkillCategory::Reaction => "⚡",
            SkillCategory::Signature => "💎",
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            SkillCategory::Active => "Activa",
            SkillCategory::Passive => "Pasiva",
            SkillCategory::Ultimate => "Ultimate",
            SkillCategory::Reaction => "Reacción",
            SkillCategory::Signature => "Firma",
        }
    }
    
    /// Rango de poder base para esta categoría
    pub fn base_power_range(&self) -> (u8, u8) {
        match self {
            SkillCategory::Active => (3, 6),
            SkillCategory::Passive => (2, 5),
            SkillCategory::Ultimate => (6, 8),
            SkillCategory::Reaction => (3, 5),
            SkillCategory::Signature => (5, 7),
        }
    }
}

// ============================================================
// ESTRUCTURA DE HABILIDAD
// ============================================================

/// Una habilidad generada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub name_en: String,
    pub description: String,
    pub description_en: String,
    pub category: SkillCategory,
    pub power_level: u8,           // 1-10
    pub cooldown: Option<String>,  // "1 turno", "1 vez por combate", etc.
    pub cost: Option<String>,      // "10 MP", "1 punto de tensión", etc.
    pub unlock_reason: String,     // Por qué tiene esta habilidad (narrativo)
    pub dice_formula: Option<String>, // Para futuro sistema de dados: "d20+3", "2d6", etc.
}

impl Skill {
    /// Calcula el bonus para tiradas de d20 basado en poder
    pub fn dice_bonus(&self) -> i32 {
        (self.power_level as i32) / 2
    }
    
    /// Calcula el costo de MP basado en poder y categoría
    pub fn calculate_mp_cost(&self) -> u32 {
        match self.category {
            SkillCategory::Active => 5 + (self.power_level as u32 * 2),
            SkillCategory::Ultimate => 15 + (self.power_level as u32 * 4),
            _ => 0,
        }
    }
}

// ============================================================
// GENERADOR DE HABILIDADES
// ============================================================

/// Generador de habilidades basado en el sistema SoulForge
pub struct SkillForge;

impl SkillForge {
    /// Genera un conjunto de habilidades para un personaje
    /// 
    /// # Parámetros
    /// - `class`: Clase del personaje (Mago, Guerrero, Paladín, etc.)
    /// - `tier`: Tier del alma (GACHA - determina poder y cantidad)
    /// - `age`: Edad del personaje (solo afecta narrativa, NO poder)
    /// - `trauma`: Herida psicológica (opcional, genera habilidad de firma)
    pub fn generate<R: Rng>(
        rng: &mut R,
        class: &str,
        tier: SoulTier,
        age: u32,
        trauma: Option<&str>,
    ) -> Vec<Skill> {
        let mut skills = Vec::new();
        let slot_count = tier.skill_slots();
        let (tier_min, tier_max) = tier.power_range(); // Poder determinado SOLO por Tier
        
        // Obtener pool de habilidades según clase
        let class_pool = Self::get_class_pool(class);
        
        // Filtrar según tier (no dar ultimates a tiers bajos)
        let available: Vec<_> = class_pool.iter()
            .filter(|(_, _, _, _, cat)| {
                match cat {
                    SkillCategory::Ultimate => tier.unlocks_ultimate(),
                    _ => true,
                }
            })
            .cloned()
            .collect();
        
        // Seleccionar habilidades
        let selected: Vec<_> = available.choose_multiple(rng, slot_count.min(available.len())).cloned().collect();
        
        for (name, name_en, desc, desc_en, category) in selected {
            // Poder determinado directamente por el Tier (Sistema Gacha)
            // La edad NO afecta el poder
            let power = rng.gen_range(tier_min..=tier_max);
            
            let cooldown = Self::generate_cooldown(rng, &category);
            let cost = Self::generate_cost(&category, power);
            let unlock_reason = Self::generate_unlock_reason(rng, tier, age);
            let dice_formula = Self::generate_dice_formula(&category, power);
            
            skills.push(Skill {
                name: name.to_string(),
                name_en: name_en.to_string(),
                description: desc.to_string(),
                description_en: desc_en.to_string(),
                category,
                power_level: power,
                cooldown,
                cost,
                unlock_reason,
                dice_formula,
            });
        }
        
        // Añadir habilidad de Firma si el tier lo permite y hay trauma
        if tier.unlocks_signature() {
            if let Some(trauma) = trauma {
                skills.push(Self::generate_signature_skill(rng, trauma, tier));
            }
        }
        
        skills
    }
    
    // ============================================================
    // POOLS DE CLASE
    // ============================================================
    
    fn get_class_pool(class: &str) -> Vec<(&'static str, &'static str, &'static str, &'static str, SkillCategory)> {
        match class.to_lowercase().as_str() {
            "mago" | "wizard" | "hechicero" | "sorcerer" | "brujo" | "warlock" => vec![
                ("Rayo Arcano", "Arcane Bolt", "Dispara un proyectil de energía mística", "Fires a bolt of mystic energy", SkillCategory::Active),
                ("Escudo Místico", "Mystic Shield", "Crea una barrera protectora temporal", "Creates a temporary protective barrier", SkillCategory::Reaction),
                ("Meditación Profunda", "Deep Meditation", "Recupera maná lentamente", "Slowly recovers mana", SkillCategory::Passive),
                ("Visión Arcana", "Arcane Sight", "Detecta magia y auras ocultas", "Detects magic and hidden auras", SkillCategory::Passive),
                ("Tormenta de Fuego", "Firestorm", "Invoca llamas devastadoras en área", "Summons devastating flames in area", SkillCategory::Ultimate),
                ("Telekinesis", "Telekinesis", "Mueve objetos con la mente", "Moves objects with the mind", SkillCategory::Active),
                ("Canalizar Elemento", "Channel Element", "Infunde arma con poder elemental", "Infuses weapon with elemental power", SkillCategory::Active),
                ("Contraconjuro", "Counterspell", "Anula un hechizo enemigo", "Nullifies an enemy spell", SkillCategory::Reaction),
                ("Distorsión Temporal", "Time Warp", "Ralentiza el tiempo brevemente", "Briefly slows down time", SkillCategory::Ultimate),
                ("Catástrofe Arcana", "Arcane Catastrophe", "Libera toda la energía acumulada", "Releases all accumulated energy", SkillCategory::Ultimate),
            ],
            
            "guerrero" | "warrior" | "fighter" | "bárbaro" | "barbarian" => vec![
                ("Golpe Devastador", "Devastating Strike", "Ataque que rompe defensas", "Attack that breaks defenses", SkillCategory::Active),
                ("Postura Defensiva", "Defensive Stance", "Reduce daño mientras se mantiene", "Reduces damage while maintained", SkillCategory::Active),
                ("Voluntad de Hierro", "Iron Will", "Resistencia al miedo y control", "Resistance to fear and control", SkillCategory::Passive),
                ("Carga Brutal", "Brutal Charge", "Embiste derribando al enemigo", "Charges knocking down the enemy", SkillCategory::Active),
                ("Segundo Aliento", "Second Wind", "Recupera vitalidad en momento crítico", "Recovers vitality at critical moment", SkillCategory::Reaction),
                ("Grito de Guerra", "War Cry", "Intimida enemigos, fortalece aliados", "Intimidates enemies, strengthens allies", SkillCategory::Active),
                ("Maestría con Armas", "Weapon Mastery", "Bonus pasivo con armas preferidas", "Passive bonus with preferred weapons", SkillCategory::Passive),
                ("Ejecución", "Execution", "Daño masivo a enemigos debilitados", "Massive damage to weakened enemies", SkillCategory::Ultimate),
                ("Furia Imparable", "Unstoppable Fury", "Entra en frenesí de combate", "Enters combat frenzy", SkillCategory::Ultimate),
                ("Último Bastión", "Last Bastion", "Se niega a caer por un turno", "Refuses to fall for one turn", SkillCategory::Ultimate),
            ],
            
            "paladin" | "paladín" | "cruzado" | "crusader" | "templario" => vec![
                ("Imposición de Manos", "Lay on Hands", "Cura heridas con energía divina", "Heals wounds with divine energy", SkillCategory::Active),
                ("Aura Sagrada", "Sacred Aura", "Protege aliados cercanos", "Protects nearby allies", SkillCategory::Passive),
                ("Castigo Divino", "Divine Smite", "Infunde arma con poder sagrado", "Infuses weapon with holy power", SkillCategory::Active),
                ("Escudo de Fe", "Shield of Faith", "Barrera contra ataques oscuros", "Barrier against dark attacks", SkillCategory::Reaction),
                ("Detectar Mal", "Detect Evil", "Siente presencias malignas", "Senses evil presences", SkillCategory::Passive),
                ("Luz Purificadora", "Purifying Light", "Limpia maldiciones y venenos", "Cleanses curses and poisons", SkillCategory::Active),
                ("Mártir", "Martyr", "Recibe daño destinado a aliados", "Takes damage meant for allies", SkillCategory::Reaction),
                ("Juicio Final", "Final Judgment", "Ataque devastador contra el mal", "Devastating attack against evil", SkillCategory::Ultimate),
                ("Avatar de la Luz", "Avatar of Light", "Transforma en ser de luz pura", "Transforms into being of pure light", SkillCategory::Ultimate),
                ("Resurrección", "Resurrection", "Revive a un aliado caído", "Revives a fallen ally", SkillCategory::Ultimate),
            ],
            
            "picaro" | "pícaro" | "rogue" | "ladrón" | "asesino" | "thief" | "assassin" => vec![
                ("Ataque Furtivo", "Sneak Attack", "Daño extra desde las sombras", "Extra damage from shadows", SkillCategory::Active),
                ("Evasión", "Evasion", "Esquiva ataques de área", "Dodges area attacks", SkillCategory::Passive),
                ("Veneno Mortal", "Deadly Poison", "Aplica toxina a las armas", "Applies toxin to weapons", SkillCategory::Active),
                ("Paso de Sombra", "Shadow Step", "Teletransporte corto a sombras", "Short teleport to shadows", SkillCategory::Active),
                ("Instinto del Cazador", "Hunter's Instinct", "Detecta trampas y enemigos", "Detects traps and enemies", SkillCategory::Passive),
                ("Escapar", "Vanish", "Desaparece brevemente", "Disappears briefly", SkillCategory::Reaction),
                ("Reflejos Felinos", "Cat-like Reflexes", "Bonus a iniciativa y esquiva", "Bonus to initiative and dodge", SkillCategory::Passive),
                ("Golpe Letal", "Lethal Strike", "Crítico garantizado desde sigilo", "Guaranteed critical from stealth", SkillCategory::Ultimate),
                ("Maestro de Sombras", "Shadow Master", "Control total sobre oscuridad", "Total control over darkness", SkillCategory::Ultimate),
                ("Muerte Súbita", "Sudden Death", "Ejecuta enemigos debilitados", "Executes weakened enemies", SkillCategory::Ultimate),
            ],
            
            "clerigo" | "clérigo" | "cleric" | "sacerdote" | "priest" => vec![
                ("Curación Mayor", "Greater Heal", "Restaura salud significativa", "Restores significant health", SkillCategory::Active),
                ("Bendición", "Blessing", "Mejora estadísticas de aliados", "Improves ally statistics", SkillCategory::Active),
                ("Expulsar Muertos", "Turn Undead", "Daña o ahuyenta no-muertos", "Damages or repels undead", SkillCategory::Active),
                ("Escudo Sagrado", "Holy Shield", "Protección divina temporal", "Temporary divine protection", SkillCategory::Reaction),
                ("Comunión Divina", "Divine Communion", "Recupera maná mediante oración", "Recovers mana through prayer", SkillCategory::Passive),
                ("Santuario", "Sanctuary", "Crea zona segura temporal", "Creates temporary safe zone", SkillCategory::Active),
                ("Fe Inquebrantable", "Unwavering Faith", "Resistencia a efectos oscuros", "Resistance to dark effects", SkillCategory::Passive),
                ("Milagro", "Miracle", "Invoca intervención divina", "Invokes divine intervention", SkillCategory::Ultimate),
                ("Lluvia de Luz", "Rain of Light", "Sanación masiva en área", "Mass healing in area", SkillCategory::Ultimate),
                ("Resurreción", "Resurrection", "Devuelve vida a un caído", "Returns life to the fallen", SkillCategory::Ultimate),
            ],
            
            "ranger" | "arquero" | "cazador" | "hunter" | "explorador" => vec![
                ("Tiro Certero", "Precise Shot", "Disparo con bonus a precisión", "Shot with accuracy bonus", SkillCategory::Active),
                ("Rastrear", "Track", "Sigue el rastro de cualquier criatura", "Follows the trail of any creature", SkillCategory::Active),
                ("Camuflaje Natural", "Natural Camouflage", "Bonus a sigilo en naturaleza", "Stealth bonus in nature", SkillCategory::Passive),
                ("Compañero Animal", "Animal Companion", "Vínculo con criatura aliada", "Bond with allied creature", SkillCategory::Passive),
                ("Lluvia de Flechas", "Arrow Rain", "Disparo múltiple en área", "Multiple shots in area", SkillCategory::Active),
                ("Esquivar Peligro", "Danger Sense", "Reacción ante emboscadas", "Reaction to ambushes", SkillCategory::Reaction),
                ("Conocimiento del Terreno", "Terrain Knowledge", "Ventaja en terreno conocido", "Advantage in known terrain", SkillCategory::Passive),
                ("Tiro Imposible", "Impossible Shot", "Disparo que ignora cobertura", "Shot that ignores cover", SkillCategory::Ultimate),
                ("Uno con la Naturaleza", "One with Nature", "Control sobre elementos naturales", "Control over natural elements", SkillCategory::Ultimate),
                ("Depredador Supremo", "Supreme Predator", "Caza implacable al objetivo", "Relentless hunt of target", SkillCategory::Ultimate),
            ],
            
            "bardo" | "bard" | "trovador" => vec![
                ("Inspirar Coraje", "Inspire Courage", "Aliados ganan bonus a ataques", "Allies gain attack bonus", SkillCategory::Active),
                ("Canción de Curación", "Healing Song", "Restaura HP lentamente a aliados", "Slowly restores ally HP", SkillCategory::Active),
                ("Conocimiento Arcano", "Arcane Knowledge", "Bonus a identificar magia", "Bonus to identify magic", SkillCategory::Passive),
                ("Fascinación", "Fascinate", "Distrae a enemigos con música", "Distracts enemies with music", SkillCategory::Active),
                ("Contramelodía", "Countersong", "Anula efectos sonoros mágicos", "Cancels magical sound effects", SkillCategory::Reaction),
                ("Saber Popular", "Folk Knowledge", "Conoce leyendas y secretos", "Knows legends and secrets", SkillCategory::Passive),
                ("Himno de Guerra", "Battle Hymn", "Potencia masiva a aliados", "Massive ally empowerment", SkillCategory::Ultimate),
                ("Palabra de Muerte", "Death Word", "Una nota que puede matar", "A note that can kill", SkillCategory::Ultimate),
                ("Canto del Destino", "Song of Destiny", "Altera la suerte de todos", "Alters everyone's luck", SkillCategory::Ultimate),
            ],
            
            "monje" | "monk" | "artista marcial" => vec![
                ("Golpe de Ki", "Ki Strike", "Ataque cargado con energía vital", "Attack charged with vital energy", SkillCategory::Active),
                ("Palma Sanadora", "Healing Palm", "Cura mediante puntos de presión", "Heals through pressure points", SkillCategory::Active),
                ("Equilibrio Perfecto", "Perfect Balance", "Inmune a derribo y empuje", "Immune to knockdown and push", SkillCategory::Passive),
                ("Velocidad del Viento", "Wind Speed", "Movimiento extra en combate", "Extra movement in combat", SkillCategory::Passive),
                ("Desviar Proyectil", "Deflect Missile", "Atrapa o desvía flechas", "Catches or deflects arrows", SkillCategory::Reaction),
                ("Caídas Suaves", "Slow Fall", "Reduce daño por caídas", "Reduces fall damage", SkillCategory::Passive),
                ("Toque Paralizante", "Stunning Fist", "Paraliza al enemigo brevemente", "Briefly paralyzes enemy", SkillCategory::Ultimate),
                ("Mil Puños", "Thousand Fists", "Ráfaga devastadora de golpes", "Devastating flurry of blows", SkillCategory::Ultimate),
                ("Tranquilidad", "Tranquility", "Invulnerabilidad momentánea", "Momentary invulnerability", SkillCategory::Ultimate),
            ],
            
            // Clase genérica para personajes sin clase definida
            _ => vec![
                ("Ataque Básico", "Basic Attack", "Un ataque simple pero efectivo", "A simple but effective attack", SkillCategory::Active),
                ("Esquivar", "Dodge", "Evita un ataque entrante", "Avoids an incoming attack", SkillCategory::Reaction),
                ("Resistencia Natural", "Natural Resistance", "Aguanta más de lo normal", "Endures more than normal", SkillCategory::Passive),
                ("Determinación", "Determination", "Se niega a rendirse", "Refuses to give up", SkillCategory::Passive),
                ("Golpe Crítico", "Critical Strike", "Ataque con daño aumentado", "Attack with increased damage", SkillCategory::Active),
                ("Último Esfuerzo", "Last Effort", "Poder desesperado cuando herido", "Desperate power when wounded", SkillCategory::Ultimate),
            ],
        }
    }
    
    // ============================================================
    // GENERADORES AUXILIARES
    // ============================================================
    
    fn generate_cooldown<R: Rng>(rng: &mut R, category: &SkillCategory) -> Option<String> {
        match category {
            SkillCategory::Active => {
                let cooldowns = ["1 turno", "2 turnos", "3 turnos"];
                Some(cooldowns.choose(rng).unwrap().to_string())
            },
            SkillCategory::Ultimate => Some("1 vez por combate".to_string()),
            SkillCategory::Reaction => Some("1 vez por turno".to_string()),
            _ => None,
        }
    }
    
    fn generate_cost(category: &SkillCategory, power: u8) -> Option<String> {
        match category {
            SkillCategory::Active => Some(format!("{} MP", 5 + (power as u16 * 2))),
            SkillCategory::Ultimate => Some(format!("{} MP + 1 Tensión", 15 + (power as u16 * 4))),
            _ => None,
        }
    }
    
    fn generate_dice_formula(category: &SkillCategory, power: u8) -> Option<String> {
        let bonus = power / 2;
        match category {
            SkillCategory::Active => Some(format!("d20+{}", bonus)),
            SkillCategory::Passive => Some(format!("+{}", (power / 3) + 1)),
            SkillCategory::Ultimate => Some(format!("{}d6", power)),
            SkillCategory::Reaction => Some(format!("d20+{}", bonus)),
            SkillCategory::Signature => Some(format!("+{}", (power / 2) + 1)),
        }
    }
    
    fn generate_unlock_reason<R: Rng>(rng: &mut R, tier: SoulTier, age: u32) -> String {
        let tier_reasons = match tier {
            SoulTier::Eco => vec!["Instinto básico de supervivencia", "Aprendida por necesidad"],
            SoulTier::Murmullo => vec!["Perfeccionada con práctica", "El viento susurró el secreto"],
            SoulTier::Sombra => vec!["Forjada en las sombras", "Revelada en la oscuridad"],
            SoulTier::Voz => vec!["Las palabras tienen poder", "Dominada con carisma"],
            SoulTier::Alma => vec!["El destino la otorgó", "Resonancia del alma"],
            SoulTier::Ancestral => vec!["Herencia de sangre antigua", "Los ancestros guían la mano"],
            SoulTier::Primordial => vec!["Poder de eras olvidadas", "Fuerza primigenia"],
            SoulTier::Legendaria => vec!["Los bardos cantan de ella", "Técnica de leyenda"],
            SoulTier::Mitica => vec!["Trasciende lo mortal", "Poder divino canalizado"],
        };
        
        let age_suffix = if age > 60 {
            " (refinada por décadas de experiencia)"
        } else if age > 45 {
            " (madurada con los años)"
        } else if age < 25 {
            " (talento joven prometedor)"
        } else {
            ""
        };
        
        format!("{}{}", tier_reasons.choose(rng).unwrap_or(&"Adquirida con el tiempo"), age_suffix)
    }
    
    fn generate_signature_skill<R: Rng>(rng: &mut R, trauma: &str, tier: SoulTier) -> Skill {
        let trauma_lower = trauma.to_lowercase();
        
        let (name, name_en, desc, desc_en) = if trauma_lower.contains("abandono") || trauma_lower.contains("soledad") {
            ("Vínculo Irrompible", "Unbreakable Bond", 
             "La soledad te enseñó el valor de las conexiones. +3 al proteger aliados.",
             "Loneliness taught you the value of connections. +3 when protecting allies.")
        } else if trauma_lower.contains("traición") {
            ("Ojo Vigilante", "Watchful Eye",
             "La traición afiló tus sentidos. Detectas engaños automáticamente.",
             "Betrayal sharpened your senses. Automatically detect deception.")
        } else if trauma_lower.contains("pérdida") || trauma_lower.contains("muerte") {
            ("Fuerza del Duelo", "Strength of Grief",
             "El dolor te forjó. +2 poder cuando un aliado cae.",
             "Pain forged you. +2 power when an ally falls.")
        } else if trauma_lower.contains("violencia") || trauma_lower.contains("abuso") {
            ("Ira Contenida", "Contained Rage",
             "El sufrimiento se convirtió en fuerza. +4 daño cuando HP < 50%.",
             "Suffering became strength. +4 damage when HP < 50%.")
        } else if trauma_lower.contains("rechazo") || trauma_lower.contains("humillación") {
            ("Prueba Constante", "Constant Proof",
             "Debes demostrar tu valía. +2 a tiradas tras un fallo.",
             "You must prove your worth. +2 to rolls after a failure.")
        } else if trauma_lower.contains("culpa") {
            ("Redención", "Redemption",
             "Buscas expiar el pasado. +3 a acciones que ayuden a inocentes.",
             "You seek to atone. +3 to actions that help innocents.")
        } else {
            ("Marca del Destino", "Mark of Destiny",
             "Tu pasado te persigue y fortalece. Habilidad única que refleja tu historia.",
             "Your past haunts and strengthens you. Unique ability reflecting your history.")
        };
        
        // Poder de firma determinado por el Tier (Gacha)
        let (tier_min, tier_max) = tier.power_range();
        let power = rng.gen_range(tier_min..=tier_max);
        
        Skill {
            name: name.to_string(),
            name_en: name_en.to_string(),
            description: desc.to_string(),
            description_en: desc_en.to_string(),
            category: SkillCategory::Signature,
            power_level: power,
            cooldown: Some("Pasiva permanente".to_string()),
            cost: None,
            unlock_reason: format!("Forjada por trauma: \"{}\"", trauma),
            dice_formula: Some(format!("+{}", (power / 2) + 1)),
        }
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tier_skill_counts() {
        assert_eq!(SoulTier::Eco.skill_slots(), 1);
        assert_eq!(SoulTier::Sombra.skill_slots(), 3);
        assert_eq!(SoulTier::Alma.skill_slots(), 4);
        assert_eq!(SoulTier::Mitica.skill_slots(), 8);
    }
    
    #[test]
    fn test_balanced_power() {
        let mut rng = rand::thread_rng();
        
        // Eco joven: máximo ~4
        let eco_skills = SkillForge::generate(&mut rng, "Mago", SoulTier::Eco, 20, None);
        assert_eq!(eco_skills.len(), 1);
        assert!(eco_skills[0].power_level <= 5, "Eco power too high: {}", eco_skills[0].power_level);
        
        // Alma veterano: máximo ~8
        let alma_skills = SkillForge::generate(&mut rng, "Guerrero", SoulTier::Alma, 55, Some("Abandono"));
        assert_eq!(alma_skills.len(), 5); // 4 + 1 signature
        for skill in &alma_skills {
            assert!(skill.power_level <= 9, "Alma power too high: {}", skill.power_level);
        }
    }
    
    #[test]
    fn test_signature_unlocking() {
        let mut rng = rand::thread_rng();
        
        // Eco no desbloquea signature
        let eco = SkillForge::generate(&mut rng, "Mago", SoulTier::Eco, 30, Some("Trauma"));
        assert!(!eco.iter().any(|s| s.category == SkillCategory::Signature));
        
        // Sombra sí desbloquea signature
        let sombra = SkillForge::generate(&mut rng, "Mago", SoulTier::Sombra, 30, Some("Trauma"));
        assert!(sombra.iter().any(|s| s.category == SkillCategory::Signature));
    }
    
    #[test]
    fn test_ultimate_unlocking() {
        let mut rng = rand::thread_rng();
        
        // Voz no desbloquea ultimate
        for _ in 0..10 {
            let voz = SkillForge::generate(&mut rng, "Mago", SoulTier::Voz, 40, None);
            assert!(!voz.iter().any(|s| s.category == SkillCategory::Ultimate));
        }
        
        // Alma puede tener ultimate
        let mut found_ultimate = false;
        for _ in 0..20 {
            let alma = SkillForge::generate(&mut rng, "Mago", SoulTier::Alma, 40, None);
            if alma.iter().any(|s| s.category == SkillCategory::Ultimate) {
                found_ultimate = true;
                break;
            }
        }
        assert!(found_ultimate, "Alma should be able to unlock ultimates");
    }
    
    #[test]
    fn test_dice_formulas() {
        let skill = Skill {
            name: "Test".to_string(),
            name_en: "Test".to_string(),
            description: "Test".to_string(),
            description_en: "Test".to_string(),
            category: SkillCategory::Active,
            power_level: 6,
            cooldown: None,
            cost: None,
            unlock_reason: "Test".to_string(),
            dice_formula: Some("d20+3".to_string()),
        };
        
        assert_eq!(skill.dice_bonus(), 3);
        assert_eq!(skill.calculate_mp_cost(), 17);
    }
}
