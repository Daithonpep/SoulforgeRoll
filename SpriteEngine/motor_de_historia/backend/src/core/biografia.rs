//! Sistema de Biografías Procedurales Entrelazadas v2.0
//! 
//! Genera historias balanceadas con luz y sombra.
//! Las biografías se entrelazan cuando hay conexiones.

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::capas::{SietCapas, TipoHerida};
use super::{Mundo, Rol, TonoMoral, Language};

// ============================================================
// SISTEMA DE TONALIDAD - Balance luz/sombra
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tonalidad {
    Radiante,    // Alegría pura, victoria, amor
    Calido,      // Calidez, conexión, esperanza  
    Balanceado,  // Neutral, cotidiano, calma
    Melancolico, // Tristeza suave, nostalgia
    Sombrio,     // Dolor, pérdida, conflicto
}

/// Fases de vida
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaseVida {
    Origen,
    Infancia,
    Catalizador,
    Transformacion,
    Madurez,
    Legado,
    Crepusculo,
    Presente,
}

impl FaseVida {
    pub fn all() -> Vec<FaseVida> {
        vec![
            FaseVida::Origen,
            FaseVida::Infancia,
            FaseVida::Catalizador,
            FaseVida::Transformacion,
            FaseVida::Madurez,
            FaseVida::Legado,
            FaseVida::Crepusculo,
            FaseVida::Presente,
        ]
    }
    
    pub fn titulo(&self) -> &'static str {
        match self {
            FaseVida::Origen => "Los Primeros Días",
            FaseVida::Infancia => "Años Formativos", 
            FaseVida::Catalizador => "El Momento que Todo Cambió",
            FaseVida::Transformacion => "Renacimiento",
            FaseVida::Madurez => "El Peso de los Años",
            FaseVida::Legado => "Lo que Queda",
            FaseVida::Crepusculo => "El Último Horizonte",
            FaseVida::Presente => "Quien Es Ahora",
        }
    }
}

// ============================================================
// MOMENTOS DE GRACIA - La luz que da peso a la sombra
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentoGracia {
    pub nombre: String,
    pub descripcion: String,
    pub regalo: String,  // Qué le dio este momento
    pub eco: String,     // Cómo resuena en el presente
}

fn generar_momentos_gracia(rng: &mut impl Rng, nombre: &str, herida: &TipoHerida) -> Vec<MomentoGracia> {
    use super::gramatica::*;
    
    let mut momentos = Vec::new();
    
    // Momento principal basado en contraste con la herida
    let categoria_principal = match herida {
        TipoHerida::Abandono | TipoHerida::Traicion => "conexion",
        TipoHerida::Humillacion | TipoHerida::Impotencia => "logro",
        TipoHerida::Culpa => "bondad",
        _ => "general",
    };
    
    let (nom, desc, regalo, eco) = generar_momento_gracia(rng, nombre, categoria_principal);
    momentos.push(MomentoGracia { nombre: nom, descripcion: desc, regalo, eco });
    
    // Segundo momento de categoría diferente
    let categorias = ["conexion", "logro", "bondad", "general"];
    let segunda_cat = categorias.choose(rng).unwrap();
    let (nom2, desc2, regalo2, eco2) = generar_momento_gracia(rng, nombre, segunda_cat);
    momentos.push(MomentoGracia { nombre: nom2, descripcion: desc2, regalo: regalo2, eco: eco2 });
    
    // Tercer momento: pequeña alegría convertida en momento
    let alegria = generar_pequena_alegria(rng);
    let esperanza = generar_esperanza(rng);
    momentos.push(MomentoGracia {
        nombre: "Un Destello de Luz".to_string(),
        descripcion: format!("{} encuentra paz en {}", nombre, alegria),
        regalo: "Un recordatorio de que hay belleza en lo simple".to_string(),
        eco: format!("Lo impulsa a {}", esperanza),
    });
    
    momentos
}

// ============================================================
// CONFLICTOS INTERNOS - Más allá del bien vs mal
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictoInterno {
    pub nombre: String,
    pub polo_a: String,
    pub polo_b: String,
    pub descripcion: String,
    pub origen: String,
}

