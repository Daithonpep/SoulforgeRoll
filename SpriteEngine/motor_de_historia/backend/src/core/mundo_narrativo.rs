//! Contenido narrativo específico por mundo.
//! 
//! Este módulo proporciona vocabulario y frases temáticas basadas en el mundo
//! seleccionado, haciendo que cada generación sea única y coherente con el setting.

use rand::prelude::*;
use crate::core::Mundo;

// ============================================================================
// ELEMENTOS TEMÁTICOS POR MUNDO
// ============================================================================

/// Elementos y referencias temáticas para cada mundo
pub fn elementos_tematicos(mundo: &Mundo) -> Vec<&'static str> {
    match mundo {
        Mundo::MitologiaNordica => vec![
            "las runas ancestrales", "el hielo eterno", "el Valhalla", "las nornas del destino",
            "los cuervos de Odín", "el hidromiel sagrado", "los drakkars", "los berserkers",
            "el Yggdrasil", "los lobos de Fenrir", "el Ragnarök", "las valquirias",
            "los jötnar", "el Mítico martillo", "los nueve mundos", "la aurora boreal"
        ],
        Mundo::MitologiaGriega => vec![
            "el oráculo de Delfos", "el monte Olimpo", "el río Estigia", "el néctar divino",
            "las moiras", "el laberinto", "la égida", "los titanes", "el fuego de Prometeo",
            "las sirenas", "la hidra", "el carro del sol", "el destino trágico", "la hybris"
        ],
        Mundo::FantasiaMedieval | Mundo::FantasiaOscura => vec![
            "las torres de hechiceros", "los dragones antiguos", "las espadas encantadas",
            "los códices prohibidos", "los gremios secretos", "las coronas malditas",
            "los bosques encantados", "los portales dimensionales", "las profecías olvidadas"
        ],
        Mundo::SciFiCyberpunk => vec![
            "los implantes neurales", "las IAs rebeldes", "la red oscura", "los yakuza",
            "las megacorporaciones", "el neón eterno", "los androides", "la lluvia ácida",
            "los mercenarios corporativos", "los slums verticales", "el ciberespacio"
        ],
        Mundo::JaponFeudal => vec![
            "las katanas ancestrales", "el bushido", "los templos en la niebla", "los yokai",
            "los shogunes", "las geishas espías", "los ronin", "las flores de cerezo",
            "los ninjas", "el honor familiar", "los daimyō", "el seppuku"
        ],
        Mundo::Noir => vec![
            "las oficinas de detective", "las femme fatales", "el whisky barato", "los callejones lluviosos",
            "los policías corruptos", "los clubes de jazz", "los revólveres ocultos",
            "las cartas anónimas", "el humo de cigarrillo", "los secretos enterrados"
        ],
        Mundo::Steampunk => vec![
            "los autómatas de cuerda", "los dirigibles de guerra", "las sociedades secretas",
            "el éter luminoso", "las armas de rayo", "los relojes imposibles",
            "los genios inventores", "las damas científicas", "el vapor omnipresente"
        ],
        Mundo::Western => vec![
            "los duelos al mediodía", "los trenes del oro", "los sheriffs solitarios",
            "los bandidos legendarios", "los pueblos fantasma", "las minas malditas",
            "los nativos espirituales", "los caballos salvajes", "la frontera inexplorada"
        ],
        Mundo::ChinaImperial | Mundo::Wuxia => vec![
            "los monasterios de artes marciales", "el qi interior", "los eruditos de bambú",
            "las sectas secretas", "el emperador del dragón", "los venenos sutiles",
            "las rutas de la seda", "los guerreros voladores", "la poesía del combate"
        ],
        Mundo::AnimeFantasia | Mundo::Anime => vec![
            "las academias de héroes", "los torneos de poder", "los lazos de amistad",
            "las transformaciones épicas", "los flashbacks traumáticos", "los villanos carismáticos",
            "las promesas inquebrantables", "los sacrificios heroicos", "el poder del corazón"
        ],
        _ => vec![
            "los secretos olvidados", "las profecías antiguas", "los artefactos poderosos",
            "los reinos en conflicto", "las alianzas rotas", "los destinos entrelazados"
        ]
    }
}

