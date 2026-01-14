//! Sistema de Constelaciones - Grupos de personajes interconectados
//!
//! Una constelación es un grupo de almas donde las relaciones EMERGEN
//! de sus psicologías, creando tensiones narrativas orgánicas.

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::{
    Alma, ParametrosGeneracion, ParametrosConstelacion,
    Mundo, Rol, TonoMoral, DensidadRelaciones, EstructuraGrupo, ForgeConfig,
    HistoriaEntrelazada, generar_historia_entrelazada,
    ConexionPersonajes, generar_conexion_profunda,
    EventoAncla
};
use crate::relaciones::Vinculo;



/// Una constelación de personajes interconectados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constelacion {
    pub id: Uuid,
    pub semilla: u64, // Semilla para regeneración exacta
    pub nombre: String,
    pub evento_ancla: EventoAncla, // Nuevo campo
    pub mundo: Mundo,
    pub almas: Vec<Alma>,
    pub vinculos: Vec<Vinculo>,
    pub tensiones_centrales: Vec<TensionCentral>,
    pub triangulos: Vec<Triangulo>,
    pub resumen_narrativo: String,
    pub historia_conjunta: String,
    pub historias_pares: Vec<HistoriaPareja>,
    pub conexiones_profundas: Vec<ConexionPersonajes>,
}


/// Historia entre un par específico de personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoriaPareja {
    pub nombre_a: String,
    pub nombre_b: String,
    pub historia: HistoriaEntrelazada,
}


/// Una tensión narrativa central que afecta a múltiples personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionCentral {
    pub descripcion: String,
    pub personajes_involucrados: Vec<Uuid>,
    pub tipo: TipoTension,
    pub como_podria_estallar: String,
    pub stakes: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TipoTension {
    Secreto,           // Algo oculto que puede destruir todo
    CompetenciaPoder,  // Luchan por lo mismo
    LealtadDividida,   // No pueden servir a todos
    AmorProhibido,     // Sentimientos que no deberían existir
    VenganzaPendiente, // Cuentas sin saldar
    IdealEnConflicto,  // Visiones del mundo opuestas
}

/// Un triángulo de tensión entre tres personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triangulo {
    pub personaje_a: Uuid,
    pub personaje_b: Uuid,
    pub personaje_c: Uuid,
    pub dinamica: String,
    pub punto_de_quiebre: String,
}

impl Constelacion {
    pub fn new(
        semilla: u64,
        mundo: Mundo,
        almas: Vec<Alma>,
        vinculos: Vec<Vinculo>,
    ) -> Self {
        // Usar RNG determinístico basado en la semilla de la constelación
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(semilla);
    
        let tensiones = Self::detectar_tensiones(&almas, &vinculos);
        let triangulos = Self::detectar_triangulos(&almas, &vinculos);
        let resumen = Self::generar_resumen(&almas, &vinculos, &tensiones);
        
        // Generar historia conjunta usando el motor de biografías
        let nombres: Vec<String> = almas.iter().map(|a| a.identidad.nombre.clone()).collect();
        // Nota: rng es pasado mutable
        let historia = crate::core::generar_historia_conjunta(&mut rng, &nombres, &mundo);
        
        // Generar historias entrelazadas entre CADA par de personajes
        let mut historias_pares = Vec::new();
        let mut conexiones_profundas = Vec::new();
        
        for i in 0..almas.len() {
            for j in (i+1)..almas.len() {
                let nombre_a = almas[i].identidad.nombre.clone();
                let nombre_b = almas[j].identidad.nombre.clone();
                
                // Historia entrelazada básica
                let historia_par = generar_historia_entrelazada(&mut rng, &nombre_a, &nombre_b, &mundo);
                historias_pares.push(HistoriaPareja {
                    nombre_a: nombre_a.clone(),
                    nombre_b: nombre_b.clone(),
                    historia: historia_par,
                });
                
                // Conexión profunda con más detalle
                let conexion = generar_conexion_profunda(&mut rng, &nombre_a, &nombre_b, &mundo);
                conexiones_profundas.push(conexion);
            }
        }
        
        
        // Generar Evento Ancla (Historia Única)
        let evento_ancla = Self::generar_evento_ancla(&mut rng, &mundo);

        Self {
            id: Uuid::new_v4(),
            semilla,
            nombre: Self::generar_nombre(&mut rng, &mundo),
            evento_ancla,
            mundo,
            almas,
            vinculos,
            tensiones_centrales: tensiones,
            triangulos,
            resumen_narrativo: resumen,
            historia_conjunta: historia,
            historias_pares,
            conexiones_profundas,
        }
    }
    
