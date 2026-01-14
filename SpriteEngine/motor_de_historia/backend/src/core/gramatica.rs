//! Sistema de Gramáticas Narrativas para generación procedural variada.
//! 
//! Este módulo implementa gramáticas libres de contexto (CFG) para generar
//! texto narrativo con alta variabilidad sin necesidad de LLM.

use rand::prelude::*;

/// Generador de frases basado en plantillas con slots.
/// 
/// Ejemplo de uso:
/// ```
/// let frase = Gramatica::construir(rng, "[SUJETO] [VERBO] cuando [SITUACION]")
///     .slot("SUJETO", &["Él", "Ella", "El protagonista"])
///     .slot("VERBO", &["tiembla", "se paraliza", "huye"])
///     .slot("SITUACION", &["siente abandono", "percibe rechazo"])
///     .generar();
/// ```
pub struct Gramatica<'a> {
    _rng: &'a mut dyn RngCore,
    _plantilla: String,
}

impl<'a> Gramatica<'a> {
    pub fn construir<R: RngCore>(rng: &'a mut R, plantilla: &str) -> GramaticaBuilder<'a> {
        GramaticaBuilder {
            rng: rng as &mut dyn RngCore,
            plantilla: plantilla.to_string(),
        }
    }
}

pub struct GramaticaBuilder<'a> {
    rng: &'a mut dyn RngCore,
    plantilla: String,
}

impl<'a> GramaticaBuilder<'a> {
    /// Reemplaza un slot [NOMBRE] con una opción aleatoria del pool.
    pub fn slot(mut self, nombre: &str, opciones: &[&str]) -> Self {
        if let Some(opcion) = opciones.choose(self.rng) {
            let marcador = format!("[{}]", nombre);
            self.plantilla = self.plantilla.replace(&marcador, opcion);
        }
        self
    }
    
    /// Variante que acepta Strings en lugar de &str.
    pub fn slot_owned(mut self, nombre: &str, opciones: &[String]) -> Self {
        if let Some(opcion) = opciones.choose(self.rng) {
            let marcador = format!("[{}]", nombre);
            self.plantilla = self.plantilla.replace(&marcador, opcion);
        }
        self
    }
    
    /// Genera la frase final.
    pub fn generar(self) -> String {
        self.plantilla
    }
}

// ============================================================================
// POOLS DE VOCABULARIO EXPANDIDOS
// ============================================================================

/// Sujetos genéricos para causantes de heridas
pub fn sujetos_causantes() -> Vec<&'static str> {
    vec![
        "Su padre", "Su madre", "Su hermano mayor", "Su hermana",
        "Su mentor", "Su primer amor", "Su mejor amigo", "Su comunidad",
        "El líder que admiraba", "Quienes juraron protegerlo",
        "Su familia adoptiva", "El único en quien confiaba",
        "Su maestro", "Su protector", "Su prometido/a",
        "Los ancianos del pueblo", "El consejo", "Su compañero de armas",
    ]
}

/// Verbos de impacto emocional
pub fn verbos_trauma() -> Vec<&'static str> {
    vec![
        "lo traicionó", "lo abandonó", "desapareció", "murió",
        "eligió a otro", "reveló sus secretos", "lo vendió",
        "lo humilló públicamente", "destruyó su reputación",
        "usó su confianza como arma", "fingió su muerte",
        "lo dejó a merced de enemigos", "negó conocerlo",
    ]
}

/// Circunstancias de heridas
pub fn circunstancias_herida() -> Vec<&'static str> {
    vec![
        "en el momento en que más lo necesitaba",
        "sin dar explicación alguna",
        "por razones que nunca comprendió",
        "para proteger intereses propios",
        "frente a todos los que respetaba",
        "la noche antes de un momento crucial",
        "después de años de aparente lealtad",
        "mientras él arriesgaba todo por ellos",
        "usando información que compartió en confianza",
        "exactamente como había prometido no hacer",
    ]
}