/// Eventos catalizadores específicos del mundo
pub fn eventos_catalizadores(mundo: &Mundo) -> Vec<&'static str> {
    match mundo {
        Mundo::MitologiaNordica => vec![
            "la masacre de su clan durante la luna negra",
            "una maldición divina por ofender a los Aesir",
            "la visión de su muerte en las runas",
            "ser testigo del sacrificio de su líder en el Árbol del Mundo",
            "la invasión de los jötnar que destruyó su hogar",
            "el juramento de sangre que lo ató a un destino oscuro"
        ],
        Mundo::MitologiaGriega => vec![
            "una profecía del oráculo que lo maldijo desde el nacimiento",
            "la ira de un dios caprichoso que destruyó todo lo que amaba",
            "ser elegido como campeón en una guerra divina que no pidió",
            "la hybris de desafiar al Olimpo y pagar el precio",
            "ver a su ciudad arder por las maquinaciones de los inmortales"
        ],
        Mundo::JaponFeudal => vec![
            "la destrucción de su clan por orden del shogun",
            "ser forzado a cometer seppuku por un crimen que no cometió",
            "la traición de su señor feudal en la noche de los mil cortes",
            "descubrir que era descendiente de una línea maldita",
            "la masacre de los monjes que lo criaron"
        ],
        Mundo::SciFiCyberpunk => vec![
            "el borrado de su identidad por una megacorporación",
            "despertar en un cuerpo que no era suyo",
            "descubrir que sus recuerdos eran implantes falsos",
            "ver morir a su equipo en un trabajo que era una trampa",
            "ser traicionado por la IA en la que confiaba"
        ],
        Mundo::Noir => vec![
            "el asesinato de su socio que nadie quiso investigar",
            "descubrir que el amor de su vida trabajaba para el enemigo",
            "ser acusado injustamente y perderlo todo en una noche",
            "ver cómo el sistema protegía a los verdaderos criminales",
            "la traición del policía que consideraba su amigo"
        ],
        Mundo::Western => vec![
            "la masacre de su familia por forajidos que la ley ignoró",
            "perder todo en el incendio que destruyó el pueblo",
            "ser traicionado por su propio hermano por una bolsa de oro",
            "ver morir a su rancho por la sequía y la codicia de los banqueros",
            "el duelo que lo dejó como el último en pie, pero vacío"
        ],
        _ => vec![
            "un evento traumático que destruyó todo lo que conocía",
            "una traición que cambió su visión del mundo",
            "una pérdida que dejó un vacío imposible de llenar",
            "un secreto revelado que derrumbó su identidad",
            "un sacrificio que lo marcó para siempre"
        ]
    }
}

/// Genera una herida temática basada en el mundo
pub fn generar_herida_tematica(rng: &mut impl Rng, mundo: &Mundo) -> String {
    let eventos = eventos_catalizadores(mundo);
    let elementos = elementos_tematicos(mundo);
    
    let evento = eventos.choose(rng).unwrap_or(&"un evento traumático");
    let elemento = elementos.choose(rng).unwrap_or(&"fuerzas oscuras");
    
    let plantillas = vec![
        format!("{} marcó su alma. Desde entonces, {} le recuerda lo que perdió.", evento, elemento),
        format!("Todo cambió cuando {}. Ahora lleva el peso de {} en cada paso.", evento, elemento),
        format!("{} — un momento que divide su vida en 'antes' y 'después'. {} ya no significa lo mismo.", evento, elemento),
    ];
    
    plantillas.choose(rng).unwrap().clone()
}