    fn generar_evento_ancla(rng: &mut impl Rng, mundo: &Mundo) -> EventoAncla {
        let (nombres, fechas, descripciones, impactos) = match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => (
                vec!["La Masacre del Valle Gris", "La Noche de los Tres Soles", "El Asedio de Bastión Roto", "La Traición del Príncipe Ciego", "El Cisma de la Rosa", "La Caída de la Casa Vane"],
                vec!["Año 402", "Era de las Cenizas", "Ciclo 7", "El Invierno Rojo", "Hace una década"],
                vec![
                    "Una batalla que dejó el río teñido de rojo durante semanas.", 
                    "Un pacto roto que condenó a una generación entera al exilio.", 
                    "El día que la magia dejó de responder a los llamados de los clérigos.",
                    "Una rebelión sofocada con fuego valyrio y hierro frío."
                ],
                vec![
                    "El reino quedó dividido en dos facciones irreconciliables.",
                    "La fe en los protectores se rompió para siempre.",
                    "Miles fueron desplazados, creando una diáspora de huérfanos y soldados perdidos.",
                    "Se prohibió el uso de las artes antiguas bajo pena de muerte."
                ]
            ),
            Mundo::SciFiCyberpunk | Mundo::SciFiSpace => (
                vec!["El Apagón de Neo-Tokyo", "La Purga del Sector 7", "El Incidente Zero-Day", "La Guerra Corporativa de 2099", "El Colapso de la Red Helios"],
                vec!["Ciclo 2.4", "Stardate 9021.4", "Año 2142", "La Semana Negra"],
                vec![
                    "Un virus neuro-digital frió los implantes de millones en segundos.",
                    "Las corporaciones decidieron 'reestructurar' un sector entero con napalm.",
                    "La IA central de la ciudad desarrolló consciencia y apagó los sistemas de soporte vital.",
                    "Una fuga de datos reveló los experimentos genéticos secretos de la élite."
                ],
                vec![
                    "La desconfianza en la tecnología alcanzó niveles críticos.",
                    "Se establecieron zonas de exclusión donde la ley no tiene poder.",
                    "Surgió una resistencia clandestina que opera desde las sombras analógicas.",
                    "El valor de la vida humana cayó por debajo del precio del agua sintética."
                ]
            ),
            _ => (
                vec!["El Gran Incendio", "La Revuelta de Octubre", "El Colapso", "La Desaparición"],
                vec!["Hace 5 años", "1999", "El día cero"],
                vec!["Un evento que nadie vio venir y que cambió las reglas del juego.", "La ciudad ardió mientras los líderes miraban hacia otro lado."],
                vec!["Nada volvió a ser igual en las calles.", "El silencio que siguió fue más fuerte que los gritos."]
            )
        };
        
        // Elementos aleatorios modulares para más variedad
        let extra_adj = vec!["Sangriento", "Olvidado", "Silencioso", "Eterno", "Traicionado", "Roto", "Final", "Oscuro"];
        let extra_noun = vec!["Pacto", "Juramento", "Sacrificio", "Error", "Invierno", "Verano", "Silencio", "Grito"];
        
        let nombre_base = if rng.gen_bool(0.25) {
             format!("El {} {}", extra_noun.choose(rng).unwrap(), extra_adj.choose(rng).unwrap())
        } else {
             nombres.choose(rng).unwrap().to_string()
        };
        
        let fecha = fechas.choose(rng).unwrap().to_string();
        let desc = descripciones.choose(rng).unwrap().to_string();
        let impacto = impactos.choose(rng).unwrap().to_string();
        