fn generar_conflictos(rng: &mut impl Rng, nombre: &str, capas: &SietCapas) -> Vec<ConflictoInterno> {
    let mut pool = Vec::new();

    // 1. Deber vs Deseo
    let desc_deber = if rng.gen_bool(0.5) {
        format!("{} vive en la tensión entre cumplir y querer. Cada día es una negociación silenciosa.", nombre)
    } else {
        format!("Para {}, el deber es un ancla pesada; el deseo, una marea que intenta arrastrarlo.", nombre)
    };
    pool.push(ConflictoInterno {
        nombre: "Deber vs Deseo".into(),
        polo_a: "lo que debe a otros".into(),
        polo_b: "lo que anhela para sí".into(),
        descripcion: desc_deber,
        origen: "Aprendió temprano que sus deseos venían después".into(),
    });

    // 2. Pasado vs Futuro
    let desc_pasado = if rng.gen_bool(0.5) {
        format!("El pasado de {} tiene gravedad propia. Cada paso hacia adelante requiere soltar algo.", nombre)
    } else {
        format!("{} camina hacia adelante mirando hacia atrás. El ayer nunca termina de irse.", nombre)
    };
    pool.push(ConflictoInterno {
        nombre: "Pasado vs Futuro".into(),
        polo_a: "quien fue formado para ser".into(),
        polo_b: "quien elige convertirse".into(),
        descripcion: desc_pasado,
        origen: "Las expectativas de otros aún resuenan".into(),
    });

    // 3. Confianza vs Protección
    let desc_conf = if rng.gen_bool(0.5) {
        format!("{} oscila entre la necesidad de conexión y el instinto visceral de protegerse.", nombre)
    } else {
        format!("Bajar la guardia es peligroso. Mantenerla arriba es solitario. {} conoce ambos dolores.", nombre)
    };
    pool.push(ConflictoInterno {
        nombre: "Confianza vs Protección".into(),
        polo_a: "abrirse a otros".into(),
        polo_b: "mantener las murallas".into(),
        descripcion: desc_conf,
        origen: "La vulnerabilidad tuvo consecuencias antes".into(),
    });

    // 4. Máscara vs Yo
    let desc_mask = if rng.gen_bool(0.5) {
        format!("{} ha perfeccionado su máscara. Funciona tan bien que a veces olvida qué hay debajo.", nombre)
    } else {
        format!("Hay dos versiones de {}: la que todos ven y la que nadie conoce. La brecha entre ambas crece.", nombre)
    };
    pool.push(ConflictoInterno {
        nombre: "Máscara vs Yo".into(),
        polo_a: "la persona que muestra al mundo".into(),
        polo_b: "quien realmente es".into(),
        descripcion: desc_mask,
        origen: "El yo real fue rechazado alguna vez".into(),
    });

    // 5. Seguridad vs Crecimiento (Variaciones para evitar 'jaula de oro' siempre)
    let desc_seg = match rng.gen_range(0..3) {
        0 => format!("{} conoce los límites de su zona segura. A veces la jaula es dorada, pero sigue siendo jaula.", nombre),
        1 => format!("Crecer duele. Quedarse quieto asfixia. {} está decidiendo qué dolor prefiere soportar.", nombre),
        _ => format!("El horizonte llama a {}, pero el suelo conocido sujeta sus pies con fuerza.", nombre),
    };
    pool.push(ConflictoInterno {
        nombre: "Seguridad vs Crecimiento".into(),
        polo_a: "la comodidad de lo conocido".into(),
        polo_b: "el riesgo necesario para cambiar".into(),
        descripcion: desc_seg,
        origen: "El cambio trajo dolor antes".into(),
    });

    // 6. Aferrarse vs Soltar
    let desc_soltar = if rng.gen_bool(0.5) {
        format!("Hay cosas que {} no puede soltar aún. El agarre es doloroso, pero soltar parece peor.", nombre)
    } else {
        format!("{} lleva equipaje extra. Recuerdos, rencores, objetos. Soltarlos se siente como perderse.", nombre)
    };
    pool.push(ConflictoInterno {
        nombre: "Aferrarse vs Soltar".into(),
        polo_a: "lo que fue".into(),
        polo_b: "lo que podría ser".into(),
        descripcion: desc_soltar,
        origen: "Perder algo importante enseñó a no soltar nada".into(),
    });
    
    // 7. Justicia vs Piedad (NUEVO)
    pool.push(ConflictoInterno {
        nombre: "Justicia vs Piedad".into(),
        polo_a: "el castigo merecido".into(),
        polo_b: "la compasión humana".into(),
        descripcion: format!("{} ve el mundo en blanco y negro, pero su corazón a veces ve matices que la ley ignora.", nombre),
        origen: "Vio una injusticia quedar impune".into(),
    });
    
    // 8. Libertad vs Pertenencia (NUEVO)
    pool.push(ConflictoInterno {
        nombre: "Libertad vs Pertenencia".into(),
        polo_a: "ser libre sin ataduras".into(),
        polo_b: "tener un lugar y gente".into(),
        descripcion: format!("La soledad es el precio de la libertad de {}. ¿Vale la pena pagarlo?", nombre),
        origen: "Pertenecer significó perderse a sí mismo".into(),
    });

    // Elegir 2 conflictos (ahora de un pool de 8 con textos variables)
    let mut seleccionados: Vec<ConflictoInterno> = pool.choose_multiple(rng, 2).cloned().collect();

    // Personalizar el origen según la herida
    for conflicto in &mut seleccionados {
        conflicto.origen = match &capas.herida.tipo {
            TipoHerida::Abandono => format!("{} porque alguien importante se fue", conflicto.origen),
            TipoHerida::Traicion => format!("{} cuando la confianza fue rota", conflicto.origen),
            TipoHerida::Humillacion => format!("{} después de ser expuesto", conflicto.origen),
            _ => conflicto.origen.clone(),
        };
    }

    seleccionados
}