/// Cambios psicológicos tras el trauma
pub fn cambios_internos() -> Vec<&'static str> {
    vec![
        "Aprendió a no necesitar a nadie, aunque eso lo consume",
        "Construyó muros que nadie ha logrado cruzar",
        "Desarrolló un instinto para detectar traición donde no existe",
        "Se prometió nunca más ser tan vulnerable",
        "Convirtió el dolor en combustible para su ambición",
        "Dejó morir la parte de sí que podía confiar",
        "Se transformó en aquello que juró destruir",
        "Enterró su verdadero yo bajo capas de cinismo",
        "Canalizó todo hacia una obsesión que lo define",
        "Aprendió que el amor es solo otra forma de control",
    ]
}

/// Gatillos emocionales
pub fn gatillos_emocionales() -> Vec<&'static str> {
    vec![
        "Cuando percibe que alguien se aleja emocionalmente",
        "Al detectar el más mínimo signo de secretismo",
        "Cuando alguien depende demasiado de él",
        "En momentos de intimidad genuina",
        "Cuando escucha promesas que suenan familiares",
        "Al sentirse excluido de decisiones importantes",
        "Cuando algo le recuerda a quien lo hirió",
        "En situaciones que requieren vulnerabilidad",
        "Cuando debe confiar sin evidencia",
        "Al percibir lástima en la mirada de otros",
    ]
}

/// Mecanismos de defensa
pub fn mecanismos_defensa() -> Vec<&'static str> {
    vec![
        "Distanciamiento preventivo: aleja a otros antes de que lo hieran",
        "Hipervigilancia: analiza cada palabra buscando amenazas ocultas",
        "Humor como escudo: convierte el dolor en sarcasmo",
        "Control obsesivo: si controla todo, nada puede sorprenderlo",
        "Independencia extrema: rechaza ayuda incluso cuando la necesita",
        "Máscara de indiferencia: finge que nada le importa",
        "Agresión preventiva: ataca antes de ser atacado",
        "Evasión sistemática: evita cualquier situación que active el trauma",
        "Perfeccionismo: si es perfecto, no lo rechazarán",
        "Auto-sabotaje: destruye lo bueno antes de que lo destruya a él",
    ]
}

// ============================================================================
// GENERADORES ESPECÍFICOS CON GRAMÁTICAS
// ============================================================================

/// Genera una circunstancia de herida completa usando gramática.
pub fn generar_circunstancia_herida(rng: &mut impl Rng, tipo_herida: &str) -> String {
    let plantillas = match tipo_herida {
        "Abandono" => vec![
            "[SUJETO] [VERBO] [CIRCUNSTANCIA]",
            "Fue [VERBO] por [SUJETO] [CIRCUNSTANCIA]",
            "[SUJETO] eligió [ALGO] sobre su bienestar",
        ],
        "Traicion" => vec![
            "[SUJETO] [VERBO] [CIRCUNSTANCIA], destruyendo [COSA_DESTRUIDA]",
            "Descubrió que [SUJETO] había [VERBO] todo este tiempo",
            "[SUJETO] usó [ALGO_CONFIADO] para [ACCION_TRAICION]",
        ],
        _ => vec![
            "[SUJETO] [VERBO] [CIRCUNSTANCIA]",
            "Las acciones de [SUJETO] [CONSECUENCIA]",
        ],
    };
    
    let plantilla = plantillas.choose(rng).unwrap();
    
    let algo = ["su cargo", "su ambición", "otro", "el poder", "el oro", "una causa"];
    let cosa_destruida = ["su fe en la humanidad", "años de confianza", "todo lo que creía sagrado"];
    let algo_confiado = ["sus secretos más oscuros", "información privilegiada", "su vulnerabilidad"];
    let accion_traicion = ["destruirlo", "venderlo", "humillarlo ante todos"];
    let consecuencia = ["marcaron su alma para siempre", "definieron quien es hoy"];
    

    let mut resultado = plantilla.to_string();
    
    // Reemplazos manuales (sin builder para compatibilidad)
    if let Some(s) = sujetos_causantes().choose(rng) {
        resultado = resultado.replace("[SUJETO]", s);
    }
    if let Some(v) = verbos_trauma().choose(rng) {
        resultado = resultado.replace("[VERBO]", v);
    }
    if let Some(c) = circunstancias_herida().choose(rng) {
        resultado = resultado.replace("[CIRCUNSTANCIA]", c);
    }
    if let Some(a) = algo.choose(rng) {
        resultado = resultado.replace("[ALGO]", a);
    }
    if let Some(d) = cosa_destruida.choose(rng) {
        resultado = resultado.replace("[COSA_DESTRUIDA]", d);
    }
    if let Some(a) = algo_confiado.choose(rng) {
        resultado = resultado.replace("[ALGO_CONFIADO]", a);
    }
    if let Some(a) = accion_traicion.choose(rng) {
        resultado = resultado.replace("[ACCION_TRAICION]", a);
    }
    if let Some(c) = consecuencia.choose(rng) {
        resultado = resultado.replace("[CONSECUENCIA]", c);
    }
    
    resultado
}