        EventoAncla {
            nombre: nombre_base,
            anio: fecha,
            descripcion: desc,
            impacto_global: impacto,
        }
    }


    
    fn generar_nombre(rng: &mut impl Rng, mundo: &Mundo) -> String {
        
        let formas = vec!["El Círculo", "La Orden", "Los Hijos", "La Hermandad", "El Pacto", "Los Errantes", "La Vanguardia", "El Fragmento", "La Sombra", "El Eco"];
        
        let adjetivos = match mundo {
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "del Crepúsculo", "de Hierro", "Silencioso", "Olvidado", "Eterno", "Roto", 
                "de Ceniza", "de Sangre", "Nocturno", "Sagrado", "Maldito", "de Cristal",
                "del Invierno", "sin Rey", "de la Espada"
            ],
            Mundo::SciFiCyberpunk => vec![
                "de Neón", "Digital", "Cromado", "Sintético", "Desconectado", "Fantasma", 
                "Cero", "Binario", "Corrupto", "de Silicio", "Bionico", "de la Red"
            ],
            Mundo::Realista => vec![
                "Invisible", "Urbano", "del Mañana", "Perdido", "Oculto", "de Nadie", 
                "Subterráneo", "de la Calle", "Gris", "Bajo Cero"
            ],
            _ => vec!["Errante", "Lejano", "Antiguo", "Nuevo", "Final"],
        };
        
        // 30% de probabilidad de tener un sufijo extra
        let sufijo_extra = if rng.gen_bool(0.3) {
            match mundo {
                Mundo::FantasiaMedieval => Some(*["del Norte", "de la Llama", "del Bosque Viejo", "de los Reyes Muertos"].choose(rng).unwrap()),
                Mundo::SciFiCyberpunk => Some(*["v2.0", "Protocolo Sigma", "del Sector 7", "Null_Pointer"].choose(rng).unwrap()),
                _ => None
            }
        } else {
            None
        };

        let forma = formas.choose(rng).unwrap();
        let adj = adjetivos.choose(rng).unwrap();
        
        match sufijo_extra {
            Some(suf) => {
                if *forma == "Los Hijos" || *forma == "Los Errantes" {
                    format!("{} {} {}", forma, adj, suf) // Ej: Los Hijos Rotos del Norte
                } else {
                    format!("{} {} {}", forma, adj, suf)
                }
            },
            None => format!("{} {}", forma, adj) // Ej: El Círculo Roto
        }
    }
    
    fn detectar_tensiones(almas: &[Alma], vinculos: &[Vinculo]) -> Vec<TensionCentral> {
        let mut tensiones = Vec::new();
        
        // Buscar conflictos de sombras altos
        for v in vinculos {
            if v.quimica.conflicto_sombras > 0.6 {
                let a = almas.iter().find(|a| a.id == v.id_persona_a).unwrap();
                let b = almas.iter().find(|a| a.id == v.id_persona_b).unwrap();
                
                tensiones.push(TensionCentral {
                    descripcion: format!(
                        "{} y {} representan lados opuestos de un mismo conflicto interno.",
                        a.identidad.nombre, b.identidad.nombre
                    ),
                    personajes_involucrados: vec![a.id, b.id],
                    tipo: TipoTension::IdealEnConflicto,
                    como_podria_estallar: "Cuando uno tenga que elegir entre su ideal y el otro".to_string(),
                    stakes: "Su relación y posiblemente sus vidas".to_string(),
                });
            }
        }
        
        // Buscar competencias por el mismo deseo
        for i in 0..almas.len() {
            for j in (i+1)..almas.len() {
                if almas[i].capas.deseo_necesidad.deseo_consciente == 
                   almas[j].capas.deseo_necesidad.deseo_consciente {
                    tensiones.push(TensionCentral {
                        descripcion: format!(
                            "Tanto {} como {} buscan {} - solo uno puede tenerlo.",
                            almas[i].identidad.nombre,
                            almas[j].identidad.nombre,
                            almas[i].capas.deseo_necesidad.deseo_consciente.to_lowercase()
                        ),
                        personajes_involucrados: vec![almas[i].id, almas[j].id],
                        tipo: TipoTension::CompetenciaPoder,
                        como_podria_estallar: "Cuando ambos estén cerca de conseguirlo".to_string(),
                        stakes: "Sus sueños y quizás su amistad".to_string(),
                    });
                }
            }
        }
        
        tensiones.truncate(3); // Máximo 3 tensiones centrales
        tensiones
    }
    
    fn detectar_triangulos(almas: &[Alma], vinculos: &[Vinculo]) -> Vec<Triangulo> {
        let mut triangulos = Vec::new();
        
        if almas.len() < 3 {
            return triangulos;
        }
        
        // Buscar triángulos donde todos están conectados
        for i in 0..almas.len() {
            for j in (i+1)..almas.len() {
                for k in (j+1)..almas.len() {
                    let a = &almas[i];
                    let b = &almas[j];
                    let c = &almas[k];
                    
                    // Verificar si hay vínculos entre los tres
                    let ab = vinculos.iter().find(|v| 
                        (v.id_persona_a == a.id && v.id_persona_b == b.id) ||
                        (v.id_persona_a == b.id && v.id_persona_b == a.id)
                    );
                    let bc = vinculos.iter().find(|v|
                        (v.id_persona_a == b.id && v.id_persona_b == c.id) ||
                        (v.id_persona_a == c.id && v.id_persona_b == b.id)
                    );
                    let ac = vinculos.iter().find(|v|
                        (v.id_persona_a == a.id && v.id_persona_b == c.id) ||
                        (v.id_persona_a == c.id && v.id_persona_b == a.id)
                    );
                    
                    if ab.is_some() && bc.is_some() && ac.is_some() {
                        triangulos.push(Triangulo {
                            personaje_a: a.id,
                            personaje_b: b.id,
                            personaje_c: c.id,
                            dinamica: format!(
                                "{}, {} y {} forman un triángulo de tensiones.",
                                a.identidad.nombre, b.identidad.nombre, c.identidad.nombre
                            ),
                            punto_de_quiebre: "Cuando uno tenga que elegir entre los otros dos".to_string(),
                        });
                    }
                }
            }
        }
        
        triangulos.truncate(2); // Máximo 2 triángulos
        triangulos
    }
    
    fn generar_resumen(
        almas: &[Alma], 
        vinculos: &[Vinculo],
        tensiones: &[TensionCentral]
    ) -> String {
        let mut resumen = String::new();
        
        resumen.push_str(&format!(
            "Un grupo de {} almas unidas por el destino.\n\n",
            almas.len()
        ));
        
        // Personajes principales
        resumen.push_str("PERSONAJES:\n");
        for alma in almas {
            resumen.push_str(&format!(
                "• {} ({:?}) - {}\n",
                alma.identidad.nombre,
                alma.rol,
                alma.capas.mascara.imagen_proyectada
            ));
        }
        
        // Tensiones
        if !tensiones.is_empty() {
            resumen.push_str("\nTENSIONES CENTRALES:\n");
            for t in tensiones {
                resumen.push_str(&format!("• {}\n", t.descripcion));
            }
        }
        
        // Relaciones clave
        resumen.push_str("\nRELACIONES CLAVE:\n");
        for v in vinculos.iter().filter(|v| v.intensidad > 0.5) {
            let a = almas.iter().find(|a| a.id == v.id_persona_a).unwrap();
            let b = almas.iter().find(|a| a.id == v.id_persona_b).unwrap();
            resumen.push_str(&format!(
                "• {} ↔ {}: {:?}\n",
                a.identidad.nombre, b.identidad.nombre, v.tipo
            ));
        }
        
        resumen
    }
    
    /// Genera una narrativa completa de la constelación
    pub fn narrar(&self) -> String {
        let mut n = String::new();
        
        n.push_str(&format!("╔═══════════════════════════════════════════════════════════════╗\n"));
        n.push_str(&format!("║             CONSTELACIÓN: {}             ║\n", self.nombre.to_uppercase()));
        n.push_str(&format!("╚═══════════════════════════════════════════════════════════════╝\n\n"));
        
        n.push_str(&self.resumen_narrativo);
        
        n.push_str("\n\n═══════════════════════════════════════════════════════════════\n");
        n.push_str("                    PERFILES INDIVIDUALES\n");
        n.push_str("═══════════════════════════════════════════════════════════════\n\n");
        
        for alma in &self.almas {
            n.push_str(&alma.narrar());
            n.push_str("\n\n");
        }
        
        n
    }
}

