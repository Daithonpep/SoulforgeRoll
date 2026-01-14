use noise::{NoiseFn, Perlin, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use super::parser::{CharacterGeography, PointType, Biome, CitySize, EmotionalTone};

/// Configuración del mundo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub sea_level: f64,
    pub mountain_threshold: f64,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            seed: 12345,
            sea_level: 0.15,
            mountain_threshold: 0.65,
        }
    }
}

/// Celda del mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCell {
    pub x: u32,
    pub y: u32,
    pub elevation: f64,
    pub moisture: f64,
    pub temperature: f64,
    pub biome: BiomeType,
    pub is_water: bool,
    pub is_coast: bool,
    pub river_strength: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiomeType {
    DeepOcean,
    Ocean,
    Coast,
    Beach,
    Plains,
    Forest,
    DenseForest,
    Hills,
    Mountains,
    SnowPeaks,
    Desert,
    Swamp,
    Tundra,
}

/// Punto de interés en el mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPOI {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub poi_type: POIType,
    pub icon: String,
    pub owner_character: Option<String>,
    pub emotional_tone: String,
    pub importance: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum POIType {
    Origin,
    City,
    Village,
    Castle,
    Ruins,
    Temple,
    Cave,
    Landmark,
    DangerZone,
}

/// Camino entre puntos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPath {
    pub from_id: String,
    pub to_id: String,
    pub points: Vec<(f32, f32)>,
    pub path_type: PathType,
    pub danger_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathType {
    MainRoad,
    Trail,
    River,
    SeaRoute,
    SecretPath,
}

/// El mundo generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedWorld {
    pub config: WorldConfig,
    pub heightmap: Vec<Vec<f64>>,
    pub cells: Vec<Vec<MapCell>>,
    pub pois: Vec<MapPOI>,
    pub paths: Vec<MapPath>,
    pub regions: Vec<MapRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapRegion {
    pub name: String,
    pub center_x: f32,
    pub center_y: f32,
    pub bounds: (f32, f32, f32, f32), // x1, y1, x2, y2
    pub dominant_biome: BiomeType,
    pub owner_character: Option<String>,
}

pub struct WorldGenerator {
    config: WorldConfig,
    rng: ChaCha8Rng,
    perlin: Perlin,
}

impl WorldGenerator {
    pub fn new(config: WorldConfig) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(config.seed);
        let perlin = Perlin::new(config.seed as u32);
        
