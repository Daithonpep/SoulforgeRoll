// ═══════════════════════════════════════════════════════════════════════════════
// GENERADOR PROCEDURAL DE TEXTO - Sistema de "LLM Falso"
// ═══════════════════════════════════════════════════════════════════════════════
// 
// Este sistema genera texto que parece escrito por IA pero es 100% procedural.
// Usa plantillas con slots que se llenan con fragmentos combinables.
// 
// Ventajas:
//   ✓ Sin dependencia de LLM externo (Ollama, GPT)
//   ✓ Determinista (misma semilla = mismo resultado)
//   ✓ Miles de variaciones posibles
//   ✓ Consume casi cero recursos
// ═══════════════════════════════════════════════════════════════════════════════

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Generador procedural de texto literario
pub struct ProceduralTextGenerator {
    rng: ChaCha8Rng,
}

impl ProceduralTextGenerator {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE HERIDAS EMOCIONALES
    /// ═══════════════════════════════════════════════════════════════════════════
    
    /// Genera una descripción de herida emocional en español
    pub fn generar_herida_es(&mut self) -> String {
        let sujetos = ["Nunca pudo", "Jamás logró", "No consiguió", "Fue incapaz de"];
        let acciones = ["perdonarse", "olvidar", "superar", "dejar ir", "aceptar"];
        let objetos = ["lo que hizo", "aquello", "el pasado", "la traición", "su error"];
        
        let sujeto = sujetos.choose(&mut self.rng).unwrap();
        let accion = acciones.choose(&mut self.rng).unwrap();
        let objeto = objetos.choose(&mut self.rng).unwrap();
        
        format!("{} {} {}", sujeto, accion, objeto)
    }
    
