use super::Language;
use rand::seq::SliceRandom;

pub struct BancoNarrativo;

impl BancoNarrativo {
    /// Obtiene una plantilla de texto basada en clave, idioma y una semilla de aleatoriedad
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
            // ═══════════════════════════════════════════════════════════════
            // ORIGEN - Párrafos completos (3-4 oraciones)
            // ═══════════════════════════════════════════════════════════════
            "origen_radiante" => vec![
                "{0} llegó al mundo como una promesa. Hubo manos cálidas esperando, voces que ya le amaban antes de conocerle. No todo era perfecto — nada lo es — pero había suficiente luz para que los primeros recuerdos guarden el eco de la risa. El amor, aunque imperfecto, era real.",
                "Los primeros días de {0} fueron de luz genuina. Una familia que lo celebraba, un hogar que aunque pequeño, se sentía infinito para un niño. Los vecinos hablaban de buenas señales. Y aunque después vendrían tormentas, aquel comienzo dejó semillas de esperanza que germinarían en los momentos más oscuros.",
            ],
            "origen_calido" => vec![
                "{0} nació en {1}. No era perfecto, pero era suyo. Había suficiente amor para empezar, aunque a veces se manifestara de formas torpas o insuficientes. Los primeros años tuvieron el ritmo de cualquier infancia: pequeñas alegrías, miedos nocturnos, el descubrimiento gradual de que el mundo era más grande y más complejo de lo que parecía desde la ventana del cuarto.",
                "El mundo que recibió a {0} tenía grietas, sí, pero también rendijas por donde entraba la luz. Hubo momentos de genuina conexión entre las dificultades. Alguien intentó, a su manera, darle lo que podía. Y eso, aunque nunca fue suficiente del todo, plantó las bases de una resiliencia que más tarde definiría su carácter.",
            ],
            "origen_balanceado" => vec![
                "{0} llegó sin fanfarria ni tragedia, un comienzo ordinario en {1}. Un lienzo en blanco esperando pinceladas. Los días pasaron con la monotonía predecible de cualquier infancia normal — rituales familiares, juegos con otros niños, las pequeñas victorias y derrotas que forman el carácter sin que uno lo note. Nada presagiaba lo que vendría después.",
                "Los orígenes de {0} fueron tranquilos. Una familia, un lugar, un momento en el tiempo. La historia estaba por escribirse. No hubo señales de grandeza ni de tragedia, solo el transcurrir silencioso de los años formativos. Quizás esa normalidad sea lo que más añora ahora, cuando mira hacia atrás.",
            ],
            "origen_melancolico" => vec![
                "{0} llegó en un momento difícil. Las circunstancias en {1} no eran ideales — los adultos llevaban preocupaciones que trataban de ocultar, pero los niños siempre perciben más de lo que deberían. Hubo quien lo intentó, quien puso lo mejor de sí. Pero el peso de aquellos tiempos se filtró en los huesos de {0}, enseñándole muy temprano que el mundo no siempre es gentil con quienes buscan solo existir.",
                "Había sombras en la casa donde nació {0}. No monstruos, solo esas sombras cotidianas de dudas y miedos que los adultos proyectan sin querer. Aprendió a caminar evitando los charcos emocionales, a leer el ambiente antes de hablar, a hacerse pequeño cuando el aire se tensaba. Estas habilidades de supervivencia le servirían después, aunque el costo de aprenderlas tan joven nunca termina de pagarse.",
            ],
            "origen_sombrio" => vec![
                "{0} llegó al mundo presagiando tormenta. Creció en {1}, un lugar que marcaría su carácter con fuego y hielo. No todos los comienzos merecen celebrarse — algunos se soportan, se sobreviven, se entierra lo mejor que se puede en el rincón más profundo de la memoria. Los primeros años dejaron cicatrices invisibles que todavía duelen cuando cambia el tiempo emocional.",
                "El comienzo de {0} fue marcado por ausencias. Las que se notan y las que no. Un hogar que era todo menos eso, adultos demasiado consumidos por sus propias batallas para notar al niño que crecía entre sus escombros. {0} aprendió pronto que la esperanza es un lujo que no todos pueden permitirse, y que la supervivencia a veces significa dejar de sentir.",
            ],