/// Builder para crear constelaciones
pub struct ConstelacionBuilder {
    params: Option<ParametrosConstelacion>,
}

impl ConstelacionBuilder {
    pub fn new() -> Self {
        Self { params: None }
    }
    
    pub fn con_params(mut self, params: ParametrosConstelacion) -> Self {
        self.params = Some(params);
        self
    }
    
    pub fn construir(self, rng: &mut impl Rng) -> Constelacion {
        let params = self.params.unwrap_or_default();
        let config = ForgeConfig::default();
        
        // Generar almas según la estructura
        let mut almas = Vec::new();
        
        // Determinar roles según estructura
        let roles = Self::determinar_roles(&params);
        
        // Track used names to avoid duplicates
        let mut nombres_usados: std::collections::HashSet<String> = std::collections::HashSet::new();

        for i in 0..params.cantidad {
            let rol = roles.get(i).copied();
            let tono = Self::determinar_tono(rng, i, &params);
            let nombre_fijo = params.nombres_fijos.as_ref().and_then(|v| v.get(i)).cloned();
            let genero_fijo = params.generos_fijos.as_ref().and_then(|v| v.get(i)).copied();
            let edad_fija = params.edades_fijas.as_ref().and_then(|v| v.get(i)).copied();
            
            let mut alma;
            let mut intentos = 0;
            
            loop {
                alma = Alma::generar(rng, ParametrosGeneracion {
                    rol,
                    tono_moral: Some(tono),
                    mundo: Some(params.mundo),
                    nombre_fijo: nombre_fijo.clone(),
                    genero: genero_fijo,
                    edad_fija,
                    ..Default::default()
                }, &config);

                if !nombres_usados.contains(&alma.identidad.nombre) {
                    nombres_usados.insert(alma.identidad.nombre.clone());
                    break;
                }
                
                // If it's a fixed name, we can't change it, so break anyway
                if nombre_fijo.is_some() {
                    break;
                }
                
                intentos += 1;
                // Avoid infinite loop if pool is exhausted (unlikely but safe)
                if intentos > 10 {
                    // Fallback: append a suffix if duplicate persists
                    alma.identidad.nombre = format!("{} II", alma.identidad.nombre);
                    nombres_usados.insert(alma.identidad.nombre.clone());
                    break;
                }
            }
            
            almas.push(alma);
        }

        // Post-procesamiento: Aplicar motivaciones específicas al villano si existen
        if let Some(tipo_villano) = &params.tipo_villano {
             use crate::core::gramatica;
             for alma in &mut almas {
                 if alma.rol == Rol::Sombra || alma.rol == Rol::Villano {
                     let motivacion = gramatica::generar_motivacion_antagonista(tipo_villano, &params.mundo);
                     alma.capas.mentira.la_mentira = format!("{} (Raíz: {})", motivacion, tipo_villano);
                 }
             }
        }
        
        // Calcular vínculos según densidad
    let vinculos = Self::generar_vinculos(&almas, &params);
    
    // Semilla para la constelación (para poder regenerar sus partes narrativas)
    let semilla_constelacion = rng.next_u64();
    
    let mut constelacion = Constelacion::new(semilla_constelacion, params.mundo, almas, vinculos);

    // Post-procesamiento: Romance Forzado
    if let Some(modo_romance) = &params.modo_romance {
        use crate::core::TipoConexion;
        
        let n = constelacion.almas.len();
        if n >= 2 {
            let nombre_0 = constelacion.almas[0].identidad.nombre.clone();
            let nombre_1 = constelacion.almas[1].identidad.nombre.clone();
            
            if modo_romance == "Pareja" || modo_romance == "AmorProhibido" {
                // Buscar conexión entre 0 y 1
                if let Some(con) = constelacion.conexiones_profundas.iter_mut().find(|c| 
                    (c.nombre_a == nombre_0 && c.nombre_b == nombre_1) ||
                    (c.nombre_b == nombre_0 && c.nombre_a == nombre_1)
                ) {
                    con.tipo = TipoConexion::Amor;
                    let desc = if modo_romance == "AmorProhibido" {
                        "Un encuentro clandestino que desafió todas las leyes de su mundo."
                    } else {
                        "Un juramento silencioso de lealtad absoluta más allá de la muerte."
                    };
                    
                    con.momento_origen.descripcion = desc.to_string();
                    con.estado_actual = if modo_romance == "AmorProhibido" { "Secreto y peligroso".into() } else { "Inquebrantable".into() };
                    con.tension_actual = "Alta intensidad emocional".into();
                }
            } else if modo_romance == "Triangulo" && n >= 3 {
                let nombre_2 = constelacion.almas[2].identidad.nombre.clone();
                // Hacer que 0 y 2 amen a 1, o un ciclo.
                // Haremos 0-1 Amor, 1-2 Amor (conflicto)
                
                // 0-1
                if let Some(con) = constelacion.conexiones_profundas.iter_mut().find(|c| 
                    (c.nombre_a == nombre_0 && c.nombre_b == nombre_1) ||
                    (c.nombre_b == nombre_0 && c.nombre_a == nombre_1)
                ) {
                    con.tipo = TipoConexion::Amor;
                    con.estado_actual = "Complicado".into();
                }
                
                // 1-2
                if let Some(con) = constelacion.conexiones_profundas.iter_mut().find(|c| 
                    (c.nombre_a == nombre_1 && c.nombre_b == nombre_2) ||
                    (c.nombre_b == nombre_1 && c.nombre_a == nombre_2)
                ) {
                    con.tipo = TipoConexion::Amor; // O Rivalidad amorosa si 0 y 2 son rivales?
                    con.tension_actual = "Celos latentes".into();
                }
            }
        }
    }

    constelacion
}    
    fn determinar_roles(params: &ParametrosConstelacion) -> Vec<Rol> {
        match params.estructura {
            EstructuraGrupo::Grupo => {
                let mut roles = vec![Rol::Heroe, Rol::Aliado, Rol::Aliado];
                if params.incluir_antagonista {
                    roles.push(Rol::Sombra);
                }
                if params.cantidad > roles.len() {
                    roles.push(Rol::Mentor);
                }
                roles
            },
            EstructuraGrupo::Triangulo => {
                vec![Rol::Heroe, Rol::Aliado, Rol::Catalizador]
            },
            EstructuraGrupo::Paralelo => {
                vec![Rol::Heroe, Rol::Aliado, Rol::Villano, Rol::Aliado]
            },
            _ => {
                Rol::all().into_iter().take(params.cantidad).collect()
            }
        }
    }
    
    fn determinar_tono(rng: &mut impl Rng, index: usize, params: &ParametrosConstelacion) -> TonoMoral {
        if params.incluir_antagonista && index == params.cantidad - 1 {
            TonoMoral::Oscuro
        } else {
            *[TonoMoral::Claro, TonoMoral::Gris, TonoMoral::Claro].choose(rng).unwrap()
        }
    }
    
    fn generar_vinculos(almas: &[Alma], params: &ParametrosConstelacion) -> Vec<Vinculo> {
        let mut vinculos = Vec::new();
        
        let probabilidad = match params.densidad_relaciones {
            DensidadRelaciones::Dispersa => 0.3,
            DensidadRelaciones::Normal => 0.5,
            DensidadRelaciones::Densa => 0.7,
            DensidadRelaciones::Claustrofobica => 1.0,
        };
        
        for i in 0..almas.len() {
            for j in (i+1)..almas.len() {
                if rand::random::<f32>() < probabilidad {
                    vinculos.push(Vinculo::crear(&almas[i], &almas[j]));
                }
            }
        }
        
        vinculos
    }
}