/// Genera una "mentira" (creencia limitante) con profundidad psicológica.
pub fn generar_mentira(rng: &mut impl Rng, tipo_herida: &str) -> (String, String, String) {
    let mentiras_por_herida = match tipo_herida {
        "Abandono" => vec![
            (
                "Soy intrínsecamente desechable para los demás", 
                "Interpretó cada adiós circunstancial como un juicio sobre su valor personal",
                "Mi presencia importa aunque no pueda retener a nadie"
            ),
            (
                "La permanencia es una ilusión de los ingenuos",
                "Vio desmoronarse lo que parecía eterno y decidió no volver a invertir en cimientos",
                "Construir vale la pena incluso si nada es para siempre"
            ),
            (
                "Debo ser útil para justificar el espacio que ocupo",
                "Aprendió que el afecto se cortaba cuando dejaba de servir a un propósito",
                "Existo y eso es suficiente"
            ),
        ],
        "Traicion" => vec![
            (
                "La intimidad es un campo minado que debo patrullar",
                "Confundió la paz de la confianza con la negligencia de la guardia baja",
                "Vivir en guardia perpetua es otra forma de estar prisionero"
            ),
            (
                "Si alguien se acerca, es porque quiere algo de mí",
                "Proyecta la agenda oculta de su traidor en cada rostro nuevo",
                "La generosidad genuina existe y puedo recibirla"
            ),
            (
                "El conocimiento es poder; el secreto es seguridad",
                "Decidió que ser opaco es la única forma de ser invulnerable",
                "Ser conocido es el único camino para ser amado"
            ),
        ],
        "Humillacion" => vec![
            (
                "Cualquier defecto visible será usado como arma",
                "Internalizó la burla pública como una verdad objetiva sobre su insuficiencia",
                "Mis imperfecciones me conectan con la humanidad, no me excluyen"
            ),
            (
                "El respeto se impone, no se gana",
                "Cree que el miedo es la única moneda que compra dignidad duradera",
                "La verdadera autoridad nace de la autenticidad, no del control"
            ),
            (
                "Si brillo demasiado, atraeré el rayo",
                "Aprendió a encogerse para evitar ser el blanco de la envidia o el juicio",
                "Tengo derecho a ocupar espacio y mostrar mi luz"
            ),
        ],
        "Culpa" => vec![
            (
                "La felicidad es una ofensa a quienes dañé",
                "Convirtió su alegría en un acto de traición hacia sus víctimas pasadas",
                "Mi sufrimiento no repara el pasado; mi curación podría ayudar al futuro"
            ),
            (
                "Soy toxicidad vestida de persona",
                "Cree que corrompe inevitablemente todo lo que toca, por lo que se aísla",
                "Mis errores son eventos, no mi identidad"
            ),
            (
                "Debo salvar a todos para compensar al que fallé",
                "Lleva un complejo de mesías impulsado por una deuda impagable",
                "No soy dios; no puedo controlar el destino de otros"
            ),
        ],
        _ => vec![
            (
                "El mundo es un mecanismo depredador",
                "Racionalizó su dolor como evidencia de una ley natural cruel",
                "Hay bondad en el caos si elijo verla"
            ),
            (
                "Solo valgo lo que logro producir",
                "Vinculó su humanidad a su rendimiento bajo presión",
                "Soy humano, no una herramienta"
            ),
        ],
    };
    
    // Añadimos variaciones gramaticales para mayor profundidad
    let base = mentiras_por_herida.choose(rng).unwrap();
    
    // A veces devolvemos la base, a veces la modificamos ligeramente para variar el tono
    if rng.gen_bool(0.7) {
        (base.0.to_string(), base.1.to_string(), base.2.to_string())
    } else {
        // Variante más filosófica
        (
            format!("Creencia Raíz: {}", base.0),
            format!("Origen: {}", base.1),
            format!("Verdad Oculta: {}", base.2)
        )
    }
}

