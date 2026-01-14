use super::Language;
use rand::seq::SliceRandom;

pub struct BancoNarrativo;

impl BancoNarrativo {
    /// Obtiene una plantilla de texto basada en clave, idioma y una semilla de aleatoriedad (opcional)
    /// Parámetros de reemplazo: {0} = Nombre, {1} = Lugar/Causa, {2} = Extra
    pub fn obtener(key: &str, lang: &Language, params: &[&str], rng: &mut impl rand::Rng) -> String {
        let options = match lang {
            Language::English => Self::get_en(key),
            Language::Japanese => Self::get_jp(key),
            _ => Self::get_es(key),
        };

        if options.is_empty() {
            return format!("[MISSING: {}]", key);
        }

        let template = options.choose(rng).unwrap_or(&options[0]);
        
        let mut result = template.to_string();
        for (i, param) in params.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, param);
        }
        result
    }

    fn get_es(key: &str) -> Vec<&'static str> {
        match key {
            // ─── ORIGEN ───
            "origen_radiante" => vec![
                "{0} llegó al mundo como una promesa. Hubo manos cálidas esperando, voces que ya le amaban antes de conocerle.",
                "Los primeros días de {0} fueron de luz. No perfectos, pero llenos de esa esperanza torpe y sincera que rodea a toda vida nueva.",
            ],
            "origen_calido" => vec![
                "{0} nació en {1}. No era perfecto, pero era suyo. Había suficiente amor para empezar.",
                "El mundo que recibió a {0} tenía grietas, sí, pero también rendijas por donde entraba la luz.",
            ],
            "origen_balanceado" => vec![
                "{0} llegó sin fanfarria ni tragedia. Un comienzo ordinario en {1}, un lienzo en blanco esperando pinceladas.",
                "Los orígenes de {0} fueron tranquilos. Una familia, un lugar, un momento. La historia estaba por escribirse.",
            ],
            "origen_melancolico" => vec![
                "{0} llegó en un momento difícil. Las circunstancias en {1} no eran ideales, aunque hubo quien lo intentó.",
                "Había sombras en la casa donde nació {0}. No monstruos, solo esas sombras cotidianas de dudas y miedos.",
            ],
            "origen_sombrio" => vec![
                "{0} llegó al mundo presagiando tormenta. Creció en {1}, un lugar que marcaría su carácter con fuego.",
                "El comienzo de {0} fue marcado por ausencias. No todas las llegadas son celebradas; algunas son solo soportadas.",
            ],

            // ─── INFANCIA ───
            "infancia_radiante" => vec![
                "Hubo veranos eternos. {0} corría, reía, existía con esa intensidad pura que solo los niños conocen.",
                "En algún rincón de la infancia de {0} hay un tesoro intacto: un momento de felicidad absoluta.",
            ],
            "infancia_calido" => vec![
                "La infancia tuvo sus rituales de consuelo: {1}, un espacio donde las reglas del mundo adulto no aplicaban.",
                "{0} encontró refugio en {1}. No era un palacio, pero era un hogar para su imaginación.",
            ],
            "infancia_balanceado" => vec![
                "Los años pasaron como pasan para todos: días buenos, días malos. {0} creció entre lecciones aprendidas e ignoradas.",
                "No fue una infancia de cuento. Fue real. Con raspones en las rodillas y pequeñas victorias cotidianas.",
            ],
            "infancia_melancolico" => vec![
                "{0} aprendió temprano a leer el ambiente. Desarrolló antenas para lo no dicho, para el silencio antes de la tormenta.",
                "A veces {0} se sentía diferente. Como si hubiera un cristal invisible separándolo del resto del mundo.",
            ],
            "infancia_sombrio" => vec![
                "La infancia de {0} tuvo zonas prohibidas. Habitaciones donde no se entraba, verdades que se fingía no ver.",
                "Hubo momentos oscuros. {0} no habla de ellos, pero lleva el mapa de esos peligros grabado en la piel.",
            ],

            // ─── CATALIZADOR (QUIEBRE) ───
            "catalizador_intro" => vec![
                "Y entonces llegó el quiebre.",
                "Todo cambió en un instante.",
                "El punto de no retorno llegó sin aviso.",
            ],
            "catalizador_sombrio" => vec![
                "{0}. No hay forma de adornarlo: dolió. El antes y el después quedaron divididos por fuego.",
                "{0}. {1} sintió que el suelo desaparecía bajo sus pies. Solo quedó el impacto.",
            ],
            
            // ─── TRANSFORMACIÓN ───
            "transformacion_radiante" => vec![
                "De aquello emergió algo inesperado: fortaleza. {0} descubrió que podía ser autor de su vida, no solo personaje.",
                "Las cicatrices empezaron a contar una historia de supervivencia. {0} encontró un nuevo propósito.",
            ],
            "transformacion_melancolico" => vec![
                "La transformación tuvo precio. Para protegerse, {0} construyó armaduras que a veces pesan demasiado.",
                "{0} cambió, sí. Pero sanar no es línea recta, es un espiral. A veces el pasado muerde de nuevo.",
            ],

            // ─── CONFLICTOS ───
            "conflicto_deber_deseo" => vec![
                "{0} vive en la tensión entre cumplir y querer. Cada día es una negociación.",
                "Para {0}, el deber es un ancla; el deseo, una marea peligrosa.",
            ],
            "conflicto_seguridad_crecimiento" => vec![
                "{0} conoce los límites de su zona segura. A veces la jaula es dorada, pero sigue siendo jaula.",
                "Crecer duele. Quedarse quieto asfixia. {0} debe elegir qué dolor prefiere.",
                "El horizonte llama a {0}, pero el suelo conocido sujeta sus pies con fuerza.",
            ],

            _ => vec![],
        }
    }

    fn get_en(key: &str) -> Vec<&'static str> {
        match key {
            // ─── ORIGIN ───
            "origen_radiante" => vec![
                "{0} came into the world like a promise. Warm hands were waiting, voices that loved {0} even before meeting them.",
                "{0}'s first days were full of light. Not perfect, but filled with that clumsy, sincere hope of new life.",
            ],
            "origen_calido" => vec![
                "{0} was born in {1}. It wasn't perfect, but it was theirs. There was enough love to start.",
                "The world that welcomed {0} had cracks, yes, but also gaps where the light got in.",
            ],
            "origen_balanceado" => vec![
                "{0} arrived without fanfare or tragedy. An ordinary beginning in {1}, a blank canvas waiting for the first strokes.",
                "{0}'s origins were quiet. A family, a place, a moment. The story was yet to be written.",
            ],
            "origen_melancolico" => vec![
                "{0} arrived at a difficult time. Circumstances in {1} were not ideal, though some tried their best.",
                "There were shadows in the house where {0} was born. Not monsters, just the daily shadows of doubt and fear.",
            ],
            "origen_sombrio" => vec![
                "{0} came into the world foreshadowing a storm. Raised in {1}, a place that would mark their character with fire.",
                "{0}'s beginning was marked by absence. Not all arrivals are celebrated; some are merely endured.",
            ],

            // ─── CHILDHOOD ───
            "infancia_radiante" => vec![
                "There were eternal summers. {0} ran, laughed, existed with that pure intensity only children know.",
                "In some corner of {0}'s childhood lies an intact treasure: a moment of absolute happiness.",
            ],
            "infancia_calido" => vec![
                "Childhood had its rituals of comfort: {1}, a space where adult rules didn't apply.",
                "{0} found refuge in {1}. Not a palace, but a home for their imagination.",
            ],
            "infancia_balanceado" => vec![
                "The years passed as they do for everyone: good days, bad days. {0} grew between lessons learned and ignored.",
                "It wasn't a fairy tale childhood. It was real. With scraped knees and small daily victories.",
            ],
            "infancia_melancolico" => vec![
                "{0} learned early to read the room. Developed antennas for the unspoken, for the silence before the storm.",
                "Sometimes {0} felt different. As if there were an invisible glass wall separating them from the rest of the world.",
            ],
            "infancia_sombrio" => vec![
                "{0}'s childhood had forbidden zones. Rooms not entered, truths everyone pretended not to see.",
                "There were dark moments. {0} doesn't speak of them, but carries the map of those dangers etched in skin.",
            ],
            
            // ─── CATALYST (BREAK) ───
            "catalizador_intro" => vec![
                "And then the breaking point arrived.",
                "Everything changed in an instant.",
                "The point of no return came without warning.",
            ],
            "catalizador_sombrio" => vec![
                "{0}. There is no way to sugarcoat it: it hurt. The before and after were divided by fire.",
                "{0}. {1} felt the ground disappear beneath their feet. Only the impact remained.",
            ],

            // ─── TRANSFORMATION ───
            "transformacion_radiante" => vec![
                "From that emerged something unexpected: strength. {0} discovered they could be the author of their life, not just a character.",
                "The scars began to tell a story of survival. {0} found a new purpose.",
            ],
            "transformacion_melancolico" => vec![
                "Transformation came at a price. To protect themselves, {0} built armor that sometimes weighs too much.",
                "{0} changed, yes. But healing is not a straight line, it's a spiral. Sometimes the past bites back.",
            ],

            // ─── CONFLICTS ───
            "conflicto_deber_deseo" => vec![
                "{0} lives in the tension between duty and desire. Every day is a negotiation.",
                "For {0}, duty is a heavy anchor; desire, a dangerous tide.",
            ],
            "conflicto_seguridad_crecimiento" => vec![
                "{0} knows the limits of their safe zone. Sometimes the cage is golden, but it remains a cage.",
                "Growing hurts. Staying still suffocates. {0} must choose which pain to endure.",
                "The horizon calls to {0}, but the known ground holds their feet tightly.",
            ],

            _ => vec![], 
        }
    }

    fn get_jp(key: &str) -> Vec<&'static str> {
        match key {
            "origen_radiante" => vec![
                "{0}は約束のようにこの世に現れた。温かい手が待っていた。",
                "{0}の最初の日々は光に満ちていた。",
            ],
            // ... (Se podrían llenar más, por ahora fallback a inglés si está vacío es mejor, pero aquí retornaré lista vacía para que el sistema use ES o EN fallback manual si se implementara)
            // Implementaré una básica para demostrar
             "catalizador_intro" => vec![
                "そして、崩壊が訪れた。",
                "その瞬間、全てが変わった。",
            ],
            _ => Self::get_en(key), // Fallback a inglés para JP por ahora
        }
    }
}
