//! El personaje completo - Alma

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    SietCapas, ForgeConfig, ParametrosGeneracion, 
    Rol, TonoMoral, Mundo, NivelConflicto, Language
};

use super::identidad::Identidad;
use super::arco::ArcoNarrativo;
use super::biografia::{Biografia, MotorBiografia};
use super::skills::{Skill, SoulTier, SkillForge};

/// Un personaje completo con alma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alma {
    pub id: Uuid,
    pub semilla: u64,
    
    // Metadatos narrativos
    pub rol: Rol,
    pub tono_moral: TonoMoral,
    pub mundo: Mundo,
    pub nivel_conflicto: NivelConflicto,
    
    // Identidad externa
    pub identidad: Identidad,
    
    // Las 7 capas psicológicas
    pub capas: SietCapas,
    
    // Arco narrativo potencial
    pub arco: ArcoNarrativo,
    
    // Hooks para historias
    pub ganchos_narrativos: Vec<String>,
    pub momentos_definitorios: Vec<String>,
    
    // Biografía procedural
    pub biografia: Biografia,
    
    // D&D Stats (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ficha_tecnica: Option<super::DndStats>,
    
    // SoulForge Skills System
    #[serde(default)]
    pub soul_tier: Option<SoulTier>,
    #[serde(default)]
    pub skills: Vec<Skill>,
}