/// Ganchos narrativos específicos del mundo
pub fn ganchos_narrativos_mundo(rng: &mut impl Rng, mundo: &Mundo, nombre: &str) -> Vec<String> {
    let ganchos: Vec<String> = match mundo {
        Mundo::MitologiaNordica => vec![
            format!("{} ha visto su muerte en las runas. Ahora corre contra el destino que las nornas tejieron.", nombre),
            format!("Un cuervo de Odín sigue a {} a todas partes. Aún no sabe si es protección o vigilancia.", nombre),
            format!("{} lleva una espada que susurra en nórdico antiguo. Dice que su verdadero portador aún no ha nacido.", nombre),
            format!("Los jötnar reconocen a {} cuando lo ven. Algo en su sangre no es del todo humano.", nombre),
        ],
        Mundo::MitologiaGriega => vec![
            format!("El oráculo profetizó a {} grandeza o ruina total. El destino aún no ha decidido cuál.", nombre),
            format!("{} ofendió a un dios menor. La maldición es sutil pero implacable.", nombre),
            format!("{} lleva sangre divina en sus venas. Los monstruos pueden olerla.", nombre),
            format!("Una deuda con el inframundo pende sobre la cabeza de {}. Hades siempre cobra.", nombre),
        ],
        Mundo::JaponFeudal => vec![
            format!("{} porta una katana sin nombre que ningún herrero reconoce. Su filo es perfecto.", nombre),
            format!("Un yokai protege a {} desde las sombras. El precio de esta protección aún no ha sido revelado.", nombre),
            format!("{} es el último de su escuela de esgrima. Alguien está cazando a todos los demás.", nombre),
            format!("El shogun quiere a {} muerto. El emperador lo quiere vivo. {} solo quiere paz.", nombre, nombre),
        ],
        Mundo::SciFiCyberpunk => vec![
            format!("{} tiene un chip en su cabeza que no recuerda haberse instalado. A veces muestra cosas.", nombre),
            format!("Una megacorporación ha puesto precio a la cabeza de {}. El motivo está en datos que no recuerda tener.", nombre),
            format!("Hay una IA que contacta a {} en sueños. Dice que es la última esperanza de algo.", nombre),
            format!("El DNA de {} está en una lista negra. Cada scanner biométrico es una trampa potencial.", nombre),
        ],
        Mundo::Noir => vec![
            format!("{} tiene una foto en su escritorio. La mujer de la foto lleva tres años 'desaparecida'.", nombre),
            format!("{} tiene un caso que nunca pudo resolver. Los sospechosos siguen muriendo uno a uno.", nombre),
            format!("Alguien envía cartas a {} con fechas futuras. Las predicciones siempre se cumplen.", nombre),
            format!("{} conoce un secreto que podría destruir a la familia más poderosa de la ciudad.", nombre),
        ],
        Mundo::Western => vec![
            format!("{} busca al hombre que mató a su familia. El rastro tiene diez años de frío.", nombre),
            format!("Hay un precio en la cabeza de {} que sube cada mes. Alguien lo quiere muy muerto.", nombre),
            format!("{} lleva un mapa de una mina de oro que nunca existió. O eso dicen.", nombre),
            format!("Un pueblo fantasma llama a {} en sueños. Dice que dejó algo sin terminar allí.", nombre),
        ],
        _ => vec![
            format!("{} carga con un secreto que podría cambiar el equilibrio del mundo.", nombre),
            format!("Hay alguien del pasado de {} que pronto reaparecerá. Y no viene en paz.", nombre),
            format!("Una profecía antigua menciona el nombre de {}. Aún no sabe qué papel juega.", nombre),
            format!("Algo observa a {} desde las sombras, esperando el momento adecuado.", nombre),
        ]
    };
    
    ganchos.choose_multiple(rng, 2).cloned().collect()
}