            // ═══════════════════════════════════════════════════════════════
            // INFANCIA - Párrafos más extensos
            // ═══════════════════════════════════════════════════════════════
            "infancia_radiante" => vec![
                "Hubo veranos eternos. {0} corría, reía, existía con esa intensidad pura que solo los niños conocen. Había un grupo de amigos — los nombres se han desdibujado, pero la sensación permanece. Aventuras inventadas, reinos de cartón, promesas solemnes que se olvidaban al día siguiente. En algún rincón de esa época hay un tesoro intacto: un momento de felicidad absoluta que todavía puede invocar cuando la oscuridad aprieta.",
                "La infancia de {0} fue un regalo que entonces no supo valorar. Figuras protectoras que parecían invencibles, un mundo donde los problemas tenían solución antes de la hora de dormir, la certeza inocente de que así sería siempre. Claro, esa certeza se rompería después — todas lo hacen — pero los fragmentos de aquella luz aún brillan entre sus recuerdos cuando necesita recordar que la bondad existe.",
            ],
            "infancia_calido" => vec![
                "La infancia tuvo sus rituales de consuelo: {1}, un espacio donde las reglas del mundo adulto no aplicaban. {0} aprendió a construir refugios — físicos y mentales — donde podía ser simplemente un niño sin las complejidades que los mayores arrastraban. No todo era fácil, pero había suficientes momentos de luz para equilibrar las sombras. Una comida favorita, una canción que calmaba, un lugar secreto donde el mundo exterior no podía alcanzarlo.",
                "{0} encontró refugio en {1}. No era un palacio, pero era un hogar para su imaginación. Los días difíciles se equilibraban con pequeñas victorias: un cumplido inesperado, un abrazo cuando más lo necesitaba, la sensación de ser visto aunque fuera por breves momentos. Estos fragmentos de conexión serían los ladrillos con los que más tarde intentaría reconstruir lo que la vida derribó.",
            ],
            "infancia_balanceado" => vec![
                "Los años pasaron como pasan para todos: días buenos, días malos. {0} creció entre lecciones aprendidas e ignoradas, entre reglas que tenían sentido y otras que parecían absurdas. No fue una infancia de cuento — fue real. Con raspones en las rodillas, pequeñas victorias cotidianas y esas derrotas menores que duelen intensamente en el momento pero se olvidan después. Normal. Quizás lo más valioso sea eso: que fue simplemente normal.",
                "La infancia de {0} no fue excepcional en ningún sentido. Ni especialmente feliz ni particularmente trágica. Fue el lento acumularse de experiencias que forman una persona: amistades que duraron y otras que no, adultos que cumplieron y otros que fallaron, promesas del mundo que a veces se cumplían y otras revelaban ser humo. Una preparación inconsciente para lo que vendría después.",
            ],
            "infancia_melancolico" => vec![
                "{0} aprendió temprano a leer el ambiente. Desarrolló antenas para lo no dicho, para el silencio antes de la tormenta. Era un niño que observaba más de lo que hablaba, que sentía más de lo que expresaba. Descubrió que los adultos mentían constantemente — no siempre con maldad, pero mentían — y que la única persona en quien podía confiar completamente era él mismo. Este conocimiento prematuro fue una carga que no debería haber llevado tan joven.",
                "A veces {0} se sentía diferente. Como si hubiera un cristal invisible separándolo del resto del mundo. Veía a otros niños jugar con una ligereza que él no podía imitar, sus preocupaciones le parecían triviales mientras él cargaba con pesos que no tenían nombre. No era culpa de nadie exactamente, simplemente algunas almas vienen al mundo con una sensibilidad que las hace más vulnerables a las corrientes que otros ni sienten.",
            ],
            "infancia_sombrio" => vec![
                "La infancia de {0} tuvo zonas prohibidas. Habitaciones donde no se entraba, verdades que se fingía no ver, silencios que decían más que cualquier grito. Aprendió a caminar sin hacer ruido, a hacerse invisible cuando era necesario, a leer los signos de peligro en la tensión de una mandíbula o el tono de una voz. Habilidades de supervivencia que ningún niño debería necesitar, pero que le salvaron más de una vez.",
                "Hubo momentos oscuros. {0} no habla de ellos, pero lleva el mapa de esos peligros grabado en la piel. Cada cicatriz cuenta una historia que prefiere no contar, cada miedo irracional tiene una razón perfectamente racional en algún rincón de su pasado. La infancia debería ser un jardín; la suya fue más bien un campo minado que atravesó con heridas que aún sangran si se presionan.",
            ],