        Self { config, rng, perlin }
    }

    /// Generar mundo desde geografías de personajes
    pub fn generate_from_characters(
        &mut self,
        geographies: Vec<CharacterGeography>,
    ) -> GeneratedWorld {
        // 1. Generar heightmap base
        let heightmap = self.generate_heightmap();
        
        // 2. Calcular celdas con biomas
        let cells = self.calculate_cells(&heightmap);
        
        // 3. Asignar regiones a cada personaje
        let regions = self.assign_regions(&geographies);
        
        // 4. Colocar POIs basados en personajes
        let pois = self.place_pois(&geographies, &regions, &cells);
        
        // 5. Generar caminos entre POIs
        let paths = self.generate_paths(&pois, &cells);
        
        GeneratedWorld {
            config: self.config.clone(),
            heightmap,
            cells,
            pois,
            paths,
            regions,
        }
    }

    fn generate_heightmap(&self) -> Vec<Vec<f64>> {
        let mut heightmap = vec![vec![0.0; self.config.width as usize]; self.config.height as usize];
        
        for y in 0..self.config.height as usize {
            for x in 0..self.config.width as usize {
                // Múltiples octavas de ruido para terreno realista
                let nx = x as f64 / self.config.width as f64;
                let ny = y as f64 / self.config.height as f64;
                
                let elevation = 
                    1.0  * self.perlin.get([nx * 4.0, ny * 4.0]) +
                    0.5  * self.perlin.get([nx * 8.0, ny * 8.0]) +
                    0.25 * self.perlin.get([nx * 16.0, ny * 16.0]) +
                    0.125 * self.perlin.get([nx * 32.0, ny * 32.0]);
                
                // Normalizar a 0-1
                let normalized = (elevation + 1.0) / 2.0;
                
                // Aplicar isla (bordes más bajos)
                let dx = nx - 0.5;
                let dy = ny - 0.5;
                let distance = (dx * dx + dy * dy).sqrt() * 2.0;
                let island_factor = (1.0 - distance.min(1.0)).powf(0.5);
                
                heightmap[y][x] = normalized * island_factor;
            }
        }
        
        heightmap
    }

    fn calculate_cells(&self, heightmap: &[Vec<f64>]) -> Vec<Vec<MapCell>> {
        let mut cells = Vec::new();
        
        for (y, row) in heightmap.iter().enumerate() {
            let mut cell_row = Vec::new();
            
            for (x, &elevation) in row.iter().enumerate() {
                // Calcular humedad
                let nx = x as f64 / self.config.width as f64;
                let ny = y as f64 / self.config.height as f64;
                let moisture = (self.perlin.get([nx * 6.0 + 100.0, ny * 6.0]) + 1.0) / 2.0;
                
                // Temperatura (decrece con altitud y latitud)
                let latitude_factor = 1.0 - (ny as f64 / self.config.height as f64 - 0.5).abs() * 1.5;
                let altitude_factor = 1.0 - elevation;
                let temperature = (latitude_factor * 0.6 + altitude_factor * 0.4).max(0.0).min(1.0);
                
                let is_water = elevation < self.config.sea_level;
                let is_coast = !is_water && self.is_near_water(heightmap, x, y);
                
                let biome = self.determine_biome(elevation, moisture, temperature, is_water, is_coast);
                
                cell_row.push(MapCell {
                    x: x as u32,
                    y: y as u32,
                    elevation,
                    moisture,
                    temperature,
                    biome,
                    is_water,
                    is_coast,
                    river_strength: 0.0,
                });
            }
            
            cells.push(cell_row);
        }
        
        cells
    }

    fn is_near_water(&self, heightmap: &[Vec<f64>], x: usize, y: usize) -> bool {
        let check_range = 3;
        
        for dy in -(check_range as i32)..=check_range as i32 {
            for dx in -(check_range as i32)..=check_range as i32 {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                
                if ny < heightmap.len() && nx < heightmap[0].len() {
                    if heightmap[ny][nx] < self.config.sea_level {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    fn determine_biome(
        &self,
        elevation: f64,
        moisture: f64,
        temperature: f64,
        is_water: bool,
        is_coast: bool,
    ) -> BiomeType {
        if is_water {
            if elevation < self.config.sea_level - 0.15 {
                return BiomeType::DeepOcean;
            }
            return BiomeType::Ocean;
        }
        
        if is_coast {
            if moisture < 0.3 {
                return BiomeType::Beach;
            }
            return BiomeType::Coast;
        }
        
        // Montañas altas
        if elevation > self.config.mountain_threshold + 0.1 {
            if temperature < 0.3 {
                return BiomeType::SnowPeaks;
            }
            return BiomeType::Mountains;
        }
        
        // Colinas
        if elevation > self.config.mountain_threshold - 0.1 {
            return BiomeType::Hills;
        }
        
        // Biomas por temperatura y humedad
        if temperature < 0.2 {
            return BiomeType::Tundra;
        }
        
        if temperature > 0.8 && moisture < 0.2 {
            return BiomeType::Desert;
        }
        
        if moisture > 0.8 && temperature > 0.4 {
            return BiomeType::Swamp;
        }
        
        if moisture > 0.6 {
            if moisture > 0.75 {
                return BiomeType::DenseForest;
            }
            return BiomeType::Forest;
        }
        
        BiomeType::Plains
    }

    /// Asignar regiones del mapa a cada personaje
    fn assign_regions(&mut self, geographies: &[CharacterGeography]) -> Vec<MapRegion> {
        let num_characters = geographies.len();
        if num_characters == 0 {
            return Vec::new();
        }
        
        let mut regions = Vec::new();
        
        // Dividir el mapa en secciones según número de jugadores
        let layout = match num_characters {
            1 => vec![(0.5, 0.5)], // Centro
            2 => vec![(0.3, 0.5), (0.7, 0.5)], // Izquierda, Derecha
            3 => vec![(0.5, 0.25), (0.25, 0.7), (0.75, 0.7)], // Triángulo
            4 => vec![(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)], // Esquinas
            _ => {
                // Distribución en espiral para más jugadores
                let mut positions = Vec::new();
                let angle_step = std::f32::consts::TAU / num_characters as f32;
                for i in 0..num_characters {
                    let angle = angle_step * i as f32;
                    let radius = 0.3;
                    let x = 0.5 + angle.cos() * radius;
                    let y = 0.5 + angle.sin() * radius;
                    positions.push((x, y));
                }
                positions
            }
        };
        
        for (i, geo) in geographies.iter().enumerate() {
            let (cx, cy) = layout.get(i).copied().unwrap_or((0.5, 0.5));
            
            // Determinar bioma dominante según origen del personaje
            let dominant_biome = self.biome_to_map_biome(&geo.origin.biome);
            
            // Calcular bounds de la región
            let region_size = 0.25; // 25% del mapa cada región
            let bounds = (
                (cx - region_size / 2.0).max(0.0),
                (cy - region_size / 2.0).max(0.0),
                (cx + region_size / 2.0).min(1.0),
                (cy + region_size / 2.0).min(1.0),
            );
            
            regions.push(MapRegion {
                name: geo.origin.name.clone(),
                center_x: cx * self.config.width as f32,
                center_y: cy * self.config.height as f32,
                bounds: (
                    bounds.0 * self.config.width as f32,
                    bounds.1 * self.config.height as f32,
                    bounds.2 * self.config.width as f32,
                    bounds.3 * self.config.height as f32,
                ),
                dominant_biome,
                owner_character: Some(geo.character_id.clone()),
            });
        }
        
        regions
    }

    fn biome_to_map_biome(&self, biome: &Biome) -> BiomeType {
        match biome {
            Biome::Mountains { snow_capped: true, .. } => BiomeType::SnowPeaks,
            Biome::Mountains { .. } => BiomeType::Mountains,
            Biome::Forest { cursed: true, .. } => BiomeType::DenseForest,
            Biome::Forest { .. } => BiomeType::Forest,
            Biome::Coast { .. } => BiomeType::Coast,
            Biome::Plains { .. } => BiomeType::Plains,
            Biome::Desert { .. } => BiomeType::Desert,
            Biome::Swamp { .. } => BiomeType::Swamp,
            Biome::Tundra => BiomeType::Tundra,
            Biome::Jungle => BiomeType::DenseForest,
            Biome::Islands => BiomeType::Coast,
        }
    }

    /// Colocar POIs basados en geografía de personajes
    fn place_pois(
        &mut self,
        geographies: &[CharacterGeography],
        regions: &[MapRegion],
        cells: &[Vec<MapCell>],
    ) -> Vec<MapPOI> {
        let mut pois = Vec::new();
        
        for (i, geo) in geographies.iter().enumerate() {
            let region = &regions[i];
            
            // POI de origen (siempre presente)
            let origin_pos = self.find_suitable_position(
                region.center_x,
                region.center_y,
                &geo.origin.biome,
                cells,
            );
            
            pois.push(MapPOI {
                id: format!("origin_{}", geo.character_id),
                name: geo.origin.name.clone(),
                x: origin_pos.0,
                y: origin_pos.1,
                poi_type: POIType::Origin,
                icon: self.get_poi_icon(&geo.origin.point_type),
                owner_character: Some(geo.character_id.clone()),
                emotional_tone: format!("{:?}", geo.origin.emotional_tone),
                importance: 10,
            });
            
            // POIs relacionados
            for loc in &geo.related_locations {
                let offset_x = self.rng.gen_range(-100.0..100.0);
                let offset_y = self.rng.gen_range(-100.0..100.0);
                
                let pos = self.find_suitable_position(
                    region.center_x + offset_x,
                    region.center_y + offset_y,
                    &loc.biome,
                    cells,
                );
                
                pois.push(MapPOI {
                    id: format!("loc_{}_{}", geo.character_id, pois.len()),
                    name: loc.name.clone(),
                    x: pos.0,
                    y: pos.1,
                    poi_type: self.convert_point_type(&loc.point_type),
                    icon: self.get_poi_icon(&loc.point_type),
                    owner_character: Some(geo.character_id.clone()),
                    emotional_tone: format!("{:?}", loc.emotional_tone),
                    importance: loc.importance,
                });
            }
            
            // Zonas de peligro
            for danger in &geo.danger_zones {
                let offset = 80.0;
                let angle = self.rng.gen_range(0.0..std::f32::consts::TAU);
                
                pois.push(MapPOI {
                    id: format!("danger_{}_{}", geo.character_id, pois.len()),
                    name: danger.name.clone(),
                    x: region.center_x + angle.cos() * offset,
                    y: region.center_y + angle.sin() * offset,
                    poi_type: POIType::DangerZone,
                    icon: "☠️".to_string(),
                    owner_character: Some(geo.character_id.clone()),
                    emotional_tone: "Fear".to_string(),
                    importance: 7,
                });
            }
        }

        // FILLER: Generar POIs aleatorios para llenar el mundo
        let target_poi_count = 15;
        let current_count = pois.len();
        
        if current_count < target_poi_count {
            for _ in 0..(target_poi_count - current_count) {
                let (rx, ry) = (self.rng.gen_range(0.0..self.config.width as f32), self.rng.gen_range(0.0..self.config.height as f32));
                
                // Buscar tierra firme
                let pos = self.find_suitable_position(rx, ry, &Biome::Plains { fertile: true }, cells);
                
                let poi_type = match self.rng.gen_range(0..10) {
                    0..=3 => POIType::Village,
                    4..=5 => POIType::Ruins,
                    6 => POIType::Castle,
                    7 => POIType::Cave,
                    8 => POIType::Temple,
                    _ => POIType::Landmark,
                };
                
                let name = self.generate_random_name();
                
                pois.push(MapPOI {
                    id: format!("rnd_{}", pois.len()),
                    name,
                    x: pos.0,
                    y: pos.1,
                    icon: match poi_type {
                        POIType::Village => "🛖".to_string(),
                        POIType::Ruins => "🏚️".to_string(),
                        POIType::Castle => "🏰".to_string(),
                        POIType::Cave => "🕳️".to_string(),
                        POIType::Temple => "⛪".to_string(),
                        _ => "📍".to_string(),
                    },
                    poi_type,
                    owner_character: None,
                    emotional_tone: "Neutral".to_string(),
                    importance: 2,
                });
            }
        }
        
        pois
    }

    fn generate_random_name(&mut self) -> String {
        let prefixes = ["Val", "Mor", "Dor", "Sil", "Gar", "Wen", "Thal", "Kae", "Nor", "Sur"];
        let suffixes = ["heim", "grad", "dale", "wood", "mount", "ford", "keep", "wick", "ia", "on"];
        
        let p = prefixes[self.rng.gen_range(0..prefixes.len())];
        let s = suffixes[self.rng.gen_range(0..suffixes.len())];
        
        format!("{}{}", p, s)
    }

    fn find_suitable_position(
        &mut self,
        target_x: f32,
        target_y: f32,
        preferred_biome: &Biome,
        cells: &[Vec<MapCell>],
    ) -> (f32, f32) {
        // Buscar posición cercana que no sea agua
        let search_radius = 50.0;
        let mut best_pos = (target_x, target_y);
        let mut best_score = f32::MIN;
        
        for _ in 0..20 {
            let test_x = target_x + self.rng.gen_range(-search_radius..search_radius);
            let test_y = target_y + self.rng.gen_range(-search_radius..search_radius);
            
            // Clamp to map bounds
            let test_x = test_x.max(0.0).min(self.config.width as f32 - 1.0);
            let test_y = test_y.max(0.0).min(self.config.height as f32 - 1.0);
            
            let cell_x = test_x as usize;
            let cell_y = test_y as usize;
            
            if cell_y < cells.len() && cell_x < cells[0].len() {
                let cell = &cells[cell_y][cell_x];
                
                if !cell.is_water {
                    let mut score = 0.0;
                    
                    // Bonus por estar en costa si el bioma lo requiere
                    if matches!(preferred_biome, Biome::Coast { .. }) && cell.is_coast {
                        score += 10.0;
                    }
                    
                    // Bonus por elevación alta si es montaña
                    if matches!(preferred_biome, Biome::Mountains { .. }) {
                        score += cell.elevation as f32 * 10.0;
                    }
                    
                    // Bonus por humedad alta si es bosque
                    if matches!(preferred_biome, Biome::Forest { .. }) {
                        score += cell.moisture as f32 * 10.0;
                    }
                    
                    // Cercanía al target
                    let dist = ((test_x - target_x).powi(2) + (test_y - target_y).powi(2)).sqrt();
                    score -= dist * 0.1;
                    
                    if score > best_score {
                        best_score = score;
                        best_pos = (test_x, test_y);
                    }
                }
            }
        }
        
        best_pos
    }

    fn get_poi_icon(&self, point_type: &PointType) -> String {
        match point_type {
            PointType::City { size } => match size {
                CitySize::Metropolis => "🏰",
                CitySize::Large => "🏛️",
                CitySize::Medium => "🏘️",
                CitySize::Small => "🏠",
                CitySize::Hamlet => "🛖",
            },
            PointType::Village => "🏘️",
            PointType::Castle => "🏰",
            PointType::Ruins => "🏚️",
            PointType::Temple => "⛪",
            PointType::Cave => "🕳️",
            PointType::Landmark => "🗿",
            PointType::Battlefield => "⚔️",
            PointType::Grave => "⚰️",
        }.to_string()
    }

    fn convert_point_type(&self, pt: &PointType) -> POIType {
        match pt {
            PointType::City { .. } => POIType::City,
            PointType::Village => POIType::Village,
            PointType::Castle => POIType::Castle,
            PointType::Ruins => POIType::Ruins,
            PointType::Temple => POIType::Temple,
            PointType::Cave => POIType::Cave,
            PointType::Landmark | PointType::Battlefield | PointType::Grave => POIType::Landmark,
        }
    }

    /// Generar caminos entre POIs
    fn generate_paths(&mut self, pois: &[MapPOI], cells: &[Vec<MapCell>]) -> Vec<MapPath> {
        let mut paths = Vec::new();
        
        // Conectar todos los orígenes entre sí (caminos principales)
        let origins: Vec<_> = pois.iter()
            .filter(|p| matches!(p.poi_type, POIType::Origin))
            .collect();
        
        for i in 0..origins.len() {
            for j in (i + 1)..origins.len() {
                let path_points = self.pathfind(
                    origins[i].x, origins[i].y,
                    origins[j].x, origins[j].y,
                    cells,
                );
                
                paths.push(MapPath {
                    from_id: origins[i].id.clone(),
                    to_id: origins[j].id.clone(),
                    points: path_points,
                    path_type: PathType::MainRoad,
                    danger_level: 2,
                });
            }
        }
        
        // Conectar cada POI secundario a su origen
        for poi in pois {
            if matches!(poi.poi_type, POIType::Origin) { continue; }
            
            // Encontrar el origen del mismo personaje
            if let Some(owner) = &poi.owner_character {
                if let Some(origin) = origins.iter().find(|o| {
                    o.owner_character.as_ref() == Some(owner)
                }) {
                    let path_points = self.pathfind(
                        origin.x, origin.y,
                        poi.x, poi.y,
                        cells,
                    );
                    
                    let path_type = if matches!(poi.poi_type, POIType::DangerZone) {
                        PathType::Trail
                    } else {
                        PathType::MainRoad
                    };
                    
                    paths.push(MapPath {
                        from_id: origin.id.clone(),
                        to_id: poi.id.clone(),
                        points: path_points,
                        path_type,
                        danger_level: if matches!(poi.poi_type, POIType::DangerZone) { 8 } else { 3 },
                    });
                }
            }
        }
        
        paths
    }

    /// Pathfinding simple que evita agua
    fn pathfind(
        &self,
        start_x: f32, start_y: f32,
        end_x: f32, end_y: f32,
        cells: &[Vec<MapCell>],
    ) -> Vec<(f32, f32)> {
        let mut points = vec![(start_x, start_y)];
        
        // Interpolación simple con desvío para evitar agua
        let steps = 20;
        
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let base_x = start_x + (end_x - start_x) * t;
            let base_y = start_y + (end_y - start_y) * t;
            
            // Añadir algo de curvatura natural
            let perpendicular_x = -(end_y - start_y);
            let perpendicular_y = end_x - start_x;
            let len = (perpendicular_x.powi(2) + perpendicular_y.powi(2)).sqrt();
            
            let wave = (t * std::f32::consts::PI * 2.0).sin() * 15.0;
            let final_x = base_x + (perpendicular_x / len) * wave;
            let final_y = base_y + (perpendicular_y / len) * wave;
            
            // Verificar si es agua y ajustar
            let cell_x = (final_x as usize).min(cells[0].len() - 1);
            let cell_y = (final_y as usize).min(cells.len() - 1);
            
            if cells[cell_y][cell_x].is_water {
                // Buscar tierra cercana
                points.push((base_x, base_y)); // Usar punto base
            } else {
                points.push((final_x, final_y));
            }
        }
        
        points.push((end_x, end_y));
        points
    }
}