impl Alma {
    pub fn generar(rng: &mut impl Rng, params: ParametrosGeneracion, config: &ForgeConfig) -> Self {
        let semilla = params.semilla.unwrap_or_else(|| rng.gen());
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(semilla);
        
        // Determinar parámetros
        let rol = params.rol.unwrap_or_else(|| *Rol::all().choose(&mut rng).unwrap());
        let tono = params.tono_moral.unwrap_or_else(|| *TonoMoral::all().choose(&mut rng).unwrap());
        let mundo = params.mundo.unwrap_or(config.mundo_default);
        let conflicto = params.nivel_conflicto.unwrap_or_else(|| *NivelConflicto::all().choose(&mut rng).unwrap());
        
        let idioma = params.idioma.unwrap_or(Language::Espanol);
        
        // Generar las 7 capas
        let capas = SietCapas::generar(&mut rng, &mundo, &idioma);
        
        // Generar identidad
        let mut identidad = Identidad::generar(&mut rng, &mundo, params.genero, params.edad_fija);
        if let Some(fijo) = &params.nombre_fijo {
            if !fijo.trim().is_empty() {
                identidad.nombre = fijo.clone();
                // Intentar separar apellido si viene en el nombre (Hack simple)
                let partes: Vec<&str> = fijo.split_whitespace().collect();
                if partes.len() > 1 {
                    identidad.apellido = Some(partes[1..].join(" "));
                }
            }
        }

        // GENERAR STATS SI ES ROL JUGADOR
        let mut ficha_tecnica = None;
        if rol == Rol::Jugador {
             let raza = params.raza.unwrap_or_else(|| {
                 use super::Raza;
                 let razas = [Raza::Humano, Raza::Elfo, Raza::Enano, Raza::Halfling, Raza::Dragonborn, Raza::Gnomo, Raza::Tiefling, Raza::Orco];
                 *razas.choose(&mut rng).unwrap()
             });
             ficha_tecnica = Some(Self::generar_stats(&mut rng, raza));
             
             // Update Identity with Race info if needed, e.g. append to description
             identidad.rasgo_distintivo = format!("{} (Raza: {:?})", identidad.rasgo_distintivo, raza);
        }
    
    if idioma != Language::Espanol {
        let lang_code = match idioma { Language::English => "en", Language::Japanese => "jp", _ => "es" };
        let seed = rng.next_u64();
        
        let vest = identidad.vestimenta.clone();
        identidad.vestimenta = super::adapter::adapt_text(&vest, lang_code, seed.wrapping_add(50));
        let rasgo = identidad.rasgo_distintivo.clone();
        identidad.rasgo_distintivo = super::adapter::adapt_text(&rasgo, lang_code, seed.wrapping_add(51));
        let voz = identidad.voz.clone();
        identidad.voz = super::adapter::adapt_text(&voz, lang_code, seed.wrapping_add(52));
        let mani = identidad.manierismo.clone();
        identidad.manierismo = super::adapter::adapt_text(&mani, lang_code, seed.wrapping_add(53));
        
        if let Some(t) = identidad.titulo.take() {
             identidad.titulo = Some(super::adapter::adapt_text(&t, lang_code, seed.wrapping_add(54)));
        }
    }
        
        // Generar arco narrativo
        let mut arco = ArcoNarrativo::generar(&mut rng, &rol, &tono, &capas.mentira);
        
        if idioma != Language::Espanol {
            let lang_code = match idioma { Language::English => "en", Language::Japanese => "jp", _ => "es" };
            let seed = rng.next_u64();
            
            let ei = arco.estado_inicial.clone();
            arco.estado_inicial = super::adapter::adapt_text(&ei, lang_code, seed.wrapping_add(60));
            let pq = arco.punto_de_quiebre.clone();
            arco.punto_de_quiebre = super::adapter::adapt_text(&pq, lang_code, seed.wrapping_add(61));
            let rp = arco.resolucion_positiva.clone();
            arco.resolucion_positiva = super::adapter::adapt_text(&rp, lang_code, seed.wrapping_add(62));
        }
        
        // Generar hooks
        let ganchos = Self::generar_ganchos(&mut rng, &capas, &idioma);
        let momentos = Self::generar_momentos(&mut rng, &capas, &idioma);
        
        // Generar biografía procedural
        let biografia = MotorBiografia::generar(
            &mut rng,
            &identidad.nombre,
            &capas,
            &mundo,
            &rol,
            &tono,
            &idioma,
            Some(identidad.edad)
        );
        
        // ------------------------------------------------------------
        // INTEGRACIÓN SISTEMA DE HABILIDADES SOULFORGE
        // ------------------------------------------------------------
        
        // 1. Determinar Tier por RNG (Gacha/Suerte)
        let tier_roll = rng.gen_range(0.0..100.0);
        let soul_tier = if tier_roll < 0.05 { SoulTier::Mitica }      // 0.05%
                       else if tier_roll < 0.25 { SoulTier::Legendaria } // 0.2%
                       else if tier_roll < 1.5 { SoulTier::Primordial }  // 1.25%
                       else if tier_roll < 5.0 { SoulTier::Ancestral }   // 3.5%
                       else if tier_roll < 13.0 { SoulTier::Alma }       // 8%
                       else if tier_roll < 25.0 { SoulTier::Voz }        // 12%
                       else if tier_roll < 45.0 { SoulTier::Sombra }     // 20%
                       else if tier_roll < 70.0 { SoulTier::Murmullo }   // 25%
                       else { SoulTier::Eco };                           // 30%
                       
        // 2. Determinar Clase (Usar la de ficha técnica si existe, o inferir)
        let clase = if let Some(ref ficha) = ficha_tecnica {
            ficha.clase.clone()
        } else {
            // Inferir clase básica basada en stats o rol si no hay ficha
            match rol {
                Rol::Heroe | Rol::Guardian | Rol::Mercenario => "Guerrero",
                Rol::Villano | Rol::Sombra | Rol::Embaucador => "Pícaro",
                Rol::Mentor | Rol::Profeta => "Mago",
                Rol::Lider | Rol::Rebelde => "Paladín",
                _ => "Aventurero"
            }.to_string()
        };
        
        // 3. Generar Habilidades
        // Extraer un trauma de la herida para la Signature Skill
        let trauma = format!("{} {}", capas.herida.causante, capas.herida.circunstancia);
        
        let skills = SkillForge::generate(
            &mut rng, 
            &clase, 
            soul_tier, 
            identidad.edad, 
            Some(&trauma)
        );

        Self {
            id: Uuid::new_v4(),
            semilla,
            rol,
            tono_moral: tono,
            mundo,
            nivel_conflicto: conflicto,
            identidad,
            capas,
            arco,
            ganchos_narrativos: ganchos,
            momentos_definitorios: momentos,
            biografia,
            ficha_tecnica,
            soul_tier: Some(soul_tier),
            skills,
        }
    }

