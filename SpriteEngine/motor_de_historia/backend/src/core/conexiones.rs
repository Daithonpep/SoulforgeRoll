//! Sistema de Conexiones Entrelazadas y Arcos Narrativos
//! Complementa biografia.rs con sistemas más profundos

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Mundo, Rol};

// ============================================================
// TIPOS DE ARCO NARRATIVO EXPANDIDOS
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TipoArcoExpandido {
    CaidaAscenso,          // Comienza mal, termina bien
    InocenciaSabiduria,    // Aprende verdades difíciles
    AislamientoConexion,   // De solo a acompañado
    CadenasLibertad,       // Escape de limitaciones
    HeridaSanacion,        // Proceso de sanar
    FirmeEnTormenta,       // Mantiene quien es bajo presión
    ResistenciaAceptacion, // Deja de luchar
    Convertirse,           // Se transforma en algo nuevo
    Integracion,           // Une partes fragmentadas
    Espejo,                // Se refleja en otro
    Catalizador,           // Cambia por influencia de otro
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcoExpandido {
    pub tipo: TipoArcoExpandido,
    pub estado_inicial: String,
    pub punto_giro: String,
    pub estado_actual: String,
    pub pregunta_central: String,
    pub leccion: String,
    pub futuros_posibles: Vec<String>,
}

// ============================================================
// TIPOS DE CONEXIÓN ENTRE PERSONAJES
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TipoConexion {
    Sangre,              // Familia biológica
    FamiliaElegida,      // Familia elegida
    HeridaCompartida,    // Trauma compartido
    TriunfoCompartido,   // Victoria juntos
    Trinchera,           // Sobrevivieron algo juntos
    MentorEstudiante,    
    ProtectorProtegido,  
    Rivales,             
    Socios,              
    Catalizador,         // Uno transformó al otro
    Espejo,              // Se reflejan mutuamente
    Amor,                
    Deuda,               
    Herida,              // Se dañaron
    Juramento,           
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentoCompartidoExpandido {
    pub descripcion: String,
    pub perspectiva_a: String,
    pub perspectiva_b: String,
    pub impacto_a: String,
    pub impacto_b: String,
    pub aun_resuena: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConexionPersonajes {
    pub nombre_a: String,
    pub nombre_b: String,
    pub tipo: TipoConexion,
    pub momento_origen: MomentoCompartidoExpandido,
    pub momentos_clave: Vec<MomentoCompartidoExpandido>,
    pub estado_actual: String,
    pub tension_actual: String,
    pub salud: f32,      // 0 (tóxica) a 1 (sana)
    pub profundidad: f32, // 0 (superficial) a 1 (profunda)
}

// ============================================================
// CATEGORÍAS DE GRACIA EXPANDIDAS
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CategoriaGracia {
    SerVisto,
    Pertenencia,
    AmorIncondicional,
    Amistad,
    Mentoria,
    Dominio,
    Descubrimiento,
    Valentia,
    Creacion,
    Asombro,
    Belleza,
    Paz,
    AlegriaP,
    Perdon,
    Liberacion,
    Aceptacion,
    Proposito,
    Legado,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentoGraciaExpandido {
    pub categoria: CategoriaGracia,
    pub titulo: String,
    pub descripcion: String,
    pub detalle_sensorial: String,
    pub regalo: String,
    pub eco: String,
    pub leccion: String,
    pub fragilidad: f32,
    pub puede_volverse_herida: bool,
}

// ============================================================
// GENERADORES
// ============================================================

pub fn generar_arco_expandido(rng: &mut impl Rng, nombre: &str, rol: &Rol) -> ArcoExpandido {
    let tipo = match rol {
        Rol::Heroe => *[TipoArcoExpandido::CaidaAscenso, TipoArcoExpandido::FirmeEnTormenta, TipoArcoExpandido::HeridaSanacion].choose(rng).unwrap(),
        Rol::Villano => *[TipoArcoExpandido::CaidaAscenso, TipoArcoExpandido::Convertirse, TipoArcoExpandido::ResistenciaAceptacion].choose(rng).unwrap(),
        Rol::Mentor => *[TipoArcoExpandido::InocenciaSabiduria, TipoArcoExpandido::Integracion].choose(rng).unwrap(),
        _ => *[TipoArcoExpandido::Convertirse, TipoArcoExpandido::AislamientoConexion, TipoArcoExpandido::HeridaSanacion].choose(rng).unwrap(),
    };
    
    let (inicial, giro, actual, pregunta, leccion, futuros) = match tipo {
        TipoArcoExpandido::CaidaAscenso => (
            format!("{} estaba en el fondo de un pozo que parecía no tener salida", nombre),
            "el momento donde elegir rendirse o seguir fue real, y eligió seguir".into(),
            format!("{} está de pie, no sin cicatrices, pero de pie", nombre),
            "¿Se puede volver a levantarse después de caer tan bajo?".into(),
            "La resiliencia no es no caer, es levantarse".into(),
            vec!["consolidar lo ganado".into(), "ayudar a otros en el mismo pozo".into()],
        ),
        TipoArcoExpandido::HeridaSanacion => (
            format!("{} cargaba heridas que sangraban hacia adentro", nombre),
            "el momento donde ignorar dejó de ser opción".into(),
            format!("{} está en proceso, porque sanar no es un destino", nombre),
            "¿Se puede sanar completamente, o solo aprender a vivir con?".into(),
            "Sanar no es olvidar, es integrar".into(),
            vec!["días donde la herida no define el día".into(), "ayudar a otros en su proceso".into()],
        ),
        TipoArcoExpandido::AislamientoConexion => (
            format!("{} estaba solo, con murallas que mantenían todo afuera", nombre),
            "alguien que no aceptó un no por respuesta".into(),
            format!("{} tiene lazos que se siente seguro sostener", nombre),
            "¿El riesgo de la conexión vale el dolor potencial?".into(),
            "La vulnerabilidad es precio y puerta de la conexión".into(),
            vec!["profundizar los vínculos existentes".into(), "enfrentar el miedo cuando amenace".into()],
        ),
        TipoArcoExpandido::FirmeEnTormenta => (
            format!("{} tenía un centro que sería puesto a prueba", nombre),
            "el momento donde ceder habría sido más fácil".into(),
            format!("{} sigue igual en esencia, diferente en expresión", nombre),
            "¿La adversidad revela el carácter o lo forja?".into(),
            "Saber quién eres es ancla en la tormenta".into(),
            vec!["nuevas tormentas con más confianza".into(), "ser ancla para otros".into()],
        ),
        TipoArcoExpandido::Convertirse => (
            format!("{} era quien las circunstancias le hicieron ser", nombre),
            "el permiso —interno o externo— de cambiar".into(),
            format!("{} habita una identidad elegida, no heredada", nombre),
            "¿Se elige quién ser o se descubre?".into(),
            "La identidad es más fluida de lo que enseñan".into(),
            vec!["integrar versiones pasadas con presente".into(), "nuevas transformaciones".into()],
        ),
        _ => (
            format!("{} buscaba entenderse a sí mismo", nombre),
            "un espejo que mostró verdades difíciles".into(),
            format!("{} se conoce mejor, para bien o para mal", nombre),
            "¿Quién soy realmente debajo de todo?".into(),
            "Conocerse es proceso, no destino".into(),
            vec!["integrar lo descubierto".into(), "seguir explorando".into()],
        ),
    };
    
    ArcoExpandido {
        tipo,
        estado_inicial: inicial,
        punto_giro: giro,
        estado_actual: actual,
        pregunta_central: pregunta,
        leccion,
        futuros_posibles: futuros,
    }
}

pub fn generar_conexion_profunda(
    rng: &mut impl Rng,
    nombre_a: &str,
    nombre_b: &str,
    _mundo: &Mundo,
) -> ConexionPersonajes {
    let tipos_posibles = vec![
        TipoConexion::FamiliaElegida,
        TipoConexion::HeridaCompartida,
        TipoConexion::Trinchera,
        TipoConexion::MentorEstudiante,
        TipoConexion::Rivales,
        TipoConexion::Catalizador,
        TipoConexion::Espejo,
    ];
    let tipo = *tipos_posibles.choose(rng).unwrap();
    
    // Generadores de texto modular
    let (origen_desc, persp_a, persp_b, imp_a, imp_b) = match tipo {
        TipoConexion::FamiliaElegida => {
            let acciones = vec!["Se eligieron mutuamente", "Se encontraron", "Sus caminos coincidieron", "El destino los colisionó"];
            let contextos = vec!["en un momento donde la soledad pesaba demasiado", "cuando el resto del mundo les dio la espalda", "en el silencio de una noche cualquiera", "durante una crisis que rompió todo lo demás"];
            let lazos = vec!["reconociendo algo familiar en la mirada del otro", "y supieron que ya no caminarían solos", "formando un pacto sin necesidad de palabras", "llenando los vacíos que la familia de sangre dejó"];
            
            (
                format!("{} {}, {}.", acciones.choose(rng).unwrap(), contextos.choose(rng).unwrap(), lazos.choose(rng).unwrap()),
                format!("{} encontró en {} lo que siempre buscó sin saberlo.", nombre_a, nombre_b),
                format!("Para {}, {} se convirtió en el ancla que nunca tuvo.", nombre_b, nombre_a),
                "Redefinió su concepto de lealtad.".into(),
                "Encontró un lugar donde ser vulnerable es seguro.".into(),
            )
        },
        TipoConexion::HeridaCompartida => {
            let eventos = vec!["El mismo trauma", "Una pérdida idéntica", "El eco de un fracaso similar", "Haber sobrevivido a lo mismo"];
            let efectos = vec!["los marcó a ambos para siempre", "creó un lenguaje secreto entre ellos", "los aisló del resto pero los unió entre sí", "dejó una cicatriz que solo el otro puede tocar"];
            
            (
                format!("{} {}, y eso {}.", eventos.choose(rng).unwrap(), efectos.choose(rng).unwrap(), vec!["sin remedio", "profundamente", "de forma irrevocable"].choose(rng).unwrap()),
                format!("{} sabe que {} no necesita explicaciones para entender el dolor.", nombre_a, nombre_b),
                format!("{} ve su propia sombra reflejada en los ojos de {}.", nombre_b, nombre_a),
                "No está solo en la oscuridad.".into(),
                "Alguien más carga el mismo peso.".into(),
            )
        },
        TipoConexion::Trinchera => {
            let situaciones = vec!["Sobrevivieron a lo imposible juntos", "Pasaron por el infierno y volvieron", "Compartieron el último recurso cuando no había nada", "Se cubrieron las espaldas cuando todos huían"];
            let vinculos = vec!["forjando una confianza de acero", "creando una deuda de vida impagable", "atando sus destinos con nudos ciegos", "aprendiendo a respirar al mismo ritmo"];
            
            (
                format!("{}. Eso terminó {}.", situaciones.choose(rng).unwrap(), vinculos.choose(rng).unwrap()),
                format!("{} confía su vida a {} sin dudar un segundo.", nombre_a, nombre_b),
                format!("{} sabe que {} es la única certeza en el caos.", nombre_b, nombre_a),
                "Certeza absoluta de respaldo.".into(),
                "Un vínculo probado en fuego.".into(),
            )
        },
        TipoConexion::MentorEstudiante => {
            let inicios = vec!["Uno vio una chispa, el otro buscaba fuego", "Fue un encuentro de necesidad mutua", "El maestro apareció cuando el estudiante estaba roto", "El estudiante desafió al maestro y ganó su respeto"];
            
            (
                format!("{}. La relación creció desde ahí.", inicios.choose(rng).unwrap()),
                format!("{} vio en {} alguien digno de su legado.", nombre_a, nombre_b),
                format!("{} encontró en {} la brújula que le faltaba.", nombre_b, nombre_a),
                "El peso de guiar a otro.".into(),
                "El desafío de superar al guía.".into(),
            )
        },
        _ => (
             format!("{} y {} chocaron como planetas en órbita.", nombre_a, nombre_b),
             format!("Para {}, {} es un misterio.", nombre_a, nombre_b),
             format!("{} aún intenta descifrar a {}.", nombre_b, nombre_a),
             "Un cambio de perspectiva radical.".into(),
             "Preguntas nuevas sin respuesta.".into(),
        )
    };
    
    let momento_origen = MomentoCompartidoExpandido {
        descripcion: origen_desc,
        perspectiva_a: persp_a,
        perspectiva_b: persp_b,
        impacto_a: imp_a,
        impacto_b: imp_b,
        aun_resuena: true,
    };
    
    // Generar momentos clave dinámicos
    let intros = vec!["Hubo una noche donde", "Durante el viaje,", "En un momento de calma,", "Bajo la presión del momento,"];
    let acciones_clave = vec!["las caretas cayeron", "una verdad salió a la luz", "un silencio dijo más que mil palabras", "una decisión imposible fortaleció el lazo"];
    let consecuencias_clave = vec!["y nada volvió a ser igual", "marcando un antes y un después", "sellando el pacto", "abriendo una puerta nueva"];
    
    let desc_clave = format!("{} {} {}.", intros.choose(rng).unwrap(), acciones_clave.choose(rng).unwrap(), consecuencias_clave.choose(rng).unwrap());
    
    let momento_clave = MomentoCompartidoExpandido {
        descripcion: desc_clave,
        perspectiva_a: format!("{} sintió que por fin podía respirar.", nombre_a),
        perspectiva_b: format!("{} entendió la profundidad de lo que compartían.", nombre_b),
        impacto_a: "Vulnerabilidad permitida.".into(),
        impacto_b: "Conexión validada.".into(),
        aun_resuena: true,
    };
    
    let estados = vec![
        "mantienen lo que construyeron con cuidado",
        "trabajan en reparar grietas recientes",
        "están más sincronizados que nunca",
        "navegan un silencio cómodo",
        "se entienden con una mirada",
    ];
    
    let tensiones = vec![
        "palabras no dichas que pesan",
        "el miedo a perder lo construido",
        "caminos que empiezan a divergir",
        "secretos antiguos que arañan la superficie",
    ];
    
    ConexionPersonajes {
        nombre_a: nombre_a.to_string(),
        nombre_b: nombre_b.to_string(),
        tipo,
        momento_origen,
        momentos_clave: vec![momento_clave],
        estado_actual: estados.choose(rng).unwrap().to_string(),
        tension_actual: tensiones.choose(rng).unwrap().to_string(),
        salud: rng.gen_range(0.5..0.9),
        profundidad: rng.gen_range(0.5..0.9),
    }
}

pub fn generar_gracia_expandida(rng: &mut impl Rng, nombre: &str) -> MomentoGraciaExpandido {
    let categorias = vec![
        (CategoriaGracia::SerVisto, "La Mirada que Reconoce", 
         format!("Alguien miró a {} y vio —no la máscara, no el rol— sino a la persona debajo.", nombre),
         "El silencio cómodo que siguió. La sensación de no tener que explicar nada.",
         "la certeza de que ser visto es posible",
         "Busca esos ojos en cada rostro nuevo",
         "Existo más allá de lo que muestro"),
        (CategoriaGracia::Pertenencia, "El Lugar en la Mesa",
         format!("Había un espacio guardado. Un sitio que era suyo, sin tener que ganarlo. {} perteneció.", nombre),
         "El ruido de conversaciones donde su voz era una más, no invitada sino esperada.",
         "la memoria de lo que significa pertenecer",
         "Busca mesas donde haya un lugar para todos",
         "Hay espacios donde encajo"),
        (CategoriaGracia::Amistad, "La Risa Compartida",
         format!("Un momento donde la risa brotó, genuina, contagiosa. {} y alguien más, riendo hasta que dolían las costillas.", nombre),
         "El sonido de dos risas mezclándose. Lágrimas de alegría.",
         "la memoria de que la alegría compartida existe",
         "Busca personas que le hagan reír así",
         "La alegría se multiplica al compartirse"),
        (CategoriaGracia::Valentia, "El Momento de Valentía",
         format!("{} tuvo miedo. Mucho miedo. Pero actuó de todas formas.", nombre),
         "El corazón latiendo en la garganta. Las manos temblando pero moviéndose.",
         "conocimiento de su propia valentía",
         "Recuerda ese momento cuando el miedo paraliza",
         "El coraje no es ausencia de miedo"),
        (CategoriaGracia::Paz, "El Silencio Perfecto",
         format!("Por un momento, la mente de {} se calló. No había pasado que lamentar ni futuro que temer.", nombre),
         "La respiración que se hace lenta. La tensión que se disuelve sin esfuerzo.",
         "la experiencia de paz real",
         "Busca ese silencio, sabe que existe",
         "La paz es posible, aunque sea breve"),
        (CategoriaGracia::Proposito, "La Llamada",
         format!("Algo hizo clic. {} supo, con una certeza que no podía explicar, para qué estaba aquí.", nombre),
         "El hormigueo de reconocimiento. La energía que surge de la claridad.",
         "un norte en el mapa interno",
         "Vuelve a esa brújula cuando se pierde",
         "Hay algo que solo yo puedo hacer"),
    ];
    
    let (cat, titulo, desc, sensorial, regalo, eco, leccion) = categorias.choose(rng).unwrap().clone();
    
    MomentoGraciaExpandido {
        categoria: cat,
        titulo: titulo.to_string(),
        descripcion: desc.to_string(),
        detalle_sensorial: sensorial.to_string(),
        regalo: regalo.to_string(),
        eco: eco.to_string(),
        leccion: leccion.to_string(),
        fragilidad: rng.gen_range(0.2..0.7),
        puede_volverse_herida: rng.gen_bool(0.3),
    }
}