            // ═══════════════════════════════════════════════════════════════
            // CATALIZADOR - El momento del quiebre
            // ═══════════════════════════════════════════════════════════════
            "catalizador_intro" => vec![
                "Y entonces llegó el quiebre.",
                "Todo cambió en un instante. Un antes y un después tan definidos que parecen pertenecer a vidas diferentes.",
                "El punto de no retorno llegó sin aviso, como siempre hacen las tormentas que realmente importan.",
            ],
            "catalizador_sombrio" => vec![
                "{1}. No hay forma de adornarlo: dolió. El antes y el después quedaron divididos por fuego. Lo que {0} creía saber sobre el mundo, sobre la gente, sobre sí mismo, se derrumbó en cuestión de horas. Algunas cosas que se rompen pueden repararse; otras simplemente aprenden a funcionar rotas.",
                "{1}. {0} sintió que el suelo desaparecía bajo sus pies. Solo quedó el impacto y el largo trabajo de recomponer los fragmentos. Hay heridas que no cicatrizan, solo aprenden a doler de formas más manejables. Esta fue una de ellas: el tipo de dolor que cambia la arquitectura del alma.",
            ],
            
            // ═══════════════════════════════════════════════════════════════
            // TRANSFORMACIÓN - El renacer
            // ═══════════════════════════════════════════════════════════════
            "transformacion_radiante" => vec![
                "De aquello emergió algo inesperado: fortaleza. {0} descubrió que podía ser autor de su vida, no solo personaje. Las cicatrices empezaron a contar una historia de supervivencia, no de derrota. No fue un cambio instantáneo — nada real lo es — sino un lento reconocerse en el espejo con menos disgusto, una gradual reconciliación con las partes rotas que nunca terminarán de sanar pero han aprendido a coexistir.",
                "El fénix es un mito, pero la resiliencia es real. {0} encontró, en las cenizas de lo que fue, material para construir algo nuevo. No mejor necesariamente — solo diferente. Una versión de sí mismo que había pasado por el fuego y emergido con una claridad forjada en dolor. El propósito que encontró no borró el pasado, pero le dio un uso para el futuro.",
            ],
            "transformacion_melancolico" => vec![
                "La transformación tuvo precio. Para protegerse, {0} construyó armaduras que a veces pesan demasiado. Mecanismos de defensa que funcionaron entonces pero ahora interfieren con la vida que intenta construir. El trauma es así: te da herramientas para sobrevivir y después te cobra por usarlas. Cada adaptación tiene un costo que no se ve hasta mucho después.",
                "{0} cambió, sí. Pero sanar no es línea recta, es un espiral. A veces el pasado muerde de nuevo cuando menos lo espera. Las victorias sobre los propios demonios nunca son permanentes; son treguas que hay que renegociar cada día. La persona que emergió de aquello es más fuerte en algunos aspectos, más frágil en otros. Simplemente diferente.",
            ],