    fn generar_stats(rng: &mut impl Rng, raza: super::Raza) -> super::DndStats {
        use super::Raza;
        
        // Base Stats (3d6 equivalent ish, standard array shuffled)
        let mut stats = vec![15, 14, 13, 12, 10, 8];
        stats.shuffle(rng);
        
        let mut f = stats[0];
        let mut d = stats[1];
        let mut c = stats[2];
        let mut i = stats[3];
        let mut s = stats[4];
        let mut ch = stats[5];
        
        // Racial Bonuses
        match raza {
            Raza::Humano => { f+=1; d+=1; c+=1; i+=1; s+=1; ch+=1; },
            Raza::Elfo => { d+=2; i+=1; },
            Raza::Enano => { c+=2; f+=2; },
            Raza::Halfling => { d+=2; ch+=1; },
            Raza::Dragonborn => { f+=2; ch+=1; },
            Raza::Gnomo => { i+=2; c+=1; },
            Raza::Tiefling => { ch+=2; i+=1; },
            Raza::Orco => { f+=2; c+=1; },
        };
        
        let hp = 10 + ((c as i32 - 10) / 2).max(0) as u16; 
        
        super::DndStats {
            fuerza: f, destreza: d, constitucion: c, inteligencia: i, sabiduria: s, carisma: ch,
            raza, clase: "Aventurero".to_string(), nivel: 1, hp, ac: 10 + ((d as i32 - 10) / 2).max(0) as u8
        }
    }
    
    fn generar_ganchos(rng: &mut impl Rng, capas: &SietCapas, lang: &Language) -> Vec<String> {
        let mut ganchos = vec![
            format!("Alguien del pasado reaparece con noticias sobre {}", capas.herida.causante),
            format!("Se ve forzado a confrontar: {}", capas.mentira.catalizador_potencial),
            "Una oportunidad demasiado buena para ser verdad".to_string(),
            "Debe elegir entre lo que desea y lo que es correcto".to_string(),
            format!("Descubre la verdad sobre {}", capas.herida.circunstancia),
        ];
        
        // Adaptar si es necesario
        if *lang != Language::Espanol {
             let lang_code = match lang { Language::English => "en", Language::Japanese => "jp", _ => "es" };
             let seed = rng.next_u64();
             ganchos = ganchos.iter().enumerate().map(|(i, g)| {
                 super::adapter::adapt_text(g, lang_code, seed.wrapping_add(i as u64))
             }).collect();
        }

        ganchos.shuffle(rng);
        ganchos.truncate(3);
        ganchos
    }
    
    fn generar_momentos(rng: &mut impl Rng, capas: &SietCapas, lang: &Language) -> Vec<String> {
        let mut momentos = vec![
            format!("Cuando {}", capas.mascara.trigger_que_la_rompe.to_lowercase()),
            format!("Cuando confronta que {}", capas.sombra.rasgo_negado.to_lowercase()),
            "El silencio antes de la confesión más difícil".to_string(),
            "Cuando elige ser vulnerable ante quien podría destruirlo".to_string(),
            "El instante donde decide quién quiere ser".to_string(),
        ];
        
        // Adaptar si es necesario
        if *lang != Language::Espanol {
             let lang_code = match lang { Language::English => "en", Language::Japanese => "jp", _ => "es" };
             let seed = rng.next_u64();
             momentos = momentos.iter().enumerate().map(|(i, m)| {
                 super::adapter::adapt_text(m, lang_code, seed.wrapping_add(100 + i as u64))
             }).collect();
        }

        momentos.shuffle(rng);
        momentos.truncate(3);
        momentos
    }
    