    /// Genera una descripción de herida emocional en inglés (natural y literaria)
    pub fn generar_herida_en(&mut self) -> String {
        // Frases completas que suenan naturales, no mecánicas
        let heridas = [
            "A wound that time refuses to heal",
            "Guilt clings to them like a second shadow",
            "The echoes of loss never truly fade",
            "Some scars run deeper than skin",
            "Haunted by choices that cannot be undone",
            "Carries a weight no one else can see",
            "The past is a ghost that won't rest",
            "Trust shattered beyond repair",
            "A betrayal that rewrote their soul",
            "Forever marked by what was taken",
            "The memory burns like an open flame",
            "Broken in ways words cannot capture",
        ];
        
        heridas.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// Genera una descripción de herida emocional en japonés (corto y natural)
    pub fn generar_herida_jp(&mut self) -> String {
        let patrones = [
            "許せない",
            "忘れられない",
            "消えない傷",
            "癒えぬ痛み",
            "過去に囚われる",
            "心の傷",
            "拭えない記憶",
            "背負う罪",
        ];
        
        patrones.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE MÁSCARAS SOCIALES
    /// ═══════════════════════════════════════════════════════════════════════════
    
    pub fn generar_mascara_es(&mut self) -> String {
        let adjetivos = ["Confiado", "Frío", "Alegre", "Servicial", "Cínico", "Distante"];
        let conectores = ["y", "pero"];
        let adjetivos2 = ["carismático", "calculador", "despreocupado", "abnegado", "sarcástico", "observador"];
        
        let adj1 = adjetivos.choose(&mut self.rng).unwrap();
        let conector = conectores.choose(&mut self.rng).unwrap();
        let adj2 = adjetivos2.choose(&mut self.rng).unwrap();
        
        format!("{} {} {}", adj1, conector, adj2)
    }
    
    pub fn generar_mascara_en(&mut self) -> String {
        // Máscaras como descripciones naturales de persona
        let mascaras = [
            "Projects confidence to hide inner doubt",
            "Wears a smile that never reaches their eyes",
            "The helpful friend everyone relies on",
            "Cool and detached, always in control",
            "Charm personified, hiding darker depths",
            "The cynic who secretly hopes to be proven wrong",
            "Quiet observer, rarely revealing true thoughts",
            "Life of the party, lonely in the crowd",
            "The reliable rock others lean on",
            "Sharp wit masking vulnerability",
            "Unshakeable calm that conceals inner storms",
            "The peacekeeper avoiding their own conflicts",
        ];
        
        mascaras.choose(&mut self.rng).unwrap().to_string()
    }
    
    pub fn generar_mascara_jp(&mut self) -> String {
        let mascaras = [
            "自信家",
            "冷徹",
            "陽気",
            "献身的",
            "皮肉屋",
            "観察者",
            "完璧主義",
            "無関心",
        ];
        
        mascaras.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE FRASES TÍPICAS
    /// ═══════════════════════════════════════════════════════════════════════════
    
    pub fn generar_frase_tipica_es(&mut self) -> String {
        let patrones = [
            ("No hay", vec!["problema", "situación", "crisis"], vec!["sin solución", "que no pueda manejar", "que me preocupe"]),
            ("Todo", vec!["estará", "saldrá", "terminará"], vec!["bien", "como debe ser", "en su lugar"]),
            ("¿Por qué", vec!["preocuparse", "complicarse", "pensar tanto"], vec!["?", " por eso?", " ahora?"]),
        ];
        
        let patron = patrones.choose(&mut self.rng).unwrap();
        let medio = patron.1.choose(&mut self.rng).unwrap();
        let final_parte = patron.2.choose(&mut self.rng).unwrap();
        
        format!("{} {} {}", patron.0, medio, final_parte)
    }
    
    pub fn generar_frase_tipica_en(&mut self) -> String {
        // Frases que un personaje diría naturalmente
        let frases = [
            "I've handled worse.",
            "Trust me, I know what I'm doing.",
            "No need to worry about that.",
            "Everything works out in the end.",
            "We'll cross that bridge when we get there.",
            "Is that supposed to scare me?",
            "I've got this under control.",
            "Don't mistake kindness for weakness.",
            "People see what they want to see.",
            "The truth has a way of surfacing.",
            "Some things are better left unsaid.",
            "Actions speak louder than words.",
        ];
        
        frases.choose(&mut self.rng).unwrap().to_string()
    }
    
    pub fn generar_frase_tipica_jp(&mut self) -> String {
        let frases = [
            "問題ない",
            "大丈夫だ",
            "なぜ心配する？",
            "どうでもいい",
            "やっぱりな",
            "そうなると思った",
            "仕方ない",
            "当然だ",
        ];
        
        frases.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE BIOGRAFÍAS
    /// ═══════════════════════════════════════════════════════════════════════════
    
    pub fn generar_biografia_fragmento_es(&mut self) -> String {
        let inicios = ["Nació en", "Creció en", "Pasó su infancia en", "Sus primeros años fueron en"];
        let lugares = ["un lugar olvidado", "las sombras", "medio del caos", "la pobreza", "un hogar roto"];
        let conectores = ["donde", "en el que", "allí"];
        let eventos = ["aprendió a sobrevivir", "perdió la inocencia", "forjó su carácter", "conoció el dolor"];
        
        let inicio = inicios.choose(&mut self.rng).unwrap();
        let lugar = lugares.choose(&mut self.rng).unwrap();
        let conector = conectores.choose(&mut self.rng).unwrap();
        let evento = eventos.choose(&mut self.rng).unwrap();
        
        format!("{} {} {} {}", inicio, lugar, conector, evento)
    }
    
    pub fn generar_biografia_fragmento_en(&mut self) -> String {
        // Fragmentos biográficos que suenan como de una novela
        let fragmentos = [
            "Childhood was a lesson in survival",
            "The streets taught what schools couldn't",
            "Raised by circumstance more than by family",
            "Early years forged in hardship and resilience",
            "Grew up learning to read people, not books",
            "The past is a country they rarely visit",
            "Home was wherever felt safe enough to sleep",
            "Youth spent chasing something always out of reach",
            "Learned early that trust is earned, not given",
            "The formative years left marks that don't show",
            "Came from nothing, determined to become something",
            "A childhood that ended too soon",
        ];
        
        fragmentos.choose(&mut self.rng).unwrap().to_string()
    }
    
    pub fn generar_biografia_fragmento_jp(&mut self) -> String {
        let fragmentos = [
            "忘れられた場所で生まれた",
            "影の中で育った",
            "混沌の中で過ごした",
            "貧困を知った",
            "壊れた家庭で育った",
            "生き延びることを学んだ",
            "純真を失った",
            "痛みを知った",
        ];
        
        fragmentos.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE NECESIDADES Y DESEOS
    /// ═══════════════════════════════════════════════════════════════════════════
    
    pub fn generar_necesidad_jp(&mut self) -> String {
        let necesidades = [
            "支配を手放すこと",         // Soltar el control
            "自分を受け入れること",     // Aceptarse a sí mismo
            "過去との和解",             // Reconciliación con el pasado
            "他者を信頼すること",       // Confiar en otros
            "弱さを見せる勇気",         // Coraje de mostrar debilidad
            "真実と向き合うこと",       // Enfrentar la verdad
            "愛されることを許す",       // Permitir ser amado
            "完璧でなくていいと知る",   // Saber que no hace falta ser perfecto
        ];
        necesidades.choose(&mut self.rng).unwrap().to_string()
    }

    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE ARCOS NARRATIVOS
    /// ═══════════════════════════════════════════════════════════════════════════

    pub fn generar_arco_inicio_jp(&mut self) -> String {
        let inicios = [
            "可能性を知らぬ者",
            "迷いの中にいる",
            "偽りの平穏",
            "変化を恐れる心",
            "孤立した魂",
        ];
        inicios.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn generar_arco_quiebre_jp(&mut self) -> String {
        let quiebres = [
            "運命の選択",           // Elección del destino
            "避けられぬ対立",       // Conflicto inevitable
            "全てを失う瞬間",       // Momento de perderlo todo
            "真実の啓示",           // Revelación de la verdad
            "後戻りできない道",     // Camino sin retorno
        ];
        quiebres.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn generar_arco_resolucion_jp(&mut self) -> String {
        let resoluciones = [
            "新たな自己の確立",     // Estableciendo un nuevo ser
            "過去からの解放",       // Liberación del pasado
            "傷と共にある平和",     // Paz con las cicatrices
            "真の強さの覚醒",       // Despertar de la verdadera fuerza
            "旅の終わり、新たな始まり", // Fin del viaje, nuevo comienzo
        ];
        resoluciones.choose(&mut self.rng).unwrap().to_string()
    }

    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE GANCHOS DE HISTORIA
    /// ═══════════════════════════════════════════════════════════════════════════

    pub fn generar_gancho_jp(&mut self) -> String {
        let ganchos = [
            "予期せぬ依頼",             // Encargo inesperado
            "過去からの訪問者",         // Visitante del pasado
            "隠された遺産",             // Herencia oculta
            "禁じられた知識",           // Conocimiento prohibido
            "裏切りの予兆",             // Presagio de traición
            "失われたものを求めて",     // Buscando lo perdido
        ];
        ganchos.choose(&mut self.rng).unwrap().to_string()
    }

    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE NECESIDADES Y DESEOS (ENGLISH)
    /// ═══════════════════════════════════════════════════════════════════════════
    
    pub fn generar_necesidad_en(&mut self) -> String {
        let necesidades = [
            // Necesidades profundas escritas como una persona
            "To finally let go of the need to control everything",
            "To accept that they are worthy of love",
            "To make peace with who they once were",
            "To learn that asking for help is not weakness",
            "To forgive themselves for past mistakes",
            "To trust that not everyone will leave",
            "To find worth beyond their achievements",
            "To embrace imperfection as part of being human",
            "To stop running from their own reflection",
            "To believe they deserve a second chance",
            "To understand that vulnerability is strength",
            "To realize they don't have to carry this burden alone",
        ];
        necesidades.choose(&mut self.rng).unwrap().to_string()
    }

    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE ARCOS NARRATIVOS (ENGLISH)
    /// ═══════════════════════════════════════════════════════════════════════════

    pub fn generar_arco_inicio_en(&mut self) -> String {
        let inicios = [
            "They haven't yet discovered what they're capable of",
            "Living behind walls they built long ago",
            "Going through the motions, but not truly living",
            "Trapped in patterns that feel impossible to break",
            "Searching for meaning in all the wrong places",
            "Holding onto a version of themselves that no longer exists",
            "Afraid to take the first step toward change",
            "Convinced that this is as good as it gets",
        ];
        inicios.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn generar_arco_quiebre_en(&mut self) -> String {
        let quiebres = [
            "The world forces them to finally confront the truth",
            "A moment that changes everything they thought they knew",
            "When the cost of staying the same becomes too high",
            "A crisis that strips away every pretense",
            "The point where there's no going back",
            "Everything they've built comes crashing down",
            "A betrayal that forces them to question everything",
            "The moment when running is no longer an option",
        ];
        quiebres.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn generar_arco_resolucion_en(&mut self) -> String {
        let resoluciones = [
            "They emerge transformed, scarred but stronger",
            "Finally at peace with who they've become",
            "The past no longer holds power over them",
            "They find strength they never knew they had",
            "A new chapter begins, written on their own terms",
            "They learn to carry the weight without being crushed",
            "Broken pieces reassembled into something new",
            "The journey continues, but they're no longer alone",
        ];
        resoluciones.choose(&mut self.rng).unwrap().to_string()
    }

    /// ═══════════════════════════════════════════════════════════════════════════
    /// GENERACIÓN DE GANCHOS DE HISTORIA (ENGLISH)
    /// ═══════════════════════════════════════════════════════════════════════════

    pub fn generar_gancho_en(&mut self) -> String {
        let ganchos = [
            // Ganchos de historia que suenan como sinopsis de novela
            "A letter arrives bearing news that changes everything",
            "Someone from the past reappears with unfinished business",
            "A dying confession reveals a long-buried secret",
            "An old debt comes due at the worst possible time",
            "A stranger knows things they shouldn't possibly know",
            "The discovery of a hidden room, and what it contains",
            "A chance encounter that proves to be anything but chance",
            "Whispers of a conspiracy that reaches higher than imagined",
            "A mysterious package with no return address",
            "The realization that they've been watched all along",
        ];
        ganchos.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// FALLBACK UNIVERSAL (La red de seguridad) - INGLÉS
    /// ═══════════════════════════════════════════════════════════════════════════
    
    /// Genera una frase abstracta/profunda en inglés cuando falla la traducción específica.
    pub fn generar_fallback_en(&mut self) -> String {
        let abstracciones = [
            // Frases evocadoras y literarias para cuando todo lo demás falla
            "Fate has a way of finding those who try to hide",
            "The soul holds secrets even the mind cannot reach",
            "Some truths are felt before they are known",
            "A fire burns that no darkness can extinguish",
            "Walking the line between shadow and light",
            "Carrying a purpose that has yet to reveal itself",
            "Time will tell what words cannot",
            "A promise that echoes through the years",
            "The road ahead is uncertain, but the will is not",
            "What lies beneath the surface runs deeper than expected",
            "Eyes that have seen too much, and forgotten nothing",
            "A story still being written, one choice at a time",
            "Strength forged in fires that never fully died",
            "The silence speaks volumes to those who listen",
        ];
        abstracciones.choose(&mut self.rng).unwrap().to_string()
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// FALLBACK UNIVERSAL (La red de seguridad) - JAPONÉS
    /// ═══════════════════════════════════════════════════════════════════════════
    
    /// Genera una frase abstracta/profunda en japonés cuando falla la traducción específica.
    /// Esto evita mostrar español.
    pub fn generar_fallback_jp(&mut self) -> String {
        let abstracciones = [
            "運命は不可解だ",           // El destino es inescrutable
            "魂の深淵",                 // El abismo del alma
            "言葉にできない想い",       // Sentimientos indescriptibles
            "静かなる決意",             // Determinación silenciosa
            "影と光の間で",             // Entre la sombra y la luz
            "秘められた目的",           // Propósito oculto
            "時が答えを出すだろう",     // El tiempo dará la respuesta
            "心に刻まれた誓い",         // Juramento grabado en el corazón
        ];
        abstracciones.choose(&mut self.rng).unwrap().to_string()
    }
    /// ═══════════════════════════════════════════════════════════════════════════
    /// ADAPTACIÓN INTELIGENTE DE TEXTO ACTUALIZADA
    /// ═══════════════════════════════════════════════════════════════════════════
    
    /// Intenta adaptar texto español a otro idioma de forma procedural
    /// Si no encuentra coincidencia exacta, genera algo similar
    pub fn adaptar_inteligente(&mut self, texto_es: &str, target_lang: &str) -> String {
        // Detectar tipo de contenido por palabras clave
        let t = texto_es.to_lowercase();
        
        match target_lang {
            "en" => {
                // Heridas
                if t.contains("nunca") || t.contains("jamás") || t.contains("dolor") || t.contains("culpa") || t.contains("perdon") {
                    return self.generar_herida_en();
                }
                
                // Máscaras
                if t.contains("confiado") || t.contains("frío") || t.contains("alegre") || t.contains("imagen") || t.contains("aparenta") {
                    return self.generar_mascara_en();
                }
                
                // Frases típicas
                if t.contains("problema") || t.contains("preocup") || t.contains("todo está") || t.contains("importa") {
                    return self.generar_frase_tipica_en();
                }
                
                // Biografía
                if t.contains("nació") || t.contains("creció") || t.contains("infancia") || t.contains("pasó") {
                    return self.generar_biografia_fragmento_en();
                }
                
                // Necesidades / Deseos / Mentiras
                if t.contains("aceptar") || t.contains("necesita") || t.contains("desea") || t.contains("cree que") || t.contains("control") {
                    return self.generar_necesidad_en();
                }
                
                // Arcos Narrativos
                if t.contains("posibilidad") || t.contains("potencial") || t.contains("incompleto") || t.contains("perdido") {
                    return self.generar_arco_inicio_en();
                }
                if t.contains("momento") || t.contains("decisión") || t.contains("elegir") || t.contains("quiebre") || t.contains("confronta") {
                    return self.generar_arco_quiebre_en();
                }
                if t.contains("aprende") || t.contains("finalmente") || t.contains("paz") || t.contains("resolución") || t.contains("nuevo") {
                    return self.generar_arco_resolucion_en();
                }
                
                // Ganchos
                if t.contains("oportunidad") || t.contains("forzado") || t.contains("debe") || t.contains("misión") || t.contains("encargo") {
                    return self.generar_gancho_en();
                }
                
                // FALLBACK RADICAL: Si nada coincide, devolver algo en inglés genérico
                self.generar_fallback_en()
            },
            "jp" => {
                // Detección heurística mejorada
                
                // Heridas
                if t.contains("nunca") || t.contains("jamás") || t.contains("dolor") || t.contains("culpa") {
                    return self.generar_herida_jp();
                }
                
                // Máscaras
                if t.contains("confiado") || t.contains("frío") || t.contains("alegre") || t.contains("imagen") {
                    return self.generar_mascara_jp();
                }
                
                // Frases típicas
                if t.contains("problema") || t.contains("preocup") || t.contains("todo está") || t.contains("importa") {
                    return self.generar_frase_tipica_jp();
                }
                
                // Biografía
                if t.contains("nació") || t.contains("creció") || t.contains("infancia") {
                    return self.generar_biografia_fragmento_jp();
                }
                
                // Necesidades / Deseos / Mentiras (Palabras clave del sistema o comunes)
                if t.contains("aceptar") || t.contains("necesita") || t.contains("desea") || t.contains("cree que") || t.contains("control") {
                    return self.generar_necesidad_jp();
                }
                
                // Arcos Narrativos
                if t.contains("posibilidad") || t.contains("potencial") || t.contains("incompleto") {
                    return self.generar_arco_inicio_jp();
                }
                if t.contains("momento") || t.contains("decisión") || t.contains("elegir") || t.contains("quiebre") {
                    return self.generar_arco_quiebre_jp();
                }
                if t.contains("aprende") || t.contains("finalmente") || t.contains("paz") || t.contains("resolución") {
                    return self.generar_arco_resolucion_jp();
                }
                
                // Ganchos
                if t.contains("oportunidad") || t.contains("forzado") || t.contains("debe") || t.contains("misión") {
                    return self.generar_gancho_jp();
                }
                
                // FALLBACK RADICAL: Si nada coincide, devolver algo en japonés genérico en lugar de español.
                // Esto garantiza que NO HAYA ESPAÑOL MEZCLADO.
                self.generar_fallback_jp()
            },
            _ => texto_es.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generacion_determinista() {
        let mut gen1 = ProceduralTextGenerator::with_seed(12345);
        let mut gen2 = ProceduralTextGenerator::with_seed(12345);
        
        assert_eq!(gen1.generar_herida_es(), gen2.generar_herida_es());
    }
    
    #[test]
    fn test_variedad() {
        let mut gen = ProceduralTextGenerator::with_seed(42);
        let mut resultados = std::collections::HashSet::new();
        
        for _ in 0..20 {
            resultados.insert(gen.generar_herida_es());
        }
        
        // Debe generar al menos 10 variaciones diferentes
        assert!(resultados.len() >= 10);
    }
}