// ============================================================
// BIOGRAFÍA COMPLETA
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biografia {
    pub fases: Vec<FragmentoBiografia>,
    pub momentos_gracia: Vec<MomentoGracia>,
    pub conflictos: Vec<ConflictoInterno>,
    pub texto_completo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentoBiografia {
    pub fase: FaseVida,
    pub titulo: String,
    pub contenido: String,
    pub tonalidad: Tonalidad,
}

// ============================================================
// ESTRUCTURAS DE VARIABILIDAD NARRATIVA
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EstiloNarrativo {
    Cronista,    // Secuencial: Infancia -> Evento -> Actualidad
    InMediaRes,  // Impacto: Evento Traumático -> Contexto -> Resolución
    Psicologico, // Introspectivo: Herida/Mentira -> Hechos -> Estado Mental
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoAncla {
    pub nombre: String,
    pub anio: String,
    pub descripcion: String,
    pub impacto_global: String,
}

// ============================================================
// MOTOR DE BIOGRAFÍA v2.0
// ============================================================

pub struct MotorBiografia;

impl MotorBiografia {
    pub fn generar(
        rng: &mut impl Rng,
        nombre: &str,
        capas: &SietCapas,
        mundo: &Mundo,
        rol: &Rol,
        tono: &TonoMoral,
        lang: &Language,
        edad_opt: Option<u32>
    ) -> Biografia {
        // 1. Determinar Estilo Narrativo
        let estilos = vec![EstiloNarrativo::Cronista, EstiloNarrativo::InMediaRes, EstiloNarrativo::Psicologico];
        let estilo = estilos.choose(rng).unwrap_or(&EstiloNarrativo::Cronista).clone();
        
        // 2. Generar contenido base
        let balance = Self::calcular_balance(rol);
        let tonalidad_origen = Self::seleccionar_tonalidad(rng, &balance, 0);
        let tonalidad_climax = Self::seleccionar_tonalidad(rng, &balance, 3);
        let tonalidad_actual = Self::seleccionar_tonalidad(rng, &balance, 4);

        let momentos_gracia = generar_momentos_gracia(rng, nombre, &capas.herida.tipo);
        let conflictos = generar_conflictos(rng, nombre, capas);
        
        let txt_origen = Self::origen(rng, nombre, mundo, tonalidad_origen, lang);
        let txt_catalizador = Self::catalizador(rng, nombre, capas, Tonalidad::Sombrio, lang);
        let txt_transformacion = Self::transformacion(rng, nombre, capas, rol, tonalidad_climax, &momentos_gracia, lang);
        let txt_presente = Self::presente(rng, nombre, capas, tonalidad_actual, lang);
        let txt_infancia = Self::infancia(rng, nombre, capas, Tonalidad::Melancolico, &momentos_gracia, lang);

        // Generar fases extra por edad
        let edad = edad_opt.unwrap_or(25);
        let mut fases_extra = Vec::new();
        
        if edad >= 30 {
            let txt_madurez = Self::madurez(rng, nombre, edad, lang);
            fases_extra.push(FragmentoBiografia { fase: FaseVida::Madurez, titulo: "V. La Madurez".into(), contenido: txt_madurez, tonalidad: Tonalidad::Balanceado });
        }
        if edad >= 50 {
            let txt_legado = Self::legado(rng, nombre, edad, lang);
            fases_extra.push(FragmentoBiografia { fase: FaseVida::Legado, titulo: "VI. El Legado".into(), contenido: txt_legado, tonalidad: Tonalidad::Calido });
        }
        if edad >= 70 {
            let txt_crepusculo = Self::crepusculo(rng, nombre, edad, lang);
            fases_extra.push(FragmentoBiografia { fase: FaseVida::Crepusculo, titulo: "VII. El Crepúsculo".into(), contenido: txt_crepusculo, tonalidad: Tonalidad::Melancolico });
        }

        // 3. Ensamblar Fases
        let fases = match estilo {
            EstiloNarrativo::Cronista => {
                let mut f = vec![
                    FragmentoBiografia { fase: FaseVida::Origen, titulo: "I. El Origen".into(), contenido: txt_origen, tonalidad: tonalidad_origen },
                    FragmentoBiografia { fase: FaseVida::Infancia, titulo: "II. Años Formativos".into(), contenido: txt_infancia, tonalidad: Tonalidad::Melancolico },
                    FragmentoBiografia { fase: FaseVida::Catalizador, titulo: "III. El Quiebre".into(), contenido: txt_catalizador, tonalidad: Tonalidad::Sombrio },
                    FragmentoBiografia { fase: FaseVida::Transformacion, titulo: "IV. La Metamorfosis".into(), contenido: txt_transformacion, tonalidad: tonalidad_climax },
                ];
                // Insertar fases extra antes del presente
                f.append(&mut fases_extra);
                f.push(FragmentoBiografia { fase: FaseVida::Presente, titulo: "El Ahora".into(), contenido: txt_presente, tonalidad: tonalidad_actual });
                f
            },
            EstiloNarrativo::InMediaRes => {
                let mut f = vec![
                    FragmentoBiografia { fase: FaseVida::Catalizador, titulo: "⚡ EL MOMENTO CERO".into(), contenido: format!("{}. Antes de eso, la vida de {} era otra historia.", txt_catalizador, nombre), tonalidad: Tonalidad::Sombrio },
                    FragmentoBiografia { fase: FaseVida::Origen, titulo: "⏪ Flashback: El Origen".into(), contenido: format!("Para entender el dolor, hay que mirar al principio. {}", txt_origen), tonalidad: tonalidad_origen },
                    FragmentoBiografia { fase: FaseVida::Transformacion, titulo: "▶️ La Secuela".into(), contenido: txt_transformacion, tonalidad: tonalidad_climax },
                ];
                // En in media res añadimos solo si es muy viejo para dar contexto
                if edad >= 50 {
                    f.append(&mut fases_extra);
                }
                f.push(FragmentoBiografia { fase: FaseVida::Presente, titulo: "El Presente".into(), contenido: txt_presente, tonalidad: tonalidad_actual });
                f
            },
            EstiloNarrativo::Psicologico => vec![
                FragmentoBiografia { fase: FaseVida::Presente, titulo: "🧠 Estado Mental".into(), contenido: format!("{} vive atrapado en una premisa: '{}'.", nombre, capas.mentira.la_mentira), tonalidad: Tonalidad::Melancolico },
                FragmentoBiografia { fase: FaseVida::Catalizador, titulo: "La Herida Primaria".into(), contenido: format!("La raíz no está en los hechos, sino en el impacto. {}", txt_catalizador), tonalidad: Tonalidad::Sombrio },
                FragmentoBiografia { fase: FaseVida::Transformacion, titulo: "Mecanismos de Defensa".into(), contenido: format!("Para sobrevivir, {} construyó una armadura. {}", nombre, txt_transformacion), tonalidad: tonalidad_climax },
                FragmentoBiografia { fase: FaseVida::Presente, titulo: "La Realidad Externa".into(), contenido: txt_presente, tonalidad: tonalidad_actual },
            ],
        };
        
        // Construir texto completo
        let mut partes: Vec<String> = fases.iter()
            .map(|f| format!("**{}**\n\n{}", f.titulo, f.contenido))
            .collect();
            
        let estilo_str = match estilo {
            EstiloNarrativo::Cronista => "Estilo: Crónica Lineal",
            EstiloNarrativo::InMediaRes => "Estilo: In Media Res (Fracturado)",
            EstiloNarrativo::Psicologico => "Estilo: Psicológico (Introspectivo)",
        };
        
        if !conflictos.is_empty() {
             let mut seccion_conflictos = String::from("\n**Conflictos Latentes:**\n");
             for c in &conflictos {
                 seccion_conflictos.push_str(&format!("\n• *{}*: {}\n", c.nombre, c.descripcion));
             }
             partes.push(seccion_conflictos);
        }

        let texto_completo = format!("_{}_\n\n{}", estilo_str, partes.join("\n\n---\n\n"));
        
        // 4. ADAPTACIÓN DE IDIOMA
        let mut fases_finales = fases;
        let mut momentos_finales = momentos_gracia;
        let mut conflictos_finales = conflictos;
        
        if *lang != Language::Espanol {
            let lang_code = match lang { Language::English => "en", Language::Japanese => "jp", _ => "es" };
            let seed = rng.next_u64();
            
            // Adaptar Fases
            for (i, fase) in fases_finales.iter_mut().enumerate() {
                fase.titulo = super::adapter::adapt_text(&fase.titulo, lang_code, seed.wrapping_add(i as u64));
                fase.contenido = super::adapter::adapt_text(&fase.contenido, lang_code, seed.wrapping_add(100 + i as u64));
            }
            
            // Adaptar Momentos
            for (i, m) in momentos_finales.iter_mut().enumerate() {
                m.nombre = super::adapter::adapt_text(&m.nombre, lang_code, seed.wrapping_add(200 + i as u64));
                m.descripcion = super::adapter::adapt_text(&m.descripcion, lang_code, seed.wrapping_add(300 + i as u64));
                m.regalo = super::adapter::adapt_text(&m.regalo, lang_code, seed.wrapping_add(400 + i as u64));
                m.eco = super::adapter::adapt_text(&m.eco, lang_code, seed.wrapping_add(500 + i as u64));
            }

            // Adaptar Conflictos
            for (i, c) in conflictos_finales.iter_mut().enumerate() {
                c.nombre = super::adapter::adapt_text(&c.nombre, lang_code, seed.wrapping_add(600 + i as u64));
                c.polo_a = super::adapter::adapt_text(&c.polo_a, lang_code, seed.wrapping_add(700 + i as u64));
                c.polo_b = super::adapter::adapt_text(&c.polo_b, lang_code, seed.wrapping_add(800 + i as u64));
                c.descripcion = super::adapter::adapt_text(&c.descripcion, lang_code, seed.wrapping_add(900 + i as u64));
                c.origen = super::adapter::adapt_text(&c.origen, lang_code, seed.wrapping_add(1000 + i as u64));
            }
        }

        Biografia {
            fases: fases_finales,
            momentos_gracia: momentos_finales,
            conflictos: conflictos_finales,
            texto_completo,
        }
    }
    
    fn calcular_balance(rol: &Rol) -> [f32; 5] {
        // [Radiante, Cálido, Balanceado, Melancólico, Sombrío]
        match rol {
            Rol::Heroe => [0.15, 0.30, 0.30, 0.20, 0.05],
            Rol::Villano => [0.05, 0.15, 0.25, 0.30, 0.25],
            Rol::Mentor => [0.20, 0.35, 0.25, 0.15, 0.05],
            Rol::Aliado => [0.20, 0.35, 0.30, 0.10, 0.05],
            _ => [0.10, 0.25, 0.30, 0.25, 0.10], // Balanceado por defecto
        }
    }
    
    fn seleccionar_tonalidad(rng: &mut impl Rng, balance: &[f32; 5], fase_idx: usize) -> Tonalidad {
        // Ajustar balance según la fase
        let mut ajustado = *balance;
        match fase_idx {
            0 => { ajustado[1] += 0.1; ajustado[4] -= 0.05; }, // Origen más cálido
            1 => { ajustado[0] += 0.1; ajustado[3] -= 0.05; }, // Infancia más radiante
            2 => { ajustado[3] += 0.15; ajustado[0] -= 0.1; }, // Catalizador más melancólico
            3 => { ajustado[2] += 0.1; }, // Transformación balanceada
            4 => { }, // Presente según el balance base
            _ => {},
        }
        
        // Normalizar
        let suma: f32 = ajustado.iter().sum();
        for v in &mut ajustado {
            *v /= suma;
        }
        
        let r: f32 = rng.gen();
        let mut acum = 0.0;
        for (i, peso) in ajustado.iter().enumerate() {
            acum += peso;
            if r < acum {
                return match i {
                    0 => Tonalidad::Radiante,
                    1 => Tonalidad::Calido,
                    2 => Tonalidad::Balanceado,
                    3 => Tonalidad::Melancolico,
                    _ => Tonalidad::Sombrio,
                };
            }
        }
        Tonalidad::Balanceado
    }
    
    #[allow(dead_code)]
    #[allow(dead_code)]
    fn generar_fase_con_tonalidad(
        rng: &mut impl Rng,
        nombre: &str,
        capas: &SietCapas,
        mundo: &Mundo,
        rol: &Rol,
        fase: FaseVida,
        tonalidad: Tonalidad,
        momentos_gracia: &[MomentoGracia],
        lang: &Language,
    ) -> String {
        let texto_base = match fase {
            FaseVida::Origen => Self::origen(rng, nombre, mundo, tonalidad, lang),
            FaseVida::Infancia => Self::infancia(rng, nombre, capas, tonalidad, momentos_gracia, lang),
            FaseVida::Catalizador => Self::catalizador(rng, nombre, capas, tonalidad, lang),
            FaseVida::Transformacion => Self::transformacion(rng, nombre, capas, rol, tonalidad, momentos_gracia, lang),
            FaseVida::Presente => Self::presente(rng, nombre, capas, tonalidad, lang),
            FaseVida::Madurez => Self::madurez(rng, nombre, 40, lang),
            FaseVida::Legado => Self::legado(rng, nombre, 60, lang),
            FaseVida::Crepusculo => Self::crepusculo(rng, nombre, 80, lang),
        };
        
        texto_base
    }
    
    fn origen(rng: &mut impl Rng, nombre: &str, mundo: &Mundo, tonalidad: Tonalidad, lang: &Language) -> String {
        let lugares = Self::lugares_origen(mundo);
        let lugar = lugares.choose(rng).unwrap();
        
        let sufijo = match tonalidad {
            Tonalidad::Radiante => "radiante",
            Tonalidad::Calido => "calido",
            Tonalidad::Balanceado => "balanceado",
            Tonalidad::Melancolico => "melancolico",
            Tonalidad::Sombrio => "sombrio",
        };
        let key = format!("origen_{}", sufijo);
        crate::core::narrativa::BancoNarrativo::obtener(&key, lang, &[nombre, lugar], rng)
    }
    
    fn infancia(rng: &mut impl Rng, nombre: &str, _capas: &SietCapas, tonalidad: Tonalidad, _momentos_gracia: &[MomentoGracia], lang: &Language) -> String {
        let refugios = vec!["los libros", "la música", "un refugio secreto", "la soledad", "la naturaleza"];
        let refugio = refugios.choose(rng).unwrap();
        
        let sufijo = match tonalidad {
            Tonalidad::Radiante => "radiante",
            Tonalidad::Calido => "calido",
            Tonalidad::Balanceado => "balanceado",
            Tonalidad::Melancolico => "melancolico",
            Tonalidad::Sombrio => "sombrio",
        };
        let key = format!("infancia_{}", sufijo);
        crate::core::narrativa::BancoNarrativo::obtener(&key, lang, &[nombre, refugio], rng)
    }
    
    fn catalizador(rng: &mut impl Rng, nombre: &str, capas: &SietCapas, _tonalidad: Tonalidad, lang: &Language) -> String {
        let evento = format!("{} {}", capas.herida.causante, capas.herida.circunstancia);
        let key = "catalizador_sombrio";
        crate::core::narrativa::BancoNarrativo::obtener(key, lang, &[nombre, &evento], rng)
    }
    
    fn transformacion(rng: &mut impl Rng, nombre: &str, _capas: &SietCapas, _rol: &Rol, tonalidad: Tonalidad, _momentos_gracia: &[MomentoGracia], lang: &Language) -> String {
        let sufijo = match tonalidad {
            Tonalidad::Radiante => "radiante",
            Tonalidad::Melancolico => "melancolico",
             _ => "radiante", 
        };
        let key = format!("transformacion_{}", sufijo);
        crate::core::narrativa::BancoNarrativo::obtener(&key, lang, &[nombre], rng)
    }
    
    fn presente(rng: &mut impl Rng, nombre: &str, _capas: &SietCapas, _tonalidad: Tonalidad, lang: &Language) -> String {
        // Usamos una clave genérica del banco (reutilizando transformación por ahora para asegurar traducción)
        crate::core::narrativa::BancoNarrativo::obtener("transformacion_radiante", lang, &[nombre], rng)
    }
    
    fn lugares_origen(mundo: &Mundo) -> Vec<&'static str> {
        match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "un pueblo olvidado por los mapas",
                "las sombras de una fortaleza en ruinas",
                "los márgenes de un reino indiferente",
                "una aldea donde el tiempo parecía detenido",
                "las tierras salvajes más allá de la frontera",
                "un feudo donde la ley era capricho del señor",
                "un monasterio aislado entre montañas",
                "una ciudad portuaria llena de secretos",
            ],
            Mundo::SciFiCyberpunk => vec![
                "los niveles inferiores de la megaciudad",
                "un sector donde la luz del sol era un rumor",
                "las zonas libres donde las corporaciones no llegaban",
                "un complejo residencial donde miles vivían hacinados",
                "los márgenes digitales de una sociedad hiperconectada",
                "una estación orbital en declive",
            ],
            Mundo::SciFiSpace => vec![
                "una estación orbital en los confines del sistema",
                "una colonia minera donde nada crecía naturalmente",
                "una nave generacional que nadie recordaba de dónde partió",
                "un mundo terraformado a medias",
                "los asteroides del cinturón exterior",
            ],
            // ═══ MUNDOS ASIÁTICOS ═══
            Mundo::JaponFeudal => vec![
                "una aldea de montaña donde los samurai rara vez pasaban",
                "las tierras de un daimyō caído en desgracia",
                "un templo sintoísta olvidado entre bambúes",
                "los barrios bajos de una ciudad castillo",
                "una isla pesquera donde llegaban rumores de guerra",
                "un camino de la seda interior recorrido por ronin",
                "las sombras del monte sagrado",
                "un dojo abandonado que guardaba secretos antiguos",
            ],
            Mundo::ChinaImperial | Mundo::Wuxia => vec![
                "una provincia remota donde el emperador era solo un nombre",
                "los muelles del gran río, entre comerciantes y contrabandistas",
                "un monasterio de artes marciales en las montañas de niebla",
                "las calles de una ciudad amurallada llena de intriga",
                "una villa de eruditos donde la poesía pesaba más que el oro",
                "los bosques de bambú donde vivían los proscritos",
                "una casa de té que escondía una secta secreta",
                "las rutas de la seda, entre caravanas y bandidos",
            ],
            Mundo::CoreaHistorica => vec![
                "una aldea de campesinos bajo la sombra de la nobleza yangban",
                "los salones de una familia aristocrática en declive",
                "un templo budista en las montañas del norte",
                "las calles de la capital, donde intrigas y poesía se mezclaban",
                "una costa azotada por piratas japoneses",
                "un pueblo de artesanos de cerámica",
            ],
            Mundo::MitologiaAsiatica | Mundo::AnimeFantasia => vec![
                "un mundo entre mundos, donde los espíritus caminaban",
                "una academia de habilidades extraordinarias",
                "un reino flotante sobre las nubes",
                "las ruinas de una civilización olvidada",
                "un santuario donde lo imposible era cotidiano",
                "el cruce de caminos entre lo mortal y lo divino",
                "una isla que aparecía solo en noches de luna llena",
            ],
            // ═══ OTROS MUNDOS ═══
            Mundo::Victoriano => vec![
                "los barrios industriales de una ciudad de humo y niebla",
                "una mansión decadente en las afueras",
                "los callejones donde la ley no llegaba",
                "un orfanato donde se forjaban destinos duros",
                "los salones de una sociedad obsesionada con las apariencias",
                "un puerto donde llegaban secretos de las colonias",
            ],
            Mundo::MitologiaGriega => vec![
                "una isla del mar Egeo bendecida por los dioses",
                "las laderas del monte Olimpo, donde lo divino rozaba lo mortal",
                "una polis donde la filosofía y la guerra eran gemelas",
                "un templo consagrado a una deidad caprichosa",
                "los caminos que conectaban el mundo conocido",
            ],
            Mundo::MitologiaNordica => vec![
                "un fiordo donde el hielo nunca terminaba de derretirse",
                "una aldea de vikingos que soñaban con gloria",
                "las raíces del gran fresno que sostenía los mundos",
                "un bosque donde los lobos hablaban con ancianos",
                "las costas que veían partir a los drakkars",
            ],
            Mundo::PiratasCaribe => vec![
                "una isla sin ley donde los proscritos eran reyes",
                "un puerto donde cada barco traía historias y peligros",
                "las bodegas de un galeón mercante",
                "una plantación donde la justicia era un sueño lejano",
                "las aguas turquesas que escondían naufragios y tesoros",
            ],
            Mundo::Western => vec![
                "un pueblo fronterizo donde la ley era el más rápido",
                "una granja solitaria en las praderas infinitas",
                "las minas de plata donde los sueños morían",
                "un campamento de colonos que avanzaba hacia lo desconocido",
                "los territorios indios, donde dos mundos colisionaban",
            ],
            Mundo::Noir => vec![
                "los callejones de una ciudad que nunca dormía",
                "un barrio donde la policía era otro tipo de crimen",
                "las oficinas de un edificio que había visto días mejores",
                "un club nocturno donde se cerraban tratos oscuros",
                "los muelles donde los secretos se hundían con los cuerpos",
            ],
            Mundo::Steampunk => vec![
                "una ciudad de engranajes y humo perpetuo",
                "los talleres de un inventor marginado",
                "un dirigible que era ciudad flotante y prisión",
                "las calles de bronce donde la aristocracia mecánica reinaba",
                "los subterráneos donde vivían los olvidados de la revolución industrial",
            ],
            _ => vec![
                "un lugar que ya no existe como era",
                "circunstancias que nadie podría haber predicho",
                "un hogar que guardaba secretos",
                "una ciudad que nunca dormía",
                "las afueras de un mundo en transición",
            ],
        }
    }

    fn madurez(rng: &mut impl Rng, nombre: &str, _edad: u32, lang: &Language) -> String {
        crate::core::narrativa::BancoNarrativo::obtener("transformacion_radiante", lang, &[nombre], rng) // Fallback seguro
    }

    fn legado(rng: &mut impl Rng, nombre: &str, _edad: u32, lang: &Language) -> String {
        crate::core::narrativa::BancoNarrativo::obtener("transformacion_radiante", lang, &[nombre], rng) // Fallback seguro
    }
    
    fn crepusculo(rng: &mut impl Rng, nombre: &str, _edad: u32, lang: &Language) -> String {
        crate::core::narrativa::BancoNarrativo::obtener("transformacion_radiante", lang, &[nombre], rng) // Fallback seguro
    }
}