    /// Genera una narrativa legible del personaje
    pub fn narrar(&self) -> String {
        let mut n = String::new();
        
        n.push_str(&format!("═══════════════════════════════════════════════════════════════\n"));
        n.push_str(&format!("                    {}\n", self.identidad.nombre.to_uppercase()));
        if let Some(ref titulo) = self.identidad.titulo {
            n.push_str(&format!("                    \"{}\"\n", titulo));
        }
        n.push_str(&format!("═══════════════════════════════════════════════════════════════\n\n"));
        
        // Primera impresión
        n.push_str("【 PRIMERA IMPRESIÓN 】\n");
        n.push_str(&format!("{}\n", self.identidad.rasgo_distintivo));
        n.push_str(&format!("Viste: {}\n", self.identidad.vestimenta));
        n.push_str(&format!("Su voz: {}\n", self.identidad.voz));
        n.push_str(&format!("Nota particular: {}\n\n", self.identidad.manierismo));
        
        // Lo que muestra
        n.push_str("【 LO QUE MUESTRA AL MUNDO 】\n");
        n.push_str(&format!("{}\n", self.capas.mascara.comportamiento_publico));
        n.push_str(&format!("Quiere ser visto como: {}\n", self.capas.mascara.imagen_proyectada));
        n.push_str(&format!("Frase típica: \"{}\"\n\n", self.capas.mascara.frase_tipica));
        
        // El pasado
        n.push_str("【 EL PASADO QUE CARGA 】\n");
        n.push_str(&format!("{} {}.\n", self.capas.herida.causante, self.capas.herida.circunstancia));
        n.push_str(&format!("Lo que quedó: {}\n\n", self.capas.herida.como_lo_cambio));
        
        // Lo oculto
        n.push_str("【 LO QUE OCULTA 】\n");
        n.push_str(&format!("Siente: {}\n", self.capas.mascara.sentimiento_oculto));
        n.push_str(&format!("Su mayor miedo: {}\n", self.capas.mascara.miedo_central));
        n.push_str(&format!("Deseo secreto: {}\n\n", self.capas.mascara.deseo_secreto));
        
        // Conflicto central
        n.push_str("【 EL CONFLICTO CENTRAL 】\n");
        n.push_str(&format!("QUIERE: {}\n", self.capas.deseo_necesidad.deseo_consciente));
        n.push_str(&format!("Porque: {}\n", self.capas.deseo_necesidad.motivacion_del_deseo));
        n.push_str(&format!("\nNECESITA: {}\n", self.capas.deseo_necesidad.necesidad_real));
        n.push_str(&format!("No lo ve porque: {}\n\n", self.capas.deseo_necesidad.por_que_no_la_ve));
        
        // La mentira
        n.push_str("【 LA MENTIRA QUE SE CUENTA 】\n");
        n.push_str(&format!("\"{}\"\n", self.capas.mentira.la_mentira));
        n.push_str(&format!("Esto causa: {}\n", self.capas.mentira.decisiones_que_causa));
        n.push_str(&format!("Verdad que necesita: {}\n\n", self.capas.mentira.verdad_necesaria));
        
        // Arco
        n.push_str("【 ARCO NARRATIVO POTENCIAL 】\n");
        n.push_str(&format!("Comienza: {}\n", self.arco.estado_inicial));
        n.push_str(&format!("Punto de quiebre: {}\n", self.arco.punto_de_quiebre));
        n.push_str(&format!("\n✧ Si triunfa: {}\n", self.arco.resolucion_positiva));
        n.push_str(&format!("✧ Si falla: {}\n\n", self.arco.resolucion_tragica));
        
        // Ganchos
        n.push_str("【 GANCHOS NARRATIVOS 】\n");
        for g in &self.ganchos_narrativos {
            n.push_str(&format!("• {}\n", g));
        }
        
        n.push_str("\n【 MOMENTOS DEFINITORIOS 】\n");
        for m in &self.momentos_definitorios {
            n.push_str(&format!("• {}\n", m));
        }
        
        n.push_str(&format!("\n═══════════════════════════════════════════════════════════════\n"));
        n.push_str(&format!("Semilla: {} | Rol: {:?} | Tono: {:?} | Mundo: {:?}\n", 
            self.semilla, self.rol, self.tono_moral, self.mundo));
        
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use crate::core::{ForgeConfig, Language, Genero, Raza};

    #[test]
    fn test_alma_generation_with_skills() {
        let semilla = 12345;
        let mut rng = StdRng::seed_from_u64(semilla);
        let config = ForgeConfig::default();
        
        let params = ParametrosGeneracion {
            nombre_fijo: Some("TestHero".to_string()),
            genero: Some(Genero::Masculino),
            rol: Some(Rol::Jugador), // Use Jugador to ensure D&D stats/class generation
            // clase field does not exist in params, it is derived
            raza: Some(Raza::Humano),
            edad_fija: Some(25),
            idioma: Some(Language::Espanol),
            ..Default::default() 
        };

        // Pass config reference instead of seed
        let alma = Alma::generar(&mut rng, params, &config);

        println!("--- TEST GENERATION ---");
        println!("Nombre: {}", alma.identidad.nombre);
        println!("Tier: {:?}", alma.soul_tier);
        
        if let Some(tier) = &alma.soul_tier {
             println!("Tier Description: {:?}", tier);
        }

        println!("Habilidades Generadas: {}", alma.skills.len());
        for skill in &alma.skills {
            println!("- [{:?}] {} (Poder: {})", skill.category, skill.name, skill.power_level);
        }
        
        assert!(alma.soul_tier.is_some(), "El SoulTier debería generarse");
        assert!(!alma.skills.is_empty(), "Debería tener habilidades");
    }
}