/// Genera un comportamiento de máscara.
pub fn generar_mascara(rng: &mut impl Rng) -> (String, String, String) {
    let comportamientos = vec![
        ("Proyecta seguridad inquebrantable", "Líder nato que nunca duda", "El temor constante de ser descubierto"),
        ("Actúa con desapego calculado", "Alguien por encima de las emociones mundanas", "Una necesidad desesperada de conexión"),
        ("Muestra encanto superficial", "Persona agradable y sin complicaciones", "Vacío existencial que el encanto no llena"),
        ("Exhibe competencia extrema", "El experto que siempre tiene la respuesta", "Terror paralizante ante lo desconocido"),
        ("Practica humor constante", "El que alivia tensiones y nunca sufre", "Dolor profundo que el humor apenas disfraza"),
        ("Mantiene control obsesivo", "Profesional impecable y predecible", "Caos interno que amenaza con desbordarse"),
        ("Demuestra generosidad excesiva", "Benefactor desinteresado y noble", "Necesidad de ser necesitado para tener valor"),
        ("Adopta frialdad estratégica", "Pragmático sin ataduras emocionales", "Miedo atroz a ser herido de nuevo"),
    ];
    
    let seleccion = comportamientos.choose(rng).unwrap();
    (seleccion.0.to_string(), seleccion.1.to_string(), seleccion.2.to_string())
}

// ============================================================================
// CONTENIDO POSITIVO - MOMENTOS DE LUZ
// ============================================================================

/// Fortalezas y dones naturales del personaje
pub fn fortalezas_personales() -> Vec<&'static str> {
    vec![
        "Una capacidad innata para ver lo mejor en otros, incluso cuando ellos no pueden",
        "Resiliencia forjada en adversidad: se levanta cada vez que cae",
        "Una risa contagiosa que ilumina las habitaciones más oscuras",
        "Lealtad inquebrantable hacia quienes ama, sin importar el costo",
        "Creatividad desbordante que transforma lo ordinario en extraordinario",
        "La habilidad de escuchar de verdad, no solo oír",
        "Coraje silencioso que emerge cuando más se necesita",
        "Generosidad que no espera nada a cambio",
        "Una curiosidad insaciable que lo hace crecer constantemente",
        "Integridad férrea: su palabra vale más que cualquier contrato",
        "Compasión que se extiende incluso hacia quienes lo hirieron",
        "La capacidad de encontrar belleza en lo que otros ignoran",
        "Un optimismo que sobrevive a las peores tormentas",
        "Humildad genuina que invita a la conexión",
        "Persistencia que convierte los obstáculos en escalones",
    ]
}

