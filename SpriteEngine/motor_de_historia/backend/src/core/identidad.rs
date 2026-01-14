//! Sistema de identidad física y cultural

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Mundo, Genero};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identidad {
    pub nombre: String,
    pub apellido: Option<String>,
    pub titulo: Option<String>,
    pub edad: u32,
    pub genero: Genero,
    pub rasgo_distintivo: String,
    pub vestimenta: String,
    pub voz: String,
    pub manierismo: String,
    pub cicatriz: Option<String>,
}

impl Identidad {
    pub fn generar(rng: &mut impl Rng, mundo: &Mundo, genero_opt: Option<Genero>, edad_opt: Option<u32>) -> Self {
        let genero = genero_opt.unwrap_or_else(|| Self::generar_genero(rng));
        let (nombre, apellido) = Self::generar_nombre(rng, mundo, &genero);
        
        let edad = edad_opt.unwrap_or_else(|| rng.gen_range(18..55));

        Self {
            nombre,
            apellido,
            titulo: Self::generar_titulo(rng),
            edad,
            genero,
            rasgo_distintivo: Self::generar_rasgo(rng),
            vestimenta: Self::generar_vestimenta(rng),
            voz: Self::generar_voz(rng),
            manierismo: Self::generar_manierismo(rng),
            cicatriz: Self::generar_cicatriz(rng),
        }
    }
    
    fn generar_genero(rng: &mut impl Rng) -> Genero {
        if rng.gen_bool(0.5) { Genero::Masculino }
        else { Genero::Femenino }
    }
    
