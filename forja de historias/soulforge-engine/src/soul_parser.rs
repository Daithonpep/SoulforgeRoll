use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// El Alma Viva - Representación completa de un personaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingSoul {
    // === IDENTIDAD CORE ===
    pub name: String,
    pub titles: Vec<String>,
    pub archetype: String,
    
    // === PSIQUE PROFUNDA ===
    pub psyche: SoulPsyche,
    
    // === MARCAS DEL DESTINO ===
    pub scars: Vec<SoulScar>,
    pub prophecies: Vec<Prophecy>,
    pub nemesis: Option<Nemesis>,
    
    // === VÍNCULOS ===
    pub bonds: Vec<SoulBond>,
    
    // === HISTORIAL NARRATIVO ===
    pub chronicle: Vec<ChronicleEntry>,
    
    // === METADATOS DE TENSIÓN ===
    pub tension_markers: TensionMarkers,
    
    // === RAW DATA para búsquedas ===
    pub raw_attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulPsyche {
    pub core_fear: String,           // Miedo fundamental
    pub deepest_desire: String,      // Deseo más profundo
    pub fatal_flaw: String,          // Defecto fatal
    pub moral_code: Vec<String>,     // Código moral
    pub breaking_point: String,      // Punto de quiebre
    pub shadow_self: String,         // El yo sombra
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulScar {
    pub id: String,
    pub name: String,
    pub origin_event: String,
    pub trauma_type: TraumaType,
    pub healed: bool,
    pub narrative_weight: f32,       // 0.0 - 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraumaType {
    Betrayal,
    Loss,
    Failure,
    Violence,
    Abandonment,
    Corruption,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prophecy {
    pub id: String,
    pub text: String,
    pub prophecy_type: ProphecyType,
    pub fulfilled: bool,
    pub progress: f32,               // 0.0 - 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProphecyType {
    Doom,       // Destino oscuro
    Glory,      // Destino glorioso  
    Ambiguous,  // Interpretación abierta
    Choice,     // Depende de decisiones
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nemesis {
    pub name: String,
    pub relationship: String,        // "hermano", "antiguo mentor", etc.
    pub conflict_core: String,       // La raíz del conflicto
    pub threat_level: u8,            // 1-10
    pub encounters: Vec<String>,     // Historial de encuentros
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulBond {
    pub entity_name: String,
    pub bond_type: BondType,
    pub strength: f32,               // 0.0 - 1.0
    pub status: BondStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BondType {
    Love, Friendship, Rivalry, Mentorship, 
    Blood, Oath, Debt, Hatred, Complex(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BondStatus {
    Active, Strained, Broken, Dormant, Evolving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEntry {
    pub chapter: u32,
    pub event_summary: String,
    pub souls_involved: Vec<String>,
    pub consequences: Vec<String>,
    pub tension_delta: f32,          // Cambio en tensión
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionMarkers {
    pub unresolved_conflicts: u32,
    pub broken_promises: u32,
    pub pending_revenge: u32,
    pub prophecy_pressure: f32,
    pub relationship_strain: f32,
    pub overall_tension: f32,
}

// ============================================================
// PARSER PRINCIPAL
// ============================================================

pub fn parse_living_sheet(html: &str) -> LivingSoul {
    let document = Html::parse_document(html);
    let mut raw_attrs: HashMap<String, String> = HashMap::new();
    
    // === SELECTORES ADAPTATIVOS ===
    // Estos selectores buscan patrones comunes en fichas HTML
    let selectors = SelectorMatrix::new();
    
    // Extracción de datos básicos
    let name = extract_text(&document, &selectors.name)
        .unwrap_or_else(|| "Alma Sin Nombre".to_string());
    
    let titles = extract_list(&document, &selectors.titles);
    let archetype = extract_text(&document, &selectors.archetype)
        .unwrap_or_default();
    
    // === PARSEO DE PSIQUE ===
    let psyche = parse_psyche(&document, &selectors, &mut raw_attrs);
    
    // === PARSEO DE CICATRICES ===
    let scars = parse_scars(&document, &selectors);
    
    // === PARSEO DE PROFECÍAS ===
    let prophecies = parse_prophecies(&document, &selectors);
    
    // === PARSEO DE NÉMESIS ===
    let nemesis = parse_nemesis(&document, &selectors);
    
    // === PARSEO DE VÍNCULOS ===
    let bonds = parse_bonds(&document, &selectors);
    
    // === EXTRACCIÓN RAW COMPLETA ===
    extract_all_data_attributes(&document, &mut raw_attrs);
    
    LivingSoul {
        name,
        titles,
        archetype,
        psyche,
        scars,
        prophecies,
        nemesis,
        bonds,
        chronicle: Vec::new(),
        tension_markers: calculate_initial_tension(&scars, &prophecies, &bonds),
        raw_attributes: raw_attrs,
    }
}

// ============================================================
// MATRIZ DE SELECTORES ADAPTATIVA
// ============================================================

struct SelectorMatrix {
    name: Vec<Selector>,
    titles: Vec<Selector>,
    archetype: Vec<Selector>,
    fears: Vec<Selector>,
    desires: Vec<Selector>,
    flaws: Vec<Selector>,
    scars: Vec<Selector>,
    prophecies: Vec<Selector>,
    nemesis: Vec<Selector>,
    bonds: Vec<Selector>,
}

impl SelectorMatrix {
    fn new() -> Self {
        Self {
            // Múltiples selectores por campo = mayor compatibilidad
            name: vec![
                Selector::parse("[data-soul-name]").unwrap(),
                Selector::parse(".character-name").unwrap(),
                Selector::parse(".soul-name").unwrap(),
                Selector::parse("#nombre-personaje").unwrap(),
                Selector::parse("h1.name").unwrap(),
            ],
            titles: vec![
                Selector::parse("[data-titles] li").unwrap(),
                Selector::parse(".character-titles span").unwrap(),
                Selector::parse(".soul-titles > *").unwrap(),
            ],
            archetype: vec![
                Selector::parse("[data-archetype]").unwrap(),
                Selector::parse(".character-class").unwrap(),
                Selector::parse(".soul-archetype").unwrap(),
            ],
            fears: vec![
                Selector::parse("[data-fear]").unwrap(),
                Selector::parse("[data-core-fear]").unwrap(),
                Selector::parse(".character-fear").unwrap(),
                Selector::parse(".miedo-principal").unwrap(),
            ],
            desires: vec![
                Selector::parse("[data-desire]").unwrap(),
                Selector::parse("[data-deepest-desire]").unwrap(),
                Selector::parse(".character-desire").unwrap(),
            ],
            flaws: vec![
                Selector::parse("[data-flaw]").unwrap(),
                Selector::parse("[data-fatal-flaw]").unwrap(),
                Selector::parse(".character-flaw").unwrap(),
            ],
            scars: vec![
                Selector::parse("[data-scar]").unwrap(),
                Selector::parse(".soul-scar").unwrap(),
                Selector::parse(".cicatriz").unwrap(),
            ],
            prophecies: vec![
                Selector::parse("[data-prophecy]").unwrap(),
                Selector::parse(".soul-prophecy").unwrap(),
                Selector::parse(".profecia").unwrap(),
            ],
            nemesis: vec![
                Selector::parse("[data-nemesis]").unwrap(),
                Selector::parse(".soul-nemesis").unwrap(),
                Selector::parse(".nemesis-section").unwrap(),
            ],
            bonds: vec![
                Selector::parse("[data-bond]").unwrap(),
                Selector::parse(".soul-bond").unwrap(),
                Selector::parse(".vinculo").unwrap(),
            ],
        }
    }
}

// ============================================================
// FUNCIONES DE EXTRACCIÓN
// ============================================================

fn extract_text(doc: &Html, selectors: &[Selector]) -> Option<String> {
    for sel in selectors {
        if let Some(el) = doc.select(sel).next() {
            // Primero buscar data-value, luego texto interno
            if let Some(val) = el.value().attr("data-value") {
                return Some(val.trim().to_string());
            }
            let text: String = el.text().collect::<Vec<_>>().join(" ");
            let cleaned = text.trim().to_string();
            if !cleaned.is_empty() {
                return Some(cleaned);
            }
        }
    }
    None
}

fn extract_list(doc: &Html, selectors: &[Selector]) -> Vec<String> {
    for sel in selectors {
        let items: Vec<String> = doc.select(sel)
            .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !items.is_empty() {
            return items;
        }
    }
    Vec::new()
}

fn parse_psyche(doc: &Html, sel: &SelectorMatrix, raw: &mut HashMap<String, String>) -> SoulPsyche {
    let core_fear = extract_text(doc, &sel.fears).unwrap_or_default();
    let deepest_desire = extract_text(doc, &sel.desires).unwrap_or_default();
    let fatal_flaw = extract_text(doc, &sel.flaws).unwrap_or_default();
    
    raw.insert("core_fear".to_string(), core_fear.clone());
    raw.insert("deepest_desire".to_string(), deepest_desire.clone());
    raw.insert("fatal_flaw".to_string(), fatal_flaw.clone());
    
    SoulPsyche {
        core_fear,
        deepest_desire,
        fatal_flaw,
        moral_code: extract_moral_code(doc),
        breaking_point: extract_breaking_point(doc),
        shadow_self: extract_shadow_self(doc),
    }
}

fn parse_scars(doc: &Html, sel: &SelectorMatrix) -> Vec<SoulScar> {
    let mut scars = Vec::new();
    
    for selector in &sel.scars {
        for (idx, element) in doc.select(selector).enumerate() {
            let scar = SoulScar {
                id: format!("scar_{}", idx),
                name: element.value().attr("data-name")
                    .or_else(|| element.value().attr("data-scar-name"))
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Cicatriz {}", idx + 1)),
                origin_event: element.value().attr("data-origin")
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                trauma_type: parse_trauma_type(
                    element.value().attr("data-trauma-type").unwrap_or("custom")
                ),
                healed: element.value().attr("data-healed")
                    .map(|s| s == "true")
                    .unwrap_or(false),
                narrative_weight: element.value().attr("data-weight")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.5),
            };
            scars.push(scar);
        }
    }
    
    scars
}

fn parse_prophecies(doc: &Html, sel: &SelectorMatrix) -> Vec<Prophecy> {
    let mut prophecies = Vec::new();
    
    for selector in &sel.prophecies {
        for (idx, element) in doc.select(selector).enumerate() {
            let text: String = element.text().collect::<Vec<_>>().join(" ");
            
            let prophecy = Prophecy {
                id: format!("prophecy_{}", idx),
                text: text.trim().to_string(),
                prophecy_type: parse_prophecy_type(
                    element.value().attr("data-prophecy-type").unwrap_or("ambiguous")
                ),
                fulfilled: element.value().attr("data-fulfilled")
                    .map(|s| s == "true")
                    .unwrap_or(false),
                progress: element.value().attr("data-progress")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0),
            };
            prophecies.push(prophecy);
        }
    }
    
    prophecies
}

fn parse_nemesis(doc: &Html, sel: &SelectorMatrix) -> Option<Nemesis> {
    for selector in &sel.nemesis {
        if let Some(element) = doc.select(selector).next() {
            return Some(Nemesis {
                name: element.value().attr("data-name")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        element.text().collect::<Vec<_>>().join(" ").trim().to_string()
                    }),
                relationship: element.value().attr("data-relationship")
                    .unwrap_or("desconocida")
                    .to_string(),
                conflict_core: element.value().attr("data-conflict")
                    .unwrap_or("")
                    .to_string(),
                threat_level: element.value().attr("data-threat")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
                encounters: Vec::new(),
            });
        }
    }
    None
}

fn parse_bonds(doc: &Html, sel: &SelectorMatrix) -> Vec<SoulBond> {
    let mut bonds = Vec::new();
    
    for selector in &sel.bonds {
        for element in doc.select(selector) {
            let bond = SoulBond {
                entity_name: element.value().attr("data-entity")
                    .or_else(|| element.value().attr("data-name"))
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                bond_type: parse_bond_type(
                    element.value().attr("data-bond-type").unwrap_or("complex")
                ),
                strength: element.value().attr("data-strength")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.5),
                status: parse_bond_status(
                    element.value().attr("data-status").unwrap_or("active")
                ),
            };
            if !bond.entity_name.is_empty() {
                bonds.push(bond);
            }
        }
    }
    
    bonds
}

// ============================================================
// HELPERS DE PARSEO
// ============================================================

fn parse_trauma_type(s: &str) -> TraumaType {
    match s.to_lowercase().as_str() {
        "betrayal" | "traicion" => TraumaType::Betrayal,
        "loss" | "perdida" => TraumaType::Loss,
        "failure" | "fracaso" => TraumaType::Failure,
        "violence" | "violencia" => TraumaType::Violence,
        "abandonment" | "abandono" => TraumaType::Abandonment,
        "corruption" | "corrupcion" => TraumaType::Corruption,
        _ => TraumaType::Custom(s.to_string()),
    }
}

fn parse_prophecy_type(s: &str) -> ProphecyType {
    match s.to_lowercase().as_str() {
        "doom" | "condena" | "oscuro" => ProphecyType::Doom,
        "glory" | "gloria" | "luminoso" => ProphecyType::Glory,
        "choice" | "eleccion" | "decision" => ProphecyType::Choice,
        _ => ProphecyType::Ambiguous,
    }
}

fn parse_bond_type(s: &str) -> BondType {
    match s.to_lowercase().as_str() {
        "love" | "amor" => BondType::Love,
        "friendship" | "amistad" => BondType::Friendship,
        "rivalry" | "rivalidad" => BondType::Rivalry,
        "mentorship" | "mentor" => BondType::Mentorship,
        "blood" | "sangre" => BondType::Blood,
        "oath" | "juramento" => BondType::Oath,
        "debt" | "deuda" => BondType::Debt,
        "hatred" | "odio" => BondType::Hatred,
        _ => BondType::Complex(s.to_string()),
    }
}

fn parse_bond_status(s: &str) -> BondStatus {
    match s.to_lowercase().as_str() {
        "active" | "activo" => BondStatus::Active,
        "strained" | "tenso" => BondStatus::Strained,
        "broken" | "roto" => BondStatus::Broken,
        "dormant" | "dormido" => BondStatus::Dormant,
        "evolving" | "evolucionando" => BondStatus::Evolving,
        _ => BondStatus::Active,
    }
}

fn extract_moral_code(doc: &Html) -> Vec<String> {
    let selectors = vec![
        Selector::parse("[data-moral-code] li").unwrap(),
        Selector::parse(".moral-code li").unwrap(),
        Selector::parse(".codigo-moral li").unwrap(),
    ];
    for sel in selectors {
        let items: Vec<String> = doc.select(&sel)
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !items.is_empty() {
            return items;
        }
    }
    Vec::new()
}

fn extract_breaking_point(doc: &Html) -> String {
    let selectors = vec![
        Selector::parse("[data-breaking-point]").unwrap(),
        Selector::parse(".breaking-point").unwrap(),
        Selector::parse(".punto-quiebre").unwrap(),
    ];
    for sel in selectors {
        if let Some(el) = doc.select(&sel).next() {
            let text = el.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                return text.trim().to_string();
            }
        }
    }
    String::new()
}

fn extract_shadow_self(doc: &Html) -> String {
    let selectors = vec![
        Selector::parse("[data-shadow-self]").unwrap(),
        Selector::parse(".shadow-self").unwrap(),
        Selector::parse(".yo-sombra").unwrap(),
    ];
    for sel in selectors {
        if let Some(el) = doc.select(&sel).next() {
            let text = el.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                return text.trim().to_string();
            }
        }
    }
    String::new()
}

fn extract_all_data_attributes(doc: &Html, raw: &mut HashMap<String, String>) {
    let universal = Selector::parse("[data-soul-attr]").unwrap();
    for el in doc.select(&universal) {
        if let (Some(key), Some(val)) = (
            el.value().attr("data-soul-attr"),
            el.value().attr("data-value")
        ) {
            raw.insert(key.to_string(), val.to_string());
        }
    }
}

fn calculate_initial_tension(
    scars: &[SoulScar],
    prophecies: &[Prophecy],
    bonds: &[SoulBond]
) -> TensionMarkers {
    let unresolved = scars.iter().filter(|s| !s.healed).count() as u32;
    let prophecy_pressure: f32 = prophecies.iter()
        .filter(|p| !p.fulfilled)
        .map(|p| match p.prophecy_type {
            ProphecyType::Doom => 0.8,
            ProphecyType::Choice => 0.5,
            _ => 0.3,
        })
        .sum();
    
    let relationship_strain: f32 = bonds.iter()
        .map(|b| match b.status {
            BondStatus::Strained => 0.4,
            BondStatus::Broken => 0.7,
            _ => 0.0,
        })
        .sum();
    
    let overall = (unresolved as f32 * 0.1 + prophecy_pressure + relationship_strain)
        .min(1.0);
    
    TensionMarkers {
        unresolved_conflicts: unresolved,
        broken_promises: 0,
        pending_revenge: 0,
        prophecy_pressure,
        relationship_strain,
        overall_tension: overall,
    }
}