            // ═══════════════════════════════════════════════════════════════
            // CONFLICTOS INTERNOS
            // ═══════════════════════════════════════════════════════════════
            "conflicto_deber_deseo" => vec![
                "{0} vive en la tensión entre cumplir y querer. Cada día es una negociación silenciosa entre lo que debe a otros y lo que anhela para sí. Aprendió temprano que sus deseos venían después — quizás por eso ahora le cuesta saber qué quiere realmente cuando nadie está mirando.",
                "Para {0}, el deber es un ancla; el deseo, una marea peligrosa que intenta arrastrarlo hacia aguas desconocidas. Ha pasado demasiado tiempo siendo lo que otros necesitaban. Ahora que tiene la oportunidad de ser él mismo, descubre que no sabe exactamente quién es.",
            ],
            "conflicto_seguridad_crecimiento" => vec![
                "{0} conoce los límites de su zona segura. A veces la jaula es dorada, pero sigue siendo jaula. Crecer duele; quedarse quieto asfixia. Debe elegir qué dolor prefiere soportar hoy, sabiendo que mañana tendrá que elegir de nuevo.",
                "El horizonte llama a {0}, pero el suelo conocido sujeta sus pies con fuerza. Hay confort en lo familiar, aunque sea incómodo. Hay terror en lo desconocido, aunque prometa libertad. La elección nunca es tan simple como parece desde fuera.",
            ],