/// Descripciones de momentos de conexión genuina
pub fn momentos_conexion() -> Vec<&'static str> {
    vec![
        "Alguien lo miró a los ojos y vio más allá de la máscara, sin juzgar",
        "Una mano se extendió en la oscuridad y no pidió nada a cambio",
        "Descubrió que no estaba tan solo como creía cuando más lo necesitaba",
        "Alguien creyó en él antes de que él creyera en sí mismo",
        "Encontró un alma gemela donde menos lo esperaba",
        "Un extraño le devolvió la fe en la humanidad con un gesto simple",
        "Alguien eligió quedarse cuando todos los demás se fueron",
        "Fue aceptado exactamente como era, sin condiciones",
        "Descubrió que podía confiar otra vez, y no fue destruido por ello",
        "Una amistad surgió de las cenizas de su aislamiento",
    ]
}

/// Descripciones de momentos de logro personal
pub fn momentos_logro() -> Vec<&'static str> {
    vec![
        "Logró algo que nadie (incluido él mismo) creía posible",
        "Superó un miedo que lo había paralizado por años",
        "Ayudó a alguien de una manera que cambió ambas vidas",
        "Encontró su voz cuando más la necesitaba",
        "Defendió lo correcto aunque estuviera solo",
        "Transformó su dolor en algo que ayuda a otros",
        "Demostró que los escépticos estaban equivocados",
        "Creó algo hermoso desde el vacío",
        "Perdonó lo que parecía imperdonable",
        "Se perdonó a sí mismo después de años de autocastigo",
        "Eligió la esperanza cuando la desesperanza era más fácil",
        "Construyó algo duradero sobre ruinas",
    ]
}

/// Pequeñas alegrías y placeres que lo humanizan
pub fn pequenas_alegrias() -> Vec<&'static str> {
    vec![
        "El olor de la lluvia sobre tierra seca",
        "El silencio compartido con alguien que entiende",
        "Una melodía que lo transporta a tiempos mejores",
        "El calor del sol en un día frío",
        "La risa de alguien que ama",
        "Historias contadas junto al fuego",
        "El sabor de una comida que le recuerda su hogar",
        "El momento justo antes del amanecer",
        "Una victoria pequeña después de muchas derrotas",
        "El abrazo de alguien que lo acepta completamente",
        "Descubrir que alguien pensó en él sin razón",
        "La paz de un lugar que siente como propio",
    ]
}

/// Esperanzas y sueños que lo impulsan
pub fn esperanzas_suenos() -> Vec<&'static str> {
    vec![
        "Encontrar un lugar donde finalmente pertenezca",
        "Construir algo que perdure más allá de su vida",
        "Reconciliarse con quien lo hirió, o con su propia rabia",
        "Descubrir que el amor que busca ha estado cerca todo el tiempo",
        "Demostrar que su pasado no define su futuro",
        "Proteger a otros de lo que él sufrió",
        "Encontrar paz interior, no solo supervivencia",
        "Ser recordado por algo más que sus errores",
        "Reconectar con partes de sí mismo que creía perdidas",
        "Crear una familia (de sangre o elegida) que funcione",
        "Ver el mundo más allá de las fronteras que conoce",
        "Contribuir a algo más grande que él mismo",
    ]
}