// ============================================================
// HISTORIAS ENTRELAZADAS PARA CONSTELACIONES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentoCompartido {
    pub descripcion: String,
    pub impacto_a: String,
    pub impacto_b: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoriaEntrelazada {
    pub narrativa_encuentro: String,
    pub momentos_compartidos: Vec<MomentoCompartido>,
    pub dinamica_actual: String,
}

pub fn generar_historia_entrelazada(
    rng: &mut impl Rng,
    nombre_a: &str,
    nombre_b: &str,
    mundo: &Mundo,
) -> HistoriaEntrelazada {
    // Cómo se conocieron
    let encuentros = match mundo {
        Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
            format!("{} y {} se encontraron en el peor momento posible. O quizás el único momento posible. El destino tiene ese sentido del humor.", nombre_a, nombre_b),
            format!("{} y {} se cruzaron en una posada donde ambos buscaban escapar de algo. Ni uno ni otro esperaba encontrar a alguien.", nombre_a, nombre_b),
            format!("La primera vez que {} vio a {}, supo que su vida se complicaría. No sabía cuánto.", nombre_a, nombre_b),
            format!("Se conocieron cuando {} salvó la vida de {}. O quizás fue al revés. Depende de a quién le preguntes.", nombre_a, nombre_b),
        ],
        Mundo::SciFiCyberpunk => vec![
            format!("{} y {} se conocieron en la red antes que en carne. La realidad fue... diferente.", nombre_a, nombre_b),
            format!("Un trabajo salió mal. {} necesitaba salir de la zona caliente. {} tenía la salida. Así empezó todo.", nombre_a, nombre_b),
            format!("{} y {} trabajaban para la misma corporación sin saberlo. Cuando lo descubrieron, las cosas se complicaron.", nombre_a, nombre_b),
        ],
        _ => vec![
            format!("{} y {} se encontraron cuando ambos más lo necesitaban. Dos extraños que se reconocieron en algo invisible.", nombre_a, nombre_b),
            format!("{} apareció en la vida de {} como una pregunta sin respuesta. Y se quedó.", nombre_a, nombre_b),
        ],
    };
    
    // Momentos compartidos
    let momentos = vec![
        MomentoCompartido {
            descripcion: "Hubo una noche donde las máscaras cayeron. Palabras que nunca se habían dicho encontraron voz.".into(),
            impacto_a: format!("{} reveló algo que nunca había contado. Se sintió expuesto y, extrañamente, aliviado.", nombre_a),
            impacto_b: format!("{} escuchó sin juzgar. Fue la primera vez que alguien lo hacía por {}.", nombre_b, nombre_a),
        },
        MomentoCompartido {
            descripcion: "Llegó el momento de elegir. Lealtad al otro o salvarse a sí mismo.".into(),
            impacto_a: format!("{} eligió quedarse. Fue la decisión más difícil y la más fácil.", nombre_a),
            impacto_b: format!("{} nunca olvidará que {} se quedó cuando otros habrían huido.", nombre_b, nombre_a),
        },
        MomentoCompartido {
            descripcion: "Hubo un conflicto. Palabras que no se pueden retirar. Un silencio que duró demasiado.".into(),
            impacto_a: format!("{} dijo cosas que no sentía. O quizás las sentía demasiado.", nombre_a),
            impacto_b: format!("Para {}, fue la confirmación de un miedo viejo. Y también, eventualmente, la oportunidad de superarlo.", nombre_b),
        },
    ];
    
    let momentos_seleccionados: Vec<MomentoCompartido> = momentos.into_iter()
        .collect::<Vec<_>>()
        .choose_multiple(rng, 2)
        .cloned()
        .collect();
    
    // Dinámica actual
    let dinamicas = vec![
        format!("Ahora, {} y {} son lo más cercano a familia que ninguno tiene. No se eligieron. Pero se eligen cada día.", nombre_a, nombre_b),
        format!("Entre {} y {} hay un entendimiento que no necesita palabras. También hay cosas no dichas que pesan. Ambas cosas son verdad.", nombre_a, nombre_b),
        format!("La relación entre {} y {} es complicada. ¿Quién necesita a quién más? Depende del día.", nombre_a, nombre_b),
        format!("{} y {} han aprendido a pelear juntos y a pelear entre sí. Lo segundo es más difícil. Y más necesario.", nombre_a, nombre_b),
    ];
    
    HistoriaEntrelazada {
        narrativa_encuentro: encuentros.choose(rng).unwrap().clone(),
        momentos_compartidos: momentos_seleccionados,
        dinamica_actual: dinamicas.choose(rng).unwrap().clone(),
    }
}

