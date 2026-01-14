use serde::{Deserialize, Serialize};
use rand::{Rng, seq::SliceRandom};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub description: String,
    pub stats: Vec<String>,
    pub value: u32,
    pub forged_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Trinket,
    Potion,
    Material,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    Common,     // Gris
    Uncommon,   // Verde
    Rare,       // Azul
    Epic,       // Morado
    Legendary,  // Dorado
    Cursed,     // Rojo
}

pub struct ItemGenerator;

impl ItemGenerator {
    pub fn generate_loot(rarity: Option<Rarity>) -> Item {
        let mut rng = rand::thread_rng();
        
        let actual_rarity = rarity.unwrap_or_else(|| {
            let roll = rng.gen_range(0..100);
            if roll < 50 { Rarity::Common }
            else if roll < 75 { Rarity::Uncommon }
            else if roll < 90 { Rarity::Rare }
            else if roll < 98 { Rarity::Epic }
            else { Rarity::Legendary }
        });

        let item_type = match rng.gen_range(0..10) {
            0..=3 => ItemType::Weapon,
            4..=6 => ItemType::Armor,
            7..=8 => ItemType::Trinket,
            _ => ItemType::Potion,
        };

        Self::create_item(item_type, actual_rarity)
    }

    pub fn forge(item_type: ItemType, material_quality: u8, smith_skill: u8) -> Item {
        let mut rng = rand::thread_rng();
        
        // Forge logic: Quality + Skill determines rarity
        let score = material_quality as u32 + smith_skill as u32 + rng.gen_range(0..20);
        let rarity = if score < 50 { Rarity::Common }
            else if score < 100 { Rarity::Uncommon }
            else if score < 150 { Rarity::Rare }
            else if score < 190 { Rarity::Epic }
            else { Rarity::Legendary };

        Self::create_item(item_type, rarity)
    }

    fn create_item(item_type: ItemType, rarity: Rarity) -> Item {
        let mut rng = rand::thread_rng();
        
        let (name, desc, stats) = match item_type {
            ItemType::Weapon => {
                let bases = ["Espada", "Hacha", "Lanza", "Daga", "Maza", "Arco", "Mandoble"];
                let adjs = ["Oxidada", "de Hierro", "de Acero", "Obsidiana", "Rúnica", "Ancestral", "Vorpal", "Dragónica"];
                let nouns = ["del Soldado", "del Rey", "de la Furia", "del Ocaso", "de Sangre", "del Viento", "del Dolor"];
                
                let n = format!("{} {} {}", 
                    bases.choose(&mut rng).unwrap(), 
                    match rarity { Rarity::Common => "", _ => adjs.choose(&mut rng).unwrap() },
                    match rarity { Rarity::Common | Rarity::Uncommon => "", _ => nouns.choose(&mut rng).unwrap() }
                ).trim().replace("  ", " ");

                let dmg = match rarity {
                    Rarity::Common => "1d6",
                    Rarity::Uncommon => "1d8 + 1",
                    Rarity::Rare => "2d6 + 2",
                    Rarity::Epic => "2d8 + 4 + Fire",
                    Rarity::Legendary => "3d10 + 10 + Soulbind",
                    Rarity::Cursed => "10d10 (Daño a usuario)",
                };

                (n, "Un arma forjada para el combate.".to_string(), vec![format!("Daño: {}", dmg)])
            },
            ItemType::Armor => {
                let bases = ["Peto", "Escudo", "Yelmo", "Botas", "Guanteletes"];
                let mats = ["de Cuero", "de Malla", "de Placas", "de Escamas", "de Mitril"];
                
                let n = format!("{} {}", bases.choose(&mut rng).unwrap(), mats.choose(&mut rng).unwrap());
                (n, "Protección esencial.".to_string(), vec!["Defensa +2".to_string()])
            },
            ItemType::Trinket => ("Amuleto Perdido".to_string(), "Brilla tenuemente.".to_string(), vec!["Suerte +1".to_string()]),
            ItemType::Potion => ("Poción Curativa".to_string(), "Bebida roja y espesa.".to_string(), vec!["Cura 2d4+2".to_string()]),
            _ => ("Material Bruto".to_string(), "Material de crafting.".to_string(), vec![])
        };

        Item {
            name,
            item_type,
            rarity,
            description: desc,
            stats,
            value: rng.gen_range(10..1000),
            forged_by: None,
        }
    }
}