            _ => vec![],
        }
    }

    fn get_en(key: &str) -> Vec<&'static str> {
        match key {
            // ═══════════════════════════════════════════════════════════════
            // ORIGIN - Full paragraphs (3-4 sentences)
            // ═══════════════════════════════════════════════════════════════
            "origen_radiante" => vec![
                "{0} came into the world like a promise. Warm hands were waiting, voices that loved {0} even before meeting them. Not everything was perfect — nothing ever is — but there was enough light that the first memories still carry echoes of laughter. The love, though imperfect, was real.",
                "{0}'s first days were full of genuine light. A family that celebrated, a home that though small, felt infinite to a child. Neighbors spoke of good omens. And though storms would come later, that beginning planted seeds of hope that would bloom in the darkest moments.",
            ],
            "origen_calido" => vec![
                "{0} was born in {1}. It wasn't perfect, but it was theirs. There was enough love to start, even if it sometimes manifested in clumsy or insufficient ways. The first years had the rhythm of any childhood: small joys, nighttime fears, the gradual discovery that the world was larger and more complex than it seemed from the bedroom window.",
                "The world that welcomed {0} had cracks, yes, but also gaps where the light got in. There were moments of genuine connection between the difficulties. Someone tried, in their own way, to give what they could. And that, though never quite enough, laid the foundation for a resilience that would later define their character.",
            ],
            "origen_balanceado" => vec![
                "{0} arrived without fanfare or tragedy. An ordinary beginning in {1}, a blank canvas waiting for the first strokes. Days passed with the predictable monotony of any normal childhood — family rituals, games with other children, small victories and defeats that shape character without one noticing. Nothing foreshadowed what would come later.",
                "{0}'s origins were quiet. A family, a place, a moment in time. The story was yet to be written. There were no signs of greatness or tragedy, only the silent passing of formative years. Perhaps that normalcy is what they miss most now, looking back.",
            ],
            "origen_melancolico" => vec![
                "{0} arrived at a difficult time. Circumstances in {1} were not ideal — adults carried worries they tried to hide, but children always perceive more than they should. Someone tried, put their best foot forward. But the weight of those times seeped into {0}'s bones, teaching very early that the world is not always gentle with those who simply want to exist.",
                "There were shadows in the house where {0} was born. Not monsters, just the daily shadows of doubt and fear that adults project without meaning to. They learned to walk around emotional puddles, to read the room before speaking, to make themselves small when the air tensed. These survival skills would serve them later, though the cost of learning them so young never stops paying.",
            ],
            "origen_sombrio" => vec![
                "{0} came into the world foreshadowing a storm. Raised in {1}, a place that would mark their character with fire and ice. Not all beginnings deserve celebration — some are endured, survived, buried as best as possible in the deepest corner of memory. The early years left invisible scars that still ache when the emotional weather changes.",
                "{0}'s beginning was marked by absences. The ones that are noticed and those that aren't. A home that was anything but, adults too consumed by their own battles to notice the child growing among their ruins. {0} learned early that hope is a luxury not everyone can afford, and that survival sometimes means stopping to feel.",
            ],

            // ═══════════════════════════════════════════════════════════════
            // CHILDHOOD
            // ═══════════════════════════════════════════════════════════════
            "infancia_radiante" => vec![
                "There were eternal summers. {0} ran, laughed, existed with that pure intensity only children know. There was a group of friends — names have faded, but the feeling remains. Invented adventures, cardboard kingdoms, solemn promises forgotten by next day. Somewhere in that time lies an intact treasure: a moment of absolute happiness they can still invoke when darkness presses close.",
                "{0}'s childhood was a gift not appreciated then. Protective figures who seemed invincible, a world where problems had solutions before bedtime, the innocent certainty that it would always be so. Of course, that certainty would break later — they all do — but fragments of that light still shine among their memories when they need to remember that goodness exists.",
            ],
            "infancia_calido" => vec![
                "Childhood had its rituals of comfort: {1}, a space where adult rules didn't apply. {0} learned to build refuges — physical and mental — where they could simply be a child without the complexities adults dragged around. Not everything was easy, but there were enough moments of light to balance the shadows. A favorite meal, a calming song, a secret place where the outside world couldn't reach.",
                "{0} found refuge in {1}. Not a palace, but a home for their imagination. Difficult days were balanced with small victories: an unexpected compliment, a hug when most needed, the feeling of being seen even briefly. These fragments of connection would become the bricks with which they'd later try to rebuild what life tore down.",
            ],
            "infancia_balanceado" => vec![
                "The years passed as they do for everyone: good days, bad days. {0} grew between lessons learned and ignored, between rules that made sense and others that seemed absurd. It wasn't a fairy tale childhood — it was real. With scraped knees, small daily victories, and those minor defeats that hurt intensely in the moment but are forgotten later. Normal. Perhaps the most valuable thing is that: it was simply normal.",
                "{0}'s childhood wasn't exceptional in any sense. Neither especially happy nor particularly tragic. It was the slow accumulation of experiences that form a person: friendships that lasted and others that didn't, adults who delivered and others who failed, promises of the world that sometimes came true and others that proved to be smoke. An unconscious preparation for what was to come.",
            ],
            "infancia_melancolico" => vec![
                "{0} learned early to read the room. Developed antennas for the unspoken, for the silence before the storm. They were a child who observed more than spoke, who felt more than expressed. They discovered that adults lied constantly — not always with malice, but they lied — and that the only person they could completely trust was themselves. This premature knowledge was a burden that shouldn't have been carried so young.",
                "Sometimes {0} felt different. As if there were an invisible glass wall separating them from the rest of the world. They watched other children play with a lightness they couldn't imitate, their worries seemed trivial while they carried weights that had no name. It wasn't anyone's fault exactly, just that some souls come into the world with a sensitivity that makes them more vulnerable to currents others don't even feel.",
            ],
            "infancia_sombrio" => vec![
                "{0}'s childhood had forbidden zones. Rooms not entered, truths everyone pretended not to see, silences that said more than any shout. They learned to walk without making noise, to become invisible when necessary, to read signs of danger in the tension of a jaw or the tone of a voice. Survival skills no child should need, but that saved them more than once.",
                "There were dark moments. {0} doesn't speak of them, but carries the map of those dangers etched in skin. Each scar tells a story they prefer not to tell, each irrational fear has a perfectly rational reason somewhere in their past. Childhood should be a garden; theirs was more of a minefield they crossed with wounds that still bleed when pressed.",
            ],
            
            "catalizador_intro" => vec![
                "And then the breaking point arrived.",
                "Everything changed in an instant. A before and after so defined they seem to belong to different lives.",
                "The point of no return came without warning, as the storms that really matter always do.",
            ],
            "catalizador_sombrio" => vec![
                "{1}. There is no way to sugarcoat it: it hurt. The before and after were divided by fire. What {0} thought they knew about the world, about people, about themselves, collapsed in a matter of hours. Some broken things can be repaired; others simply learn to function broken.",
                "{1}. {0} felt the ground disappear beneath their feet. Only the impact remained and the long work of piecing together the fragments. Some wounds don't heal, they just learn to hurt in more manageable ways. This was one of them: the kind of pain that changes the architecture of the soul.",
            ],
            "transformacion_radiante" => vec![
                "From that emerged something unexpected: strength. {0} discovered they could be the author of their life, not just a character. The scars began to tell a story of survival, not defeat. It wasn't an instant change — nothing real is — but a slow recognizing themselves in the mirror with less disgust, a gradual reconciliation with broken parts that will never fully heal but have learned to coexist.",
                "The phoenix is a myth, but resilience is real. {0} found, in the ashes of what was, material to build something new. Not necessarily better — just different. A version of themselves that had passed through fire and emerged with a clarity forged in pain. The purpose they found didn't erase the past, but gave it a use for the future.",
            ],
            "transformacion_melancolico" => vec![
                "Transformation came at a price. To protect themselves, {0} built armor that sometimes weighs too much. Defense mechanisms that worked then but now interfere with the life they're trying to build. Trauma is like that: it gives you tools to survive and then charges you for using them. Every adaptation has a cost that isn't seen until much later.",
                "{0} changed, yes. But healing is not a straight line, it's a spiral. Sometimes the past bites back when least expected. Victories over one's own demons are never permanent; they're truces that must be renegotiated every day. The person who emerged from that is stronger in some ways, more fragile in others. Simply different.",
            ],
            "conflicto_deber_deseo" => vec![
                "{0} lives in the tension between duty and desire. Every day is a silent negotiation between what they owe others and what they long for themselves. They learned early that their desires came second — perhaps that's why now they struggle to know what they really want when no one is watching.",
                "For {0}, duty is an anchor; desire, a dangerous tide that tries to drag them toward unknown waters. They've spent too long being what others needed. Now that they have the chance to be themselves, they discover they don't know exactly who that is.",
            ],
            "conflicto_seguridad_crecimiento" => vec![
                "{0} knows the limits of their safe zone. Sometimes the cage is golden, but it remains a cage. Growing hurts; staying still suffocates. They must choose which pain to endure today, knowing tomorrow they'll have to choose again.",
                "The horizon calls to {0}, but the known ground holds their feet tightly. There's comfort in the familiar, even if uncomfortable. There's terror in the unknown, even if it promises freedom. The choice is never as simple as it looks from outside.",
            ],

            _ => vec![], 
        }
    }

    fn get_jp(key: &str) -> Vec<&'static str> {
        match key {
            "origen_radiante" => vec![
                "{0}は約束のように生まれてきた。温かい手が待っていた。完璧ではなかったが、笑い声のこだまを残すには十分な光があった。不完全でも、愛は本物だった。",
                "{0}の最初の日々は光に満ちていた。祝福する家族、小さくても子供には無限に感じる家。近所の人々は良い兆しを語った。後に嵐が来るとしても、その始まりは最も暗い瞬間に芽吹く希望の種を蒔いた。",
            ],
            "catalizador_intro" => vec![
                "そして、崩壊の時が訪れた。",
                "その瞬間、全てが変わった。まるで別の人生のような明確な境界線。",
                "転機は警告なく訪れた。本当に重要な嵐はいつもそうだ。",
            ],
            "catalizador_sombrio" => vec![
                "{1}。飾る言葉はない。痛かった。以前と以後は炎で分断された。{0}が世界について、人々について、自分自身について信じていたものは、数時間で崩れ落ちた。",
                "{1}。{0}は足元の地面が消えるのを感じた。衝撃だけが残り、破片を拾い集める長い作業が始まった。癒えない傷もある。ただ、より扱いやすい痛みを学ぶだけだ。",
            ],
            "transformacion_radiante" => vec![
                "そこから予想外のものが生まれた：強さ。{0}は自分の人生の著者になれることを発見した。傷跡は敗北ではなく、生存の物語を語り始めた。一瞬の変化ではなかった。鏡を見ても嫌悪が減り、決して完全には癒えないが共存を学んだ壊れた部分との和解が徐々に進んだ。",
            ],
            _ => Self::get_en(key), // Fallback to English for JP
        }
    }
}