    fn generar_nombre(rng: &mut impl Rng, mundo: &Mundo, genero: &Genero) -> (String, Option<String>) {
        let nombres_m = match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "Aldric", "Kael", "Theron", "Varen", "Draken", "Ezran", "Lucian", "Malakai", "Silas", "Ronan", "Caden",
                "Roland", "Gareth", "Tristan", "Edmund", "Garrick", "Thorne", "Valerius", "Caelum", "Darius", 
                "Eamon", "Faelan", "Galen", "Hadrian", "Ivar", "Jareth", "Kaelen", "Leander", "Magnus", "Neron", 
                "Orion", "Peregrin", "Quentin", "Reric", "Soren", "Tavian", "Ulric", "Varic", "Wulf", "Xander", 
                "Yoric", "Zephyr", "Grimm", "Stryker", "Wolf", "Hawk", "Blade", "Frost", "Alaric", "Beric", 
                "Cedric", "Daric", "Elric", "Fenric", "Godric", "Henric", "Joric", "Kedric", "Lyric", "Meric", 
                "Osric", "Roderic", "Ulric", "Yorick", "Zeric", "Arthas", "Bran", "Cormac", "Declan", "Ewan", 
                "Finn", "Gavin", "Heath", "Ian", "Julian", "Kieran", "Lachlan", "Mason", "Nolan", "Owen", "Patrick", 
                "Quinn", "Rhys", "Sean", "Teague", "Vaughn", "Wyatt", "Xavier", "York", "Zane"
            ],
            Mundo::SciFiCyberpunk | Mundo::SciFiSpace | Mundo::SciFiPostApocaliptico => vec![
                "Zero", "Raze", "Vector", "Chrome", "Ash", "Cipher",
                "Neon", "Pulse", "Ryker", "Jax", "Kade", "Zane"
            ],
            // ═══ JAPÓN ═══
            Mundo::JaponFeudal | Mundo::AnimeFantasia | Mundo::Anime => vec![
                "Kenshin", "Takeshi", "Ryoma", "Musashi", "Nobunaga", "Ieyasu", "Shingen", "Yukimura", "Masamune", "Hanzo",
                "Sasuke", "Jubei", "Goemon", "Kojiro", "Toshiro", "Hiro", "Kenji", "Taro", "Jiro", "Saburo", "Shiro", 
                "Goro", "Rokuro", "Hachiro", "Kuro", "Akira", "Makoto", "Satoshi", "Yoshi", "Naoki", "Daisuke", 
                "Ryosuke", "Keisuke", "Sosuke", "Kyosuke", "Shosuke", "Eisuke", "Katsumi", "Masumi", "Yoshimi", 
                "Kazumi", "Haruto", "Yuto", "Souta", "Minato", "Ren", "Riku", "Sora", "Kaito", "Asahi", "Hinata", 
                "Itsuki", "Arata", "Yamato", "Tatsuki", "Hayato", "Daiki", "Tomoya", "Yuma", "Kunta", "Sho"
            ],
            // ═══ CHINA ═══
            Mundo::ChinaImperial | Mundo::Wuxia => vec![
                "Wei", "Zhang", "Chen", "Liu", "Zhao",
                "Feng", "Long", "Jian", "Ming", "Xian",
                "Yun", "Bao", "Hao", "Rui", "Tao"
            ],
            // ═══ COREA ═══
            Mundo::CoreaHistorica => vec![
                "Joon", "Min-ho", "Seok", "Tae-yang", "Hyun",
                "Sung", "Jin", "Dae", "Woo", "Kwan",
                "Yong", "Chul", "Hwan", "Suk", "Myung"
            ],
            // ═══ MITOLOGÍA ASIÁTICA ═══
            Mundo::MitologiaAsiatica => vec![
                "Ryujin", "Susanoo", "Inari", "Fujin", "Raijin",
                "Tsukuyomi", "Bishamon", "Ebisu", "Daikoku", "Hotei"
            ],
            // ═══ MITOLOGÍA GRIEGA ═══
            Mundo::MitologiaGriega => vec![
                "Alexios", "Nikolaos", "Theron", "Leonidas", "Demetrios",
                "Kassandros", "Stephanos", "Aristos", "Markos", "Petros"
            ],
            // ═══ MITOLOGÍA NÓRDICA ═══
            Mundo::MitologiaNordica => vec![
                "Bjorn", "Ragnar", "Leif", "Erik", "Gunnar",
                "Ivar", "Sigurd", "Thorstein", "Harald", "Ulf"
            ],
            // ═══ PIRATAS ═══
            Mundo::PiratasCaribe => vec![
                "Jack", "Blackbeard", "Morgan", "Drake", "Flint",
                "Bones", "Silver", "Hook", "Kidd", "Rackham"
            ],
            // ═══ WESTERN ═══
            Mundo::Western => vec![
                "Wyatt", "Jesse", "Billy", "Doc", "Clay",
                "Colt", "Dakota", "Dusty", "Harlan", "Zeke"
            ],
            // ═══ NOIR ═══
            Mundo::Noir => vec![
                "Vincent", "Raymond", "Philip", "Sam", "Jack",
                "Frank", "Eddie", "Tony", "Mickey", "Rocco"
            ],
            // ═══ VICTORIANO ═══  
            Mundo::Victoriano => vec![
                "Edmund", "Theodore", "Augustus", "Cornelius", "Percival",
                "Reginald", "Archibald", "Bartholomew", "Cedric", "Montague"
            ],
            // ═══ STEAMPUNK ═══
            Mundo::Steampunk => vec![
                "Gideon", "Isambard", "Nikola", "Orion", "Phineas",
                "Alistair", "Barnaby", "Caspian", "Dorian", "Jasper"
            ],
            _ => vec![
                "Marcus", "David", "James", "Victor", "Adrian", "Carlos",
                "Alex", "Jordan", "Morgan", "Quinn", "River", "Sage"
            ],
        };
        
        let nombres_f = match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "Seraphina", "Lyria", "Isolde", "Elara", "Althea", "Mira", "Vivienne", "Rowena", "Astrid", "Freya", 
                "Helena", "Liora", "Aria", "Briar", "Celia", "Dahlia", "Elowen", "Fiora", "Genevieve", "Hazel", 
                "Iris", "Juniper", "Kaia", "Luna", "Maeve", "Nora", "Ophelia", "Piper", "Quinn", "Rose", "Stella", 
                "Thea", "Una", "Violet", "Willow", "Xanthe", "Yara", "Zara", "Amara", "Beatrix", "Calliope", 
                "Delphine", "Ember", "Faye", "Gaia", "Harper", "Ivy", "Jade", "Kora", "Lila", "Mina", "Nova", 
                "Olive", "Pearl", "Ruby", "Sage", "Tessa", "Ursa", "Vera", "Wren", "Xena", "Yvaine", "Zinnia"
            ],
            Mundo::SciFiCyberpunk | Mundo::SciFiSpace | Mundo::SciFiPostApocaliptico => vec![
                "Nyx", "Nova", "Vesper", "Prism", "Echo", "Siren",
                "Jade", "Storm", "Raven", "Phoenix", "Luna", "Celeste"
            ],
            // ═══ JAPÓN ═══
            Mundo::JaponFeudal | Mundo::AnimeFantasia | Mundo::Anime => vec![
                "Sakura", "Hana", "Yuki", "Akemi", "Michiko",
                "Rei", "Ayame", "Kasumi", "Midori", "Tomoe",
                "Kaede", "Momiji", "Azumi", "Chiyo", "Ran"
            ],
            // ═══ CHINA ═══
            Mundo::ChinaImperial | Mundo::Wuxia => vec![
                "Mei", "Xiu", "Lan", "Hua", "Ling",
                "Yue", "Jing", "Lian", "Fang", "Qing",
                "Xia", "Hong", "Yin", "Zhen", "Yu"
            ],
            // ═══ COREA ═══
            Mundo::CoreaHistorica => vec![
                "Min-ji", "Soo-yeon", "Hye-won", "Eun-bi", "Ji-yeon",
                "Yeon-hee", "Seon-a", "Ha-na", "Bo-ra", "Chae-won"
            ],
            // ═══ MITOLOGÍA ASIÁTICA ═══
            Mundo::MitologiaAsiatica => vec![
                "Amaterasu", "Izanami", "Benzaiten", "Kaguya", "Tamamo",
                "Otohime", "Uzume", "Konohana", "Sengen", "Inari"
            ],
            // ═══ MITOLOGÍA GRIEGA ═══
            Mundo::MitologiaGriega => vec![
                "Kassandra", "Helena", "Xenia", "Ariadne", "Penelope",
                "Elektra", "Thalia", "Daphne", "Athena", "Selene"
            ],
            // ═══ MITOLOGÍA NÓRDICA ═══
            Mundo::MitologiaNordica => vec![
                "Freya", "Astrid", "Sigrid", "Ingrid", "Helga",
                "Thyra", "Ragnhild", "Gudrun", "Brynhild", "Skuld"
            ],
            // ═══ PIRATAS ═══
            Mundo::PiratasCaribe => vec![
                "Anne", "Mary", "Grace", "Charlotte", "Ruby",
                "Scarlett", "Tempest", "Pearl", "Coral", "Marina"
            ],
            // ═══ WESTERN ═══
            Mundo::Western => vec![
                "Calamity", "Belle", "Rose", "Annie", "Jessie",
                "Dakota", "Cheyenne", "Sierra", "Savannah", "Jolene"
            ],
            // ═══ NOIR ═══
            Mundo::Noir => vec![
                "Vera", "Rita", "Gloria", "Vivian", "Mildred",
                "Carmen", "Lola", "Dolores", "Gilda", "Stella"
            ],
            // ═══ VICTORIANO ═══
            Mundo::Victoriano => vec![
                "Arabella", "Cordelia", "Evangeline", "Genevieve", "Henrietta",
                "Josephine", "Millicent", "Ophelia", "Prudence", "Winifred"
            ],
            // ═══ STEAMPUNK ═══
            Mundo::Steampunk => vec![
                "Eliza", "Ada", "Constance", "Beatrix", "Clementine",
                "Dorothea", "Eugenia", "Florence", "Harriet", "Isolde"
            ],
            _ => vec![
                "Elena", "Sarah", "Ana", "Emma", "Maya", "Clara",
                "Sofia", "Isabella", "Victoria", "Natalia", "Olivia", "Amelia"
            ],
        };
        
        let apellidos = match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "Ravencroft", "Shadowend", "Darkhollow", "Stormborn", "Ironwood",
                "Blackthorn", "Ashford", "Nightfall", "Silvermoon", "Frostbourne"
            ],
            Mundo::SciFiCyberpunk | Mundo::SciFiSpace | Mundo::SciFiPostApocaliptico => vec![
                "Chrome", "Voltage", "Nexus", "Cipher", "Protocol",
                "Vector", "Quantum", "Null", "Proxy", "Static"
            ],
            // ═══ JAPÓN ═══
            Mundo::JaponFeudal | Mundo::AnimeFantasia | Mundo::Anime => vec![
                "Takeda", "Uesugi", "Oda", "Tokugawa", "Sanada",
                "Date", "Shimazu", "Mori", "Hojo", "Matsuda"
            ],
            // ═══ CHINA ═══
            Mundo::ChinaImperial | Mundo::Wuxia => vec![
                "Long", "Feng", "Xiao", "Song", "Guo",
                "Wu", "Yang", "Huang", "Zhou", "Sun"
            ],
            // ═══ COREA ═══
            Mundo::CoreaHistorica => vec![
                "Kim", "Park", "Lee", "Choi", "Jung",
                "Kang", "Cho", "Yoon", "Jang", "Han"
            ],
            // ═══ NÓRDICA ═══
            Mundo::MitologiaNordica => vec![
                "Ragnarsson", "Lothbrok", "Ironside", "Boneless", "Sigurdsson",
                "Haraldsson", "Eiriksson", "Bjornsson", "Gunnarsson", "Thorsson"
            ],
            _ => vec![
                "Reyes", "Nakamura", "O'Brien", "Volkov", "Schmidt",
                "Dubois", "Santos", "Kowalski", "Andersson", "Kim"
            ],
        };
        
        let nombre = match genero {
            Genero::Masculino => nombres_m.choose(rng).unwrap(),
            Genero::Femenino => nombres_f.choose(rng).unwrap(),

        };
        
        let apellido = if rng.gen_bool(0.7) {
            Some(apellidos.choose(rng).unwrap().to_string())
        } else { None };
        
        (nombre.to_string(), apellido)
    }
    
    fn generar_titulo(rng: &mut impl Rng) -> Option<String> {
        if rng.gen_bool(0.4) {
            let titulos = [
                "El Silencioso", "Manos Rojas", "El Último de su Nombre",
                "La Sombra", "Corazón de Hierro", "El Errante",
                "Sin Nombre", "El Caído", "La Voz del Trueno",
                "Ojos de Tormenta", "El Roto", "La Esperanza Perdida"
            ];
            Some(titulos.choose(rng).unwrap().to_string())
        } else { None }
    }
    
    fn generar_rasgo(rng: &mut impl Rng) -> String {
        let rasgos = [
            "Ojos que parecen ver más de lo que deberían",
            "Una sonrisa que nunca llega a los ojos",
            "Manos que tiemblan cuando está quieto",
            "Postura militar incluso dormido",
            "Una mirada que hace sentir juzgado"
        ];
        rasgos.choose(rng).unwrap().to_string()
    }
    
    fn generar_vestimenta(rng: &mut impl Rng) -> String {
        let vestimentas = [
            "Ropa práctica, siempre lista para huir",
            "Capas que ocultan su verdadera forma",
            "Vestimenta que contradice su rol social",
            "Algo elegante con rastros de desgaste",
            "Simple, como queriendo pasar desapercibido"
        ];
        vestimentas.choose(rng).unwrap().to_string()
    }
    
    fn generar_voz(rng: &mut impl Rng) -> String {
        let voces = [
            "Grave y medida, cada palabra elegida",
            "Suave pero con un filo que advierte",
            "Cansada, como quien no espera ser escuchado",
            "Intensa, incluso cuando susurra",
            "Cambiante según la audiencia"
        ];
        voces.choose(rng).unwrap().to_string()
    }
    
    fn generar_manierismo(rng: &mut impl Rng) -> String {
        let manierismos = [
            "Cuenta cosas obsesivamente - pasos, latidos",
            "Habla de sí mismo en tercera persona bajo estrés",
            "Repite la última palabra de otros antes de responder",
            "Se toca una cicatriz invisible cuando miente",
            "Cierra los ojos antes de decisiones importantes"
        ];
        manierismos.choose(rng).unwrap().to_string()
    }
    
    fn generar_cicatriz(rng: &mut impl Rng) -> Option<String> {
        if rng.gen_bool(0.5) {
            let cicatrices = [
                "Una quemadura en el antebrazo que oculta",
                "Una línea fina en el cuello",
                "Marcas en los nudillos"
            ];
            Some(cicatrices.choose(rng).unwrap().to_string())
        } else { None }
    }
}