/// Genera la narrativa conjunta de una constelación de forma modular y dinámica
pub fn generar_historia_conjunta(
    rng: &mut impl Rng,
    nombres: &[String],
    mundo: &Mundo,
) -> String {
    if nombres.is_empty() {
        return String::new();
    }
    
    if nombres.len() == 1 {
        return format!("La historia de {} es solitaria por ahora, un hilo único esperando tejerse en algo mayor.", nombres[0]);
    }
    
    // 1. Contexto Temporal (El "Cuándo")
    let contextos = match mundo {
        Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
            "En el invierno más largo que los ancianos recuerdan",
            "Justo después de la Caída del Último Bastión",
            "En una era donde los dioses han dejado de responder plegarias",
            "Cuando los caminos seguros dejaron de serlo",
            "Bajo la sombra creciente de una guerra que nadie pidió",
            "En los días rotos que siguieron al Gran Silencio",
        ],
        Mundo::SciFiCyberpunk => vec![
            "Tras el colapso de la Red Global",
            "En los niveles inferiores del Sector 0",
            "Cuando el protocolo de seguridad falló en cadena",
            "Durante la semana del gran apagón",
            "En una ciudad que nunca ve el sol real",
        ],
        _ => vec![
            "En un momento de cambio irreversible",
            "Cuando las viejas reglas dejaron de aplicar",
            "En medio de una calma engañosa",
            "Cuando el destino decidió jugar sus cartas",
        ],
    };
    
    // 2. El Incidente (El "Qué los unió")
    let incidentes = match mundo {
        Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
            "un contrato que nadie más quiso aceptar los ató al mismo destino",
            "una tormenta antinatural los encerró en el mismo refugio olvidado",
            "se encontraron siendo los únicos sobrevivientes de una emboscada",
            "compartieron celda antes de compartir camino",
            "una visión compartida los llevó al mismo claro en el bosque",
            "el robo de un artefacto sagrado los puso en la misma lista de buscados",
        ],
        Mundo::SciFiCyberpunk => vec![
            "una carga de datos corruptos los marcó como objetivos prioritarios",
            "se encontraron en el lado equivocado de una redada corporativa",
            "ambos buscaban al mismo fantasma digital",
            "un glitch en el sistema los asignó a la misma misión suicida",
        ],
        _ => vec![
            "una coincidencia imposible los puso en el mismo lugar a la misma hora",
            "descubrieron que compartían el mismo enemigo",
            "una pérdida común los dejó sin otra opción",
        ],
    };
    
    // 3. La Dinámica del Grupo (El "Cómo funcionan")
    let dinamicas = if nombres.len() == 2 {
        vec![
            format!("Ahora, {} y {} son dos mitades de un todo disfuncional pero efectivo.", nombres[0], nombres[1]),
            format!("{} pone el caos y {} pone el orden (o quizás al revés, depende del día).", nombres[0], nombres[1]),
            format!("Se mueven con la sincronía de quienes han peleado espalda con espalda demasiadas veces.", ),
            format!("Son un equipo improbable, mantenido unido por lealtad y desesperación a partes iguales."),
        ]
    } else {
        vec![
            "Ahora son una familia extraña, forjada no por sangre sino por supervivencia.".to_string(),
            "Funcionan como un mecanismo de relojería oxidado: ruidoso, peligroso, pero imparable.".to_string(),
            "Son un grupo de inadaptados que descubrieron que encajaban entre ellos.".to_string(),
            "Se han convertido en la única certeza del otro en un mundo incierto.".to_string(),
        ]
    };
    
    // 4. El "Pero" (Tensión o Semilla Narrativa)
    let tensiones = vec![
        "Pero la confianza es una moneda frágil que se gasta rápido.",
        "Aunque cada uno guarda un secreto que podría quemarlo todo.",
        "Sin embargo, el pasado no ha terminado con ellos.",
        "Pero saben que esta alianza tiene fecha de caducidad.",
        "Y cada noche, uno de ellos duerme con un ojo abierto.",
        "Aunque todos evitan hablar de lo que dejaron atrás.",
    ];

    format!(
        "{}, {}. {} {}",
        contextos.choose(rng).unwrap(),
        incidentes.choose(rng).unwrap(),
        dinamicas.choose(rng).unwrap(),
        tensiones.choose(rng).unwrap()
    )
}
