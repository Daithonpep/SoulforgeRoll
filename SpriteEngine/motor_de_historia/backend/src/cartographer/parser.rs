use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Datos geográficos extraídos de un personaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGeography {
    pub character_id: String,
    pub character_name: String,
    pub origin: GeographicPoint,
    pub related_locations: Vec<GeographicPoint>,
    pub routes: Vec<Route>,
    pub danger_zones: Vec<DangerZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicPoint {
    pub name: String,
    pub biome: Biome,
    pub point_type: PointType,
    pub importance: u8, // 1-10
    pub descriptors: Vec<String>,
    pub emotional_tone: EmotionalTone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Biome {
    Mountains { snow_capped: bool, volcanic: bool },
    Forest { density: u8, cursed: bool },
    Coast { has_port: bool, cliffs: bool },
    Plains { fertile: bool },
    Desert { oasis: bool },
    Swamp { toxic: bool },
    Tundra,
    Jungle,
    Islands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointType {
    City { size: CitySize },
    Village,
    Ruins,
    Castle,
    Temple,
    Cave,
    Landmark,
    Battlefield,
    Grave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CitySize { Metropolis, Large, Medium, Small, Hamlet }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmotionalTone {
    Home,       // Lugar de origen - cálido
    Fear,       // Trauma - marcas rojas
    Loss,       // Pérdida - gris/apagado
    Adventure,  // Descubrimiento - dorado
    Mystery,    // Desconocido - niebla
    Conflict,   // Batalla - rojo oscuro
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub from: String,
    pub to: String,
    pub route_type: RouteType,
    pub danger_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteType {
    Road,
    Trail,
    River,
    SeaRoute,
    SecretPath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerZone {
    pub name: String,
    pub threat_type: String,
    pub radius: f32,
    pub source_trauma: String,
}

/// El parser que extrae geografía del trasfondo
pub struct BackstoryParser {
    biome_keywords: HashMap<String, Biome>,
    point_keywords: HashMap<String, PointType>,
    emotion_keywords: HashMap<String, EmotionalTone>,
}

impl BackstoryParser {
    pub fn new() -> Self {
        Self {
            biome_keywords: Self::init_biome_keywords(),
            point_keywords: Self::init_point_keywords(),
            emotion_keywords: Self::init_emotion_keywords(),
        }
    }

    fn init_biome_keywords() -> HashMap<String, Biome> {
        let mut map = HashMap::new();
        
        // Montañas
        for word in ["montaña", "monte", "pico", "cumbre", "cordillera", "sierra", "cima"] {
            map.insert(word.to_string(), Biome::Mountains { snow_capped: false, volcanic: false });
        }
        for word in ["nevado", "helado", "glaciar", "nieve"] {
            map.insert(word.to_string(), Biome::Mountains { snow_capped: true, volcanic: false });
        }
        for word in ["volcán", "lava", "ceniza", "volcánico"] {
            map.insert(word.to_string(), Biome::Mountains { snow_capped: false, volcanic: true });
        }
        
        // Bosques
        for word in ["bosque", "arboleda", "floresta", "espesura", "selva"] {
            map.insert(word.to_string(), Biome::Forest { density: 7, cursed: false });
        }
        for word in ["maldito", "oscuro", "tenebroso", "embrujado", "prohibido"] {
            map.insert(word.to_string(), Biome::Forest { density: 9, cursed: true });
        }
        
        // Costa
        for word in ["costa", "playa", "mar", "océano", "bahía", "cala"] {
            map.insert(word.to_string(), Biome::Coast { has_port: false, cliffs: false });
        }
        for word in ["puerto", "muelle", "astillero", "dársena"] {
            map.insert(word.to_string(), Biome::Coast { has_port: true, cliffs: false });
        }
        for word in ["acantilado", "risco", "precipicio", "farallón"] {
            map.insert(word.to_string(), Biome::Coast { has_port: false, cliffs: true });
        }
        
        // Llanuras
        for word in ["pradera", "llanura", "campo", "prado", "estepa", "valle"] {
            map.insert(word.to_string(), Biome::Plains { fertile: true });
        }
        for word in ["páramo", "yermo", "baldío", "desolado"] {
            map.insert(word.to_string(), Biome::Plains { fertile: false });
        }
        
        // Desierto
        for word in ["desierto", "arena", "dunas", "seco", "árido"] {
            map.insert(word.to_string(), Biome::Desert { oasis: false });
        }
        for word in ["oasis", "palmera", "manantial"] {
            map.insert(word.to_string(), Biome::Desert { oasis: true });
        }
        
        // Pantano
        for word in ["pantano", "ciénaga", "marisma", "lodazal", "turbera"] {
            map.insert(word.to_string(), Biome::Swamp { toxic: false });
        }
        for word in ["venenoso", "tóxico", "pútrido", "pestilente"] {
            map.insert(word.to_string(), Biome::Swamp { toxic: true });
        }
        
        // Otros
        for word in ["tundra", "permafrost", "ártico"] {
            map.insert(word.to_string(), Biome::Tundra);
        }
        for word in ["jungla", "tropical", "selvático"] {
            map.insert(word.to_string(), Biome::Jungle);
        }
        for word in ["isla", "archipiélago", "atolón"] {
            map.insert(word.to_string(), Biome::Islands);
        }
        
        map
    }

    fn init_point_keywords() -> HashMap<String, PointType> {
        let mut map = HashMap::new();
        
        // Ciudades
        map.insert("ciudad".into(), PointType::City { size: CitySize::Medium });
        map.insert("capital".into(), PointType::City { size: CitySize::Metropolis });
        map.insert("metrópoli".into(), PointType::City { size: CitySize::Metropolis });
        map.insert("pueblo".into(), PointType::City { size: CitySize::Small });
        map.insert("aldea".into(), PointType::Village);
        map.insert("villa".into(), PointType::City { size: CitySize::Small });
        
        // Estructuras
        map.insert("castillo".into(), PointType::Castle);
        map.insert("fortaleza".into(), PointType::Castle);
        map.insert("torre".into(), PointType::Castle);
        map.insert("templo".into(), PointType::Temple);
        map.insert("santuario".into(), PointType::Temple);
        map.insert("iglesia".into(), PointType::Temple);
        map.insert("monasterio".into(), PointType::Temple);
        
        // Ruinas/Cuevas
        map.insert("ruinas".into(), PointType::Ruins);
        map.insert("ruina".into(), PointType::Ruins);
        map.insert("antiguo".into(), PointType::Ruins);
        map.insert("cueva".into(), PointType::Cave);
        map.insert("caverna".into(), PointType::Cave);
        map.insert("gruta".into(), PointType::Cave);
        map.insert("mina".into(), PointType::Cave);
        
        // Otros
        map.insert("tumba".into(), PointType::Grave);
        map.insert("cementerio".into(), PointType::Grave);
        map.insert("cripta".into(), PointType::Grave);
        map.insert("batalla".into(), PointType::Battlefield);
        map.insert("guerra".into(), PointType::Battlefield);
        
        map
    }

    fn init_emotion_keywords() -> HashMap<String, EmotionalTone> {
        let mut map = HashMap::new();
        
        // Home
        for word in ["nací", "hogar", "casa", "familia", "creció", "infancia", "madre", "padre"] {
            map.insert(word.to_string(), EmotionalTone::Home);
        }
        
        // Fear
        for word in ["huí", "escapé", "miedo", "terror", "pesadilla", "horrendo", "maldición"] {
            map.insert(word.to_string(), EmotionalTone::Fear);
        }
        
        // Loss
        for word in ["murió", "perdí", "muerte", "asesinado", "destruido", "arrasado", "caído"] {
            map.insert(word.to_string(), EmotionalTone::Loss);
        }
        
        // Adventure
        for word in ["descubrí", "encontré", "exploré", "aventura", "viaje", "expedición"] {
            map.insert(word.to_string(), EmotionalTone::Adventure);
        }
        
        // Mystery
        for word in ["misterio", "secreto", "oculto", "antiguo", "olvidado", "leyenda"] {
            map.insert(word.to_string(), EmotionalTone::Mystery);
        }
        
        // Conflict
        for word in ["batalla", "guerra", "enemigo", "venganza", "lucha", "combate"] {
            map.insert(word.to_string(), EmotionalTone::Conflict);
        }
        
        map
    }

    /// Parsear trasfondo completo de un personaje
    pub fn parse_character(&self, character_json: &serde_json::Value) -> CharacterGeography {
        let name = character_json["identidad"]["nombre"].as_str().unwrap_or("Unknown").to_string(); // Ajustado para coincidir con la estructura real del JSON del usuario (identidad.nombre)
        let id = character_json["id"].as_str().unwrap_or(&uuid::Uuid::new_v4().to_string()).to_string();
        
        // Extraer texto del trasfondo/biografía
        let mut backstory = String::new();
        if let Some(narrativa) = character_json.get("narrativa") {
            if let Some(resumen) = narrativa.get("resumen_historia") {
                 backstory.push_str(resumen.as_str().unwrap_or(""));
            }
        }
        if let Some(biografia) = character_json.get("biografia") {
            if let Some(historia_completa) = biografia.get("historia_completa") {
                backstory.push_str(" ");
                backstory.push_str(historia_completa.as_str().unwrap_or(""));
            }
        }
        
        // Extraer orígenes
        let origin_text = character_json["biografia"]["lugar_origen"]
            .as_str()
            .unwrap_or("");
        
        let origin = self.extract_origin(origin_text, &backstory);
        let locations = self.extract_locations(&backstory);
        let routes = self.extract_routes(&origin, &locations);
        let danger_zones = self.extract_dangers(&backstory);
        
        CharacterGeography {
            character_id: id,
            character_name: name,
            origin,
            related_locations: locations,
            routes,
            danger_zones,
        }
    }

    fn extract_origin(&self, origin_text: &str, backstory: &str) -> GeographicPoint {
        let combined = format!("{} {}", origin_text, backstory);
        let words: Vec<&str> = combined.split_whitespace().collect();
        
        // Buscar nombre propio (capitalizado después de preposición de lugar)
        let mut location_name = if !origin_text.is_empty() {
             Some(origin_text.to_string())
        } else {
            None
        };

        if location_name.is_none() {
            let location_preps = ["en", "de", "del", "desde", "hacia"];
            for (i, word) in words.iter().enumerate() {
                if location_preps.contains(&word.to_lowercase().as_str()) {
                    if let Some(next) = words.get(i + 1) {
                        if next.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                            // Capturar nombre completo (puede ser múltiples palabras)
                            let mut full_name = next.to_string();
                            for j in (i + 2)..words.len().min(i + 5) {
                                if let Some(w) = words.get(j) {
                                    if w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) 
                                    && !location_preps.contains(&w.to_lowercase().as_str()) {
                                        full_name.push(' ');
                                        full_name.push_str(w);
                                    } else {
                                        break;
                                    }
                                }
                            }
                            location_name = Some(full_name);
                            break;
                        }
                    }
                }
            }
        }
        
        // Detectar bioma
        let biome = self.detect_biome(&combined);
        
        // Detectar tipo de punto
        let point_type = self.detect_point_type(&combined);
        
        // Detectar emoción dominante
        let emotion = self.detect_emotion(&combined);
        
        GeographicPoint {
            name: location_name.unwrap_or_else(|| "Lugar de Origen".to_string()),
            biome,
            point_type,
            importance: 10, // El origen siempre es máxima importancia
            descriptors: self.extract_descriptors(&combined),
            emotional_tone: emotion,
        }
    }

    fn detect_biome(&self, text: &str) -> Biome {
        let text_lower = text.to_lowercase();
        
        for (keyword, biome) in &self.biome_keywords {
            if text_lower.contains(keyword) {
                return biome.clone();
            }
        }
        
        // Default: llanuras
        Biome::Plains { fertile: true }
    }

    fn detect_point_type(&self, text: &str) -> PointType {
        let text_lower = text.to_lowercase();
        
        for (keyword, point_type) in &self.point_keywords {
            if text_lower.contains(keyword) {
                return point_type.clone();
            }
        }
        
        PointType::Village
    }

    fn detect_emotion(&self, text: &str) -> EmotionalTone {
        let text_lower = text.to_lowercase();
        let mut emotion_counts: HashMap<EmotionalTone, u32> = HashMap::new();
        
        for (keyword, emotion) in &self.emotion_keywords {
            if text_lower.contains(keyword) {
                *emotion_counts.entry(emotion.clone()).or_insert(0) += 1;
            }
        }
        
        emotion_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(emotion, _)| emotion)
            .unwrap_or(EmotionalTone::Home)
    }

    fn extract_descriptors(&self, text: &str) -> Vec<String> {
        let descriptors = [
            "antiguo", "oscuro", "brillante", "sagrado", "maldito",
            "olvidado", "próspero", "decadente", "misterioso", "peligroso",
            "pacífico", "guerrero", "comercial", "aislado", "legendario"
        ];
        
        let text_lower = text.to_lowercase();
        descriptors
            .iter()
            .filter(|d| text_lower.contains(*d))
            .map(|d| d.to_string())
            .collect()
    }

    fn extract_locations(&self, backstory: &str) -> Vec<GeographicPoint> {
        let mut locations = Vec::new();
        let sentences: Vec<&str> = backstory.split(['.', '!', '?']).collect();
        
        for sentence in sentences {
            if sentence.len() < 10 { continue; }
            
            // Buscar menciones de lugares
            let words: Vec<&str> = sentence.split_whitespace().collect();
            let location_preps = ["en", "de", "del", "desde", "hacia", "hasta", "por"];
            
            for (i, word) in words.iter().enumerate() {
                if location_preps.contains(&word.to_lowercase().as_str()) {
                    if let Some(next) = words.get(i + 1) {
                        if next.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                            // Construir nombre del lugar
                            let mut name = next.to_string();
                            for j in (i + 2)..words.len().min(i + 4) {
                                if let Some(w) = words.get(j) {
                                    if w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                                        name.push(' ');
                                        name.push_str(w);
                                    } else {
                                        break;
                                    }
                                }
                            }
                            
                            // Evitar duplicados
                            if !locations.iter().any(|l: &GeographicPoint| l.name == name) {
                                locations.push(GeographicPoint {
                                    name,
                                    biome: self.detect_biome(sentence),
                                    point_type: self.detect_point_type(sentence),
                                    importance: 5,
                                    descriptors: self.extract_descriptors(sentence),
                                    emotional_tone: self.detect_emotion(sentence),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        locations
    }

    fn extract_routes(&self, origin: &GeographicPoint, locations: &[GeographicPoint]) -> Vec<Route> {
        let mut routes = Vec::new();
        
        // Conectar origen con cada ubicación
        for loc in locations {
            routes.push(Route {
                from: origin.name.clone(),
                to: loc.name.clone(),
                route_type: RouteType::Road,
                danger_level: match loc.emotional_tone {
                    EmotionalTone::Fear | EmotionalTone::Conflict => 8,
                    EmotionalTone::Mystery => 5,
                    _ => 2,
                },
            });
        }
        
        routes
    }

    fn extract_dangers(&self, backstory: &str) -> Vec<DangerZone> {
        let mut dangers = Vec::new();
        let danger_patterns = [
            ("maldición", "Maldición Antigua"),
            ("monstruo", "Territorio de Bestias"),
            ("bandido", "Caminos Peligrosos"),
            ("guerra", "Zona de Conflicto"),
            ("demonio", "Presencia Demoníaca"),
            ("muerto", "Tierra de los Muertos"),
            ("dragón", "Dominio del Dragón"),
        ];
        
        let text_lower = backstory.to_lowercase();
        
        for (pattern, threat) in danger_patterns {
            if text_lower.contains(pattern) {
                dangers.push(DangerZone {
                    name: format!("Zona de {}", threat),
                    threat_type: threat.to_string(),
                    radius: 100.0,
                    source_trauma: pattern.to_string(),
                });
            }
        }
        
        dangers
    }
}