/// Genera un momento de gracia completo
pub fn generar_momento_gracia(rng: &mut impl Rng, nombre: &str, categoria: &str) -> (String, String, String, String) {
    match categoria {
        "conexion" => {
            let descripcion = *momentos_conexion().choose(rng).unwrap();
            let regalo = *fortalezas_personales().choose(rng).unwrap();
            let alegria = *pequenas_alegrias().choose(rng).unwrap();
            (
                format!("El Encuentro de {}", nombre),
                descripcion.to_string(),
                format!("Le dejó: {}", regalo),
                format!("Su eco: Cuando siente {}, recuerda que la conexión es posible", alegria),
            )
        },
        "logro" => {
            let descripcion = *momentos_logro().choose(rng).unwrap();
            let esperanza = *esperanzas_suenos().choose(rng).unwrap();
            (
                format!("La Victoria de {}", nombre),
                descripcion.to_string(),
                "La certeza de que puede lograrlo de nuevo".to_string(),
                format!("Su eco: Ahora sueña con {}", esperanza),
            )
        },
        "bondad" => {
            let plantillas = vec![
                format!("Un desconocido ofreció ayuda a {} sin esperar nada. Cambió su perspectiva.", nombre),
                format!("Alguien que tenía razones para odiarlo eligió la compasión. {} nunca lo olvidó.", nombre),
                format!("En su momento más oscuro, una luz inesperada apareció para {}.", nombre),
            ];
            let descripcion = plantillas.choose(rng).unwrap().clone();
            let alegria = *pequenas_alegrias().choose(rng).unwrap();
            (
                format!("La Gracia de {}", nombre),
                descripcion,
                "Fe renovada en que la bondad existe sin condiciones".to_string(),
                format!("Su eco: Ahora busca ofrecer lo mismo cuando ve a alguien disfrutar de {}", alegria),
            )
        },
        _ => {
            let fortaleza = *fortalezas_personales().choose(rng).unwrap();
            let esperanza = *esperanzas_suenos().choose(rng).unwrap();
            (
                format!("El Don de {}", nombre),
                fortaleza.to_string(),
                "Un regalo que nadie puede quitarle".to_string(),
                format!("Su eco: Lo impulsa hacia {}", esperanza),
            )
        }
    }
}

/// Genera una fortaleza personal
pub fn generar_fortaleza(rng: &mut impl Rng) -> String {
    fortalezas_personales().choose(rng).unwrap().to_string()
}

/// Genera una pequeña alegría
pub fn generar_pequena_alegria(rng: &mut impl Rng) -> String {
    pequenas_alegrias().choose(rng).unwrap().to_string()
}

/// Genera un sueño o esperanza
pub fn generar_esperanza(rng: &mut impl Rng) -> String {
    esperanzas_suenos().choose(rng).unwrap().to_string()
}

use crate::core::Mundo;