/// Genera una sombra temática basada en el mundo
pub fn generar_sombra_tematica(rng: &mut impl Rng, mundo: &Mundo, nombre: &str) -> (String, String) {
    let sombras: Vec<(String, String)> = match mundo {
        Mundo::MitologiaNordica => vec![
            (format!("{} guarda dentro de sí la misma furia berserker que destruyó a su padre.", nombre), "cuando la batalla lo consume y pierde el control".to_string()),
            (format!("Bajo su honor, hay un pragmatismo despiadado. {} haría cualquier cosa por victoria.", nombre), "cuando nadie está mirando y el fin justifica los medios".to_string()),
            (format!("El miedo a morir olvidado carcome a {}. Busca gloria no por honor, sino por terror a la irrelevancia.", nombre), "cuando su legado parece insignificante".to_string()),
        ],
        Mundo::MitologiaGriega => vec![
            (format!("{} lleva la hybris de sus ancestros. Cree que merece más que los mortales comunes.", nombre), "cuando su orgullo es desafiado".to_string()),
            (format!("La fatalidad griega vive en {}. Acepta demasiado fácilmente que el destino es inevitable.", nombre), "cuando las cosas van mal y se rinde prematuramente".to_string()),
            (format!("La envidia de los dioses ha tocado a {}. Destruye inconscientemente lo que más ama.", nombre), "cuando alcanza la felicidad".to_string()),
        ],
        Mundo::JaponFeudal => vec![
            (format!("Bajo la máscara de honor, {} guarda un rencor que bordea el odio.", nombre), "cuando se enfrenta a quienes representan lo que perdió".to_string()),
            (format!("{} ha matado más de lo que admite. No todas las muertes fueron honorables.", nombre), "cuando el pasado amenaza con salir a la luz".to_string()),
            (format!("El deber y el deseo están en guerra dentro de {}. Algún día uno destruirá al otro.", nombre), "cuando debe elegir entre lealtad y amor".to_string()),
        ],
        Mundo::SciFiCyberpunk => vec![
            (format!("{} está más máquina de lo que admite. Su humanidad se desvanece con cada upgrade.", nombre), "cuando debe elegir entre eficiencia y empatía".to_string()),
            (format!("La adicción a la adrenalina consume a {}. Necesita el peligro como otros necesitan aire.", nombre), "cuando la vida se vuelve demasiado tranquila".to_string()),
            (format!("Bajo el cinismo, hay un idealista muerto. {} odia lo que se ha convertido.", nombre), "cuando ve a alguien con la esperanza que perdió".to_string()),
        ],
        Mundo::Noir => vec![
            (format!("{} bebe para olvidar. El problema es que funciona cada vez menos.", nombre), "cuando los recuerdos amenazan con ahogarlo".to_string()),
            (format!("{} juró nunca cruzar una línea. La ha borrado tantas veces que ya no sabe dónde estaba.", nombre), "cuando el fin parece justificar cualquier medio".to_string()),
            (format!("La ciudad ha corrompido a {} más de lo que admite. Ya no es quien vino a salvarla.", nombre), "cuando se mira al espejo y no reconoce lo que ve".to_string()),
        ],
        Mundo::Western => vec![
            (format!("{} ha matado a hombres que no merecían morir. Sus fantasmas lo visitan en noches silenciosas.", nombre), "cuando la violencia parece demasiado fácil".to_string()),
            (format!("La justicia y la venganza se han mezclado tanto que {} ya no distingue una de otra.", nombre), "cuando tiene el poder de decidir quién vive y quién muere".to_string()),
            (format!("Bajo la dureza del fronterizo, {} tiene un corazón que sangra por la familia que eligió no tener.", nombre), "cuando ve lo que otros han construido".to_string()),
        ],
        _ => vec![
            (format!("{} niega una parte de sí mismo que emergerá cuando menos lo espere.", nombre), "en momentos de estrés extremo".to_string()),
            (format!("Hay algo oscuro en {} que solo sale cuando nadie está mirando.", nombre), "cuando el control se desliza".to_string()),
            (format!("La persona que {} muestra al mundo no es quien realmente es. Vive en constante performance.", nombre), "cuando la máscara amenaza con caer".to_string()),
        ]
    };
    
    sombras.choose(rng).unwrap().clone()
}