pub fn generar_motivacion_antagonista(tipo: &str, mundo: &Mundo) -> String {
    let base = match tipo {
        "Envidia" => match mundo {
            Mundo::Noir | Mundo::Realista => vec![
                "No odia al héroe por su éxito, sino porque es el único que logra dormir tranquilo en una ciudad insomne.",
                "Cree que el sistema está amañado y el héroe es la prueba viviente de la injusticia del azar.",
                "Quiere demostrar que la integridad moral del héroe es solo falta de una oferta lo suficientemente alta."
            ],
            Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
                "Cree que los dioses le robaron el destino que ahora ostenta el héroe.",
                "Busca devorar la 'luz' del héroe porque su propia alma se ha vuelto un vacío insaciable.",
                "Quiere probar que la profecía eligió al hermano equivocado."
            ],
            Mundo::SciFiPostApocaliptico | Mundo::Western => vec![
                "En un mundo de escasez, ve la esperanza del héroe como un desperdicio de recursos vitales.",
                "Odia que el héroe mantenga sus manos limpias mientras él tuvo que ahogarse en sangre para sobrevivir.",
                "Cree que la compasión del héroe debilitará a la tribu y los matará a todos."
            ],
            _ => vec![
                "Siente que el universo le debe lo que el héroe tiene por derecho de nacimiento.",
                "Su envidia no es material, es ontológica: quiere *ser* el otro, robándole su esencia.",
                "Ve en el héroe el reflejo de todo lo que él mató en sí mismo para ganar poder."
            ]
        },
        "Ideologia" => match mundo {
            Mundo::SciFiCyberpunk | Mundo::Steampunk => vec![
                "Cree que el libre albedrío es un bug en el sistema que causa todo el sufrimiento humano; busca el orden absoluto.",
                "Quiere acelerar el colapso de la sociedad actual para construir una utopía mecánica sobre sus cenizas.",
                "Considera que el progreso tecnológico exige sacrificios que el héroe es demasiado sentimental para aceptar."
            ],
            Mundo::FantasiaMedieval | Mundo::Mitologico => vec![
                "Sirve a un orden cósmico antiguo que considera la humanidad una plaga descontrolada.",
                "Cree que la magia debe ser erradicada para que el hombre sea verdaderamente libre de los dioses.",
                "Busca restaurar un imperio perdido, convencido de que la paz solo existe bajo un yugo de hierro."
            ],
            Mundo::Western | Mundo::HistoricoModerno => vec![
                "Cree que la 'Civilización' es una mentira para los débiles; solo la ley del más fuerte es honesta.",
                "Lucha por preservar una tradición brutal, creyendo que el cambio destruirá el alma de su pueblo.",
                "Está convencido de que el caos es el estado natural y cualquier intento de orden es tiranía."
            ],
            _ => vec![
                "Cree sinceramente que está salvando al mundo, y que el héroe es el verdadero villano por impedirlo.",
                "Su utopía requiere purgar a la mitad de la población; lo ve como una cirugía necesaria, no un crimen.",
                "No busca poder personal, sino imponer una Verdad que considera absoluta e innegociable."
            ]
        },
        "Obstaculo" => match mundo {
            Mundo::Noir | Mundo::FantasiaUrbana => vec![
                "No tiene nada personal contra el héroe; simplemente es un profesional haciendo un trabajo sucio.",
                "Representa la inercia de una burocracia corrupta que aplasta cualquier intento de cambio.",
                "Es un superviviente que vendería a su madre (o al héroe) para ver un día más."
            ],
            Mundo::SciFiSpace | Mundo::SciFiCyberpunk => vec![
                "Es una IA o entidad cuya programación lógica concluye que la eliminación del héroe es la ruta óptima.",
                "Es un funcionario leal a un sistema opresivo, incapaz de cuestionar órdenes por diseño o adoctrinamiento.",
                "Representa las fuerzas del mercado o la naturaleza: indiferente, implacable e inevitable."
            ],
            _ => vec![
                "Es una fuerza de la naturaleza personificada; no se le puede odiar más de lo que se odia a un huracán.",
                "Su objetivo es legítimo y necesario, y lamentablemente, el héroe está en su camino.",
                "Protege un secreto o un lugar, y su lealtad es absoluta, sin importar la moralidad."
            ]
        },
        "PuraMaldad" => match mundo {
            Mundo::FantasiaOscura | Mundo::FantasiaMedieval => vec![
                "Busca la apoteosis a través del sufrimiento ajeno; el dolor es su sacramento.",
                "Es un avatar del vacío; su única motivación es descoser la realidad hilo a hilo.",
                "Colecciona almas rotas como quien colecciona mariposas, fascinado por sus estertores."
            ],
            Mundo::Noir | Mundo::Realista => vec![
                "Un nihilista activo que quiere demostrar que la moralidad es una broma pesada.",
                "Alguien que 'solo quiere ver el mundo arder' porque el orden le aburre profundamente.",
                "Un sociópata refinado que manipula vidas humanas como si fuera ajedrez, por pura curiosidad intelectual."
            ],
            _ => vec![
                "Su maldad es artística; busca crear la tragedia perfecta con la vida del héroe.",
                "No reconoce la humanidad en otros; para él, todos son NPCs en su juego solipsista.",
                "Una entidad antigua aburrida de la eternidad, jugando con mortales para sentir algo."
            ]
        },
        _ => vec![
            "Una obsesión oscura y personal que ni él mismo entiende del todo.",
            "Una vendetta heredada que ha consumido su propia identidad.",
            "La creencia de que el fin justifica cualquier medio, por horroroso que sea."
        ]
    };
    
    use rand::seq::SliceRandom;
    base.choose(&mut rand::thread_rng()).unwrap_or(&"Una ambición oscura.").to_string()
}

