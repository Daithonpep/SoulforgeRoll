// ═══════════════════════════════════════════════════════════════════════════════
// SOULFORGE - Literary Adaptation Layer (Capa de Adaptación Literaria)
// ═══════════════════════════════════════════════════════════════════════════════
// 
// Sistema de adaptación PROCEDURAL - Sin dependencia de IA externa
// 
// Pipeline:
//   Motor Español (canónico) → Diccionarios de Estilo → Salida EN/JP pulida
//
// Características:
//   ✓ Sin IA externa (Ollama, GPT, etc.)
//   ✓ Determinista (misma semilla = misma salida)
//   ✓ Templates literarios de alta calidad
//   ✓ Sistema de variación basado en semilla
// ═══════════════════════════════════════════════════════════════════════════════

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use super::procedural_text::ProceduralTextGenerator;

/// ═══════════════════════════════════════════════════════════════════════════════
/// Idiomas objetivo para adaptación
/// ═══════════════════════════════════════════════════════════════════════════════
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetLanguage {
    English,
    Japanese,
}

impl TargetLanguage {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::English),
            "jp" => Some(Self::Japanese),
            _ => None,
        }
    }
}

/// ═══════════════════════════════════════════════════════════════════════════════
/// Adaptador Literario Procedural
/// ═══════════════════════════════════════════════════════════════════════════════
pub struct LiteraryAdapter {
    rng: ChaCha8Rng,
    // Diccionarios de transformación
    es_to_en: HashMap<&'static str, Vec<&'static str>>,
    es_to_jp: HashMap<&'static str, Vec<&'static str>>,
    // Templates de frases literarias
    english_templates: LiteraryTemplates,
    japanese_templates: LiteraryTemplates,
}

/// Templates literarios para generar texto con estilo
#[derive(Default)]
pub struct LiteraryTemplates {
    // Heridas emocionales
    pub wounds: Vec<&'static str>,
    // Máscaras
    pub masks: Vec<&'static str>,
    // Deseos
    pub desires: Vec<&'static str>,
    // Necesidades
    pub needs: Vec<&'static str>,
    // Mentiras que cree
    pub lies: Vec<&'static str>,
    // Verdades
    pub truths: Vec<&'static str>,
    // Frases de biografía
    pub biography_intros: Vec<&'static str>,
    // Conectores narrativos
    pub connectors: Vec<&'static str>,
}

impl LiteraryAdapter {
    /// Crear adaptador con semilla para reproducibilidad
    pub fn with_seed(seed: u64) -> Self {
        let mut adapter = Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            es_to_en: HashMap::new(),
            es_to_jp: HashMap::new(),
            english_templates: LiteraryTemplates::default(),
            japanese_templates: LiteraryTemplates::default(),
        };
        adapter.init_dictionaries();
        adapter.init_templates();
        adapter
    }
    
    /// Crear adaptador con entropía aleatoria
    pub fn new() -> Self {
        Self::with_seed(rand::random())
    }

    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// Inicializar diccionarios de transformación ES → EN/JP
    /// ═══════════════════════════════════════════════════════════════════════════
    fn init_dictionaries(&mut self) {
        // ─── ESPAÑOL → INGLÉS ───
        // Frases emocionales
        self.es_to_en.insert("Nunca pudo perdonarse", vec![
            "He never learned to forgive himself",
            "Forgiveness remained forever out of reach",
            "Self-absolution was a language he never spoke",
        ]);
        self.es_to_en.insert("Arrastra una culpa", vec![
            "He carries a guilt",
            "A weight of guilt follows him",
            "Guilt clings to him like a shadow",
        ]);
        self.es_to_en.insert("que nunca logró nombrar", vec![
            "he never learned to name",
            "that defies all words",
            "beyond the reach of language",
        ]);
        self.es_to_en.insert("La traición", vec![
            "The betrayal",
            "That treachery",
            "The breach of trust",
        ]);
        self.es_to_en.insert("dejó una marca", vec![
            "left a mark",
            "carved a scar",
            "burned a brand",
        ]);
        self.es_to_en.insert("que el tiempo no borra", vec![
            "that time cannot heal",
            "beyond time's gentle mercy",
            "that refuses to fade",
        ]);
        self.es_to_en.insert("Desea", vec!["Yearns for", "Craves", "Seeks"]);
        self.es_to_en.insert("Necesita", vec!["Needs", "Requires", "Must find"]);
        self.es_to_en.insert("La mentira que cree", vec![
            "The lie he believes",
            "The false truth he clings to",
            "His cherished deception",
        ]);
        self.es_to_en.insert("La verdad que necesita", vec![
            "The truth he needs",
            "What he must come to understand",
            "The revelation awaiting him",
        ]);
        
        // ─── VOCABULARIO CLAVE (Roles, Tonos, Mundo) ───
        self.es_to_en.insert("Héroe", vec!["Hero", "Champion"]);
        self.es_to_en.insert("Villano", vec!["Villain", "Antagonist"]);
        self.es_to_en.insert("Mentor", vec!["Mentor", "Guide"]);
        self.es_to_en.insert("Cambiante", vec!["Shapeshifter", "Trickster"]);
        self.es_to_en.insert("Guardián", vec!["Guardian", "Watcher"]);
        self.es_to_en.insert("Pícaro", vec!["Rogue", "Scoundrel"]);
        self.es_to_en.insert("Profeta", vec!["Prophet", "Seer"]);
        
        self.es_to_en.insert("Oscuro", vec!["Dark", "Grim"]);
        self.es_to_en.insert("Luminoso", vec!["Light", "Radiant"]);
        self.es_to_en.insert("Gritty", vec!["Gritty", "Raw"]);
        self.es_to_en.insert("Noble", vec!["Noble", "High"]);
        self.es_to_en.insert("Trágico", vec!["Tragic", "Doomed"]);
        self.es_to_en.insert("Cínico", vec!["Cynical", "Jaded"]);

        self.es_to_en.insert("Masculino", vec!["Male"]);
        self.es_to_en.insert("Femenino", vec!["Female"]);
        
        // ─── MENTIRAS (LIES) ───
        self.es_to_en.insert("Estoy solo porque todos me traicionarán", vec!["I am alone because everyone will betray me"]);
        self.es_to_en.insert("Si no soy fuerte, no valgo nada", vec!["If I am not strong, I am nothing"]);
        self.es_to_en.insert("No merezco ser feliz", vec!["I do not deserve to be happy"]);
        self.es_to_en.insert("El mundo es injusto, debo serlo también", vec!["The world is unfair, I must be too"]);
        self.es_to_en.insert("Si sacrifico todo, seré suficiente", vec!["If I sacrifice everything, I will be enough"]);
        
        // ─── VERDADES (TRUTHS) ───
        self.es_to_en.insert("La vulnerabilidad es el precio de la conexión", vec!["Vulnerability is the price of connection"]);
        self.es_to_en.insert("La fortaleza incluye aceptar límites", vec!["Strength includes accepting limits"]);
        self.es_to_en.insert("El perdón es posible", vec!["Forgiveness is possible"]);
        self.es_to_en.insert("Puede elegir ser diferente al mundo que lo hirió", vec!["They can choose to be different from the world that hurt them"]);
        self.es_to_en.insert("Ser amado sin hacer nada extraordinario", vec!["To be loved without doing anything extraordinary"]);

        // ─── MÁSCARAS (MASKS) ───
        self.es_to_en.insert("Alguien que tiene todo bajo control", vec!["Someone who has everything under control"]);
        self.es_to_en.insert("No hay problema sin solución", vec!["There is no problem without a solution"]);
        self.es_to_en.insert("Alguien que no necesita a nadie", vec!["Someone who needs no one"]);
        self.es_to_en.insert("Las emociones son debilidad", vec!["Emotions are weakness"]);
        self.es_to_en.insert("Alguien a quien nada afecta", vec!["Someone unaffected by anything"]);
        self.es_to_en.insert("¿Por qué preocuparse?", vec!["Why worry?"]);
        self.es_to_en.insert("Alguien indispensable", vec!["Someone indispensable"]);
        self.es_to_en.insert("Tus problemas son más importantes", vec!["Your problems are more important"]);
        self.es_to_en.insert("Alguien demasiado listo para esperanzas", vec!["Someone too smart for hope"]);
        self.es_to_en.insert("Ya lo veía venir", vec!["I saw it coming"]);

        // ─── DESEOS (DESIRES) ───
        self.es_to_en.insert("Poder absoluto", vec!["Absolute power"]);
        self.es_to_en.insert("Venganza", vec!["Revenge"]);
        self.es_to_en.insert("Reconocimiento", vec!["Recognition"]);
        self.es_to_en.insert("Proteger a alguien", vec!["To protect someone"]);
        self.es_to_en.insert("Un lugar donde pertenecer", vec!["A place to belong"]);
        
        // ─── SOMBRAS (SHADOWS) ───
        self.es_to_en.insert("La crueldad que es capaz de ejercer", vec!["The cruelty they are capable of"]);
        self.es_to_en.insert("El egoísmo tras el altruismo", vec!["The selfishness behind the altruism"]);
        self.es_to_en.insert("El miedo que lo paraliza", vec!["The fear that paralyzes them"]);
        self.es_to_en.insert("La envidia que corroe", vec!["The envy that corrodes"]);
        self.es_to_en.insert("La necesidad de control", vec!["The need for control"]);
        
        // ─── ARCOS (ARCS) ───
        self.es_to_en.insert("Perdido, incompleto, sin saber su potencial", vec!["Lost, incomplete, unaware of their potential"]);
        self.es_to_en.insert("En la cima, arrogante, ciego a sus debilidades", vec!["At the peak, arrogant, blind to their weaknesses"]);
        self.es_to_en.insert("Formado, resistente, existe para forjar a otros", vec!["Formed, resilient, existing to forge others"]);
        self.es_to_en.insert("Manchado, perseguido por su pasado", vec!["Stained, haunted by their past"]);
        self.es_to_en.insert("Puro de corazón pero ingenuo", vec!["Pure of heart but naive"]);
        self.es_to_en.insert("En conflicto, en el umbral entre dos mundos", vec!["In conflict, on the threshold between two worlds"]);

        self.es_to_en.insert("Héroe", vec!["Hero", "Champion", "Protagonist"]);
        self.es_to_en.insert("Villano", vec!["Villain", "Antagonist", "Shadow"]);
        self.es_to_en.insert("Mentor", vec!["Mentor", "Guide", "Sage"]);
        self.es_to_en.insert("Cambiante", vec!["Shapeshifter", "Trickster", "Chameleon"]);
        self.es_to_en.insert("Guardián", vec!["Guardian", "Protector", "Sentinel"]);
        self.es_to_en.insert("Pícaro", vec!["Rogue", "Scoundrel", "Outlaw"]);
        self.es_to_en.insert("Profeta", vec!["Prophet", "Seer", "Oracle"]);
        
        self.es_to_en.insert("Oscuro", vec!["Dark", "Grim", "Shadowed"]);
        self.es_to_en.insert("Luminoso", vec!["Light", "Radiant", "Hopeful"]);
        self.es_to_en.insert("Gritty", vec!["Gritty", "Raw", "Hard-boiled"]);
        self.es_to_en.insert("Noble", vec!["Noble", "High", "Honorable"]);
        self.es_to_en.insert("Trágico", vec!["Tragic", "Doomed", "Melancholy"]);
        self.es_to_en.insert("Cínico", vec!["Cynical", "Jaded", "Skeptical"]);

        self.es_to_en.insert("Masculino", vec!["Male"]);
        self.es_to_en.insert("Femenino", vec!["Female"]);
        
        // ─── ESPAÑOL → JAPONÉS ───
        // Frases emocionales - Estilo literario japonés (corto, implícito)
        self.es_to_jp.insert("Nunca pudo perdonarse", vec![
            "彼は、まだ自分を許せていない",
            "許しは、遠い",
            "自責の念が消えない",
        ]);
        self.es_to_jp.insert("Arrastra una culpa", vec![
            "罪悪感を背負っている",
            "心に重荷がある",
            "罪の重さを知る",
        ]);
        self.es_to_jp.insert("que nunca logró nombrar", vec![
            "言葉にできない",
            "名付けられない",
            "口にすることさえできない",
        ]);
        self.es_to_jp.insert("La traición", vec![
            "裏切り",
            "あの背信",
            "信頼の崩壊",
        ]);
        self.es_to_jp.insert("dejó una marca", vec![
            "傷を残した",
            "痕跡が消えない",
            "刻まれた",
        ]);
        self.es_to_jp.insert("que el tiempo no borra", vec![
            "時が癒せない",
            "時間では消えない",
            "永遠に残る",
        ]);
        self.es_to_jp.insert("Desea", vec!["望むのは", "求めるのは", "欲するのは"]);
        self.es_to_jp.insert("Necesita", vec!["必要なのは", "本当に求めるのは", "心が求めるのは"]);
        self.es_to_jp.insert("La mentira que cree", vec![
            "信じている嘘",
            "彼の偽りの真実",
            "心を縛る嘘",
        ]);
        self.es_to_jp.insert("La verdad que necesita", vec![
            "必要な真実",
            "知るべき真実",
            "受け入れるべき真実",
        ]);

        // ─── VOCABULARIO CLAVE JAPONÉS ───
        self.es_to_jp.insert("Héroe", vec!["英雄", "勇者"]);
        self.es_to_jp.insert("Villano", vec!["悪役", "敵"]);
        self.es_to_jp.insert("Mentor", vec!["師匠", "先生"]);
        self.es_to_jp.insert("Cambiante", vec!["変化する者", "トリックスター"]);
        self.es_to_jp.insert("Guardián", vec!["守護者", "番人"]);
        self.es_to_jp.insert("Pícaro", vec!["ならず者", "悪党"]);
        self.es_to_jp.insert("Profeta", vec!["予言者", "巫女"]);
        
        self.es_to_jp.insert("Oscuro", vec!["暗黒", "闇"]);
        self.es_to_jp.insert("Luminoso", vec!["光", "輝き"]);
        self.es_to_jp.insert("Gritty", vec!["無骨", "冷徹"]);
        self.es_to_jp.insert("Noble", vec!["高潔", "貴族"]);
        self.es_to_jp.insert("Trágico", vec!["悲劇", "哀愁"]);
        self.es_to_jp.insert("Cínico", vec!["冷笑", "皮肉"]);

        self.es_to_jp.insert("Masculino", vec!["男性"]);
        self.es_to_jp.insert("Femenino", vec!["女性"]);
        
        // ─── MENTIRAS (JP) ───
        self.es_to_jp.insert("Estoy solo porque todos me traicionarán", vec!["誰もが裏切るから、私は一人だ"]);
        self.es_to_jp.insert("Si no soy fuerte, no valgo nada", vec!["強くなければ、価値がない"]);
        self.es_to_jp.insert("No merezco ser feliz", vec!["私に幸せになる資格はない"]);
        self.es_to_jp.insert("El mundo es injusto, debo serlo también", vec!["世界は不公平だ、私もそうあるべきだ"]);
        self.es_to_jp.insert("Si sacrifico todo, seré suficiente", vec!["全てを犠牲にすれば、満たされるだろう"]);
        
        // ─── MÁSCARAS (JP) ───
        self.es_to_jp.insert("Alguien que tiene todo bajo control", vec!["全てを掌握している者"]);
        self.es_to_jp.insert("No hay problema sin solución", vec!["解決できない問題はない"]);
        self.es_to_jp.insert("Alguien que no necesita a nadie", vec!["誰も必要としない者"]);
        self.es_to_jp.insert("Las emociones son debilidad", vec!["感情は弱さである"]);
        self.es_to_jp.insert("Alguien a quien nada afecta", vec!["何にも動じない者"]);
        self.es_to_jp.insert("¿Por qué preocuparse?", vec!["なぜ心配する？"]);
        self.es_to_jp.insert("Alguien indispensable", vec!["なくてはならない存在"]);
        self.es_to_jp.insert("Tus problemas son más importantes", vec!["あなたの問題の方が重要だ"]);
        self.es_to_jp.insert("Alguien demasiado listo para esperanzas", vec!["希望を持つには賢すぎる者"]);
        self.es_to_jp.insert("Ya lo veía venir", vec!["やっぱりそうなった"]);

        // ─── DESEOS (JP) ───
        self.es_to_jp.insert("Poder absoluto", vec!["絶対的な力"]);
        self.es_to_jp.insert("Venganza", vec!["復讐"]);
        self.es_to_jp.insert("Reconocimiento", vec!["承認"]);
        self.es_to_jp.insert("Proteger a alguien", vec!["誰かを守ること"]);
        self.es_to_jp.insert("Un lugar donde pertenecer", vec!["居場所"]);
        
        // ─── SOMBRAS (JP) ───
        self.es_to_jp.insert("La crueldad que es capaz de ejercer", vec!["行使しうる残酷さ"]);
        self.es_to_jp.insert("El egoísmo tras el altruismo", vec!["利他主義の裏にある利己の心"]);
        self.es_to_jp.insert("El miedo que lo paraliza", vec!["麻痺させる恐怖"]);
        self.es_to_jp.insert("La envidia que corroe", vec!["蝕む嫉妬"]);
        self.es_to_jp.insert("La necesidad de control", vec!["支配への渇望"]);
        
        // ─── ARCOS (JP) ───
        self.es_to_jp.insert("Perdido, incompleto, sin saber su potencial", vec!["迷い、不完全、己の可能性を知らぬ者"]);
        self.es_to_jp.insert("En la cima, arrogante, ciego a sus debilidades", vec!["頂点に立ち、傲慢で、弱点が見えぬ者"]);
        self.es_to_jp.insert("Formado, resistente, existe para forjar a otros", vec!["鍛えられ、強靭、他者を鍛えるために存在する"]);
        self.es_to_jp.insert("Manchado, perseguido por su pasado", vec!["汚れ、過去に追われる者"]);
        self.es_to_jp.insert("Puro de corazón pero ingenuo", vec!["心は清らかだが、世間知らず"]);
        self.es_to_jp.insert("En conflicto, en el umbral entre dos mundos", vec!["葛藤し、二つの世界の狭間に立つ"]);
        self.es_to_jp.insert("Héroe", vec!["英雄", "勇者"]);
        self.es_to_jp.insert("Villano", vec!["悪役", "敵"]);
        self.es_to_jp.insert("Mentor", vec!["師匠", "導き手"]);
        self.es_to_jp.insert("Cambiante", vec!["変化する者", "道化師"]);
        self.es_to_jp.insert("Guardián", vec!["守護者", "番人"]);
        self.es_to_jp.insert("Pícaro", vec!["ならず者", "悪党"]);
        self.es_to_jp.insert("Profeta", vec!["予言者", "巫女"]);
        
        self.es_to_jp.insert("Oscuro", vec!["暗黒", "闇"]);
        self.es_to_jp.insert("Luminoso", vec!["光", "輝き"]);
        self.es_to_jp.insert("Gritty", vec!["無骨", "冷徹"]);
        self.es_to_jp.insert("Noble", vec!["高潔", "貴族"]);
        self.es_to_jp.insert("Trágico", vec!["悲劇", "哀愁"]);
        self.es_to_jp.insert("Cínico", vec!["冷笑", "皮肉"]);

        self.es_to_jp.insert("Masculino", vec!["男性"]);
        self.es_to_jp.insert("Femenino", vec!["女性"]);
        
        // ─── IDENTIDAD FÍSICA (RASGOS, VESTIMENTA, VOZ, MANIERISMOS) ───
        // Rasgos distintivos
        self.es_to_en.insert("Ojos que parecen ver más de lo que deberían", vec!["Eyes that seem to see more than they should"]);
        self.es_to_en.insert("Una sonrisa que nunca llega a los ojos", vec!["A smile that never reaches the eyes"]);
        self.es_to_en.insert("Manos que tiemblan cuando está quieto", vec!["Hands that tremble when still"]);
        self.es_to_en.insert("Postura militar incluso dormido", vec!["Military posture even when asleep"]);
        self.es_to_en.insert("Una mirada que hace sentir juzgado", vec!["A gaze that makes one feel judged"]);
        
        self.es_to_jp.insert("Ojos que parecen ver más de lo que deberían", vec!["見えすぎる目"]);
        self.es_to_jp.insert("Una sonrisa que nunca llega a los ojos", vec!["目に届かない笑顔"]);
        self.es_to_jp.insert("Manos que tiemblan cuando está quieto", vec!["静止時に震える手"]);
        self.es_to_jp.insert("Postura militar incluso dormido", vec!["眠っていても軍人の姿勢"]);
        self.es_to_jp.insert("Una mirada que hace sentir juzgado", vec!["裁かれているような視線"]);
        
        // Vestimenta
        self.es_to_en.insert("Ropa práctica, siempre lista para huir", vec!["Practical clothes, always ready to flee"]);
        self.es_to_en.insert("Capas que ocultan su verdadera forma", vec!["Layers that hide their true form"]);
        self.es_to_en.insert("Vestimenta que contradice su rol social", vec!["Clothing that contradicts their social role"]);
        self.es_to_en.insert("Algo elegante con rastros de desgaste", vec!["Something elegant with traces of wear"]);
        self.es_to_en.insert("Simple, como queriendo pasar desapercibido", vec!["Simple, as if wanting to go unnoticed"]);
        
        self.es_to_jp.insert("Ropa práctica, siempre lista para huir", vec!["実用的な服、いつでも逃げられる"]);
        self.es_to_jp.insert("Capas que ocultan su verdadera forma", vec!["本当の姿を隠す重ね着"]);
        self.es_to_jp.insert("Vestimenta que contradice su rol social", vec!["社会的役割と矛盾する服装"]);
        self.es_to_jp.insert("Algo elegante con rastros de desgaste", vec!["摩耗の跡がある上品な服"]);
        self.es_to_jp.insert("Simple, como queriendo pasar desapercibido", vec!["目立たないようにシンプルな服"]);
        
        // Voz
        self.es_to_en.insert("Grave y medida, cada palabra elegida", vec!["Deep and measured, each word chosen"]);
        self.es_to_en.insert("Suave pero con un filo que advierte", vec!["Soft but with an edge that warns"]);
        self.es_to_en.insert("Cansada, como quien no espera ser escuchado", vec!["Tired, like one who doesn't expect to be heard"]);
        self.es_to_en.insert("Intensa, incluso cuando susurra", vec!["Intense, even when whispering"]);
        self.es_to_en.insert("Cambiante según la audiencia", vec!["Changing according to the audience"]);
        
        self.es_to_jp.insert("Grave y medida, cada palabra elegida", vec!["低く慎重、言葉を選ぶ"]);
        self.es_to_jp.insert("Suave pero con un filo que advierte", vec!["柔らかいが警告する鋭さがある"]);
        self.es_to_jp.insert("Cansada, como quien no espera ser escuchado", vec!["疲れた声、聞かれることを期待していない"]);
        self.es_to_jp.insert("Intensa, incluso cuando susurra", vec!["囁いても強烈"]);
        self.es_to_jp.insert("Cambiante según la audiencia", vec!["聴衆によって変わる"]);
        
        // Manierismos
        self.es_to_en.insert("Cuenta cosas obsesivamente - pasos, latidos", vec!["Counts things obsessively - steps, heartbeats"]);
        self.es_to_en.insert("Habla de sí mismo en tercera persona bajo estrés", vec!["Speaks of himself in third person under stress"]);
        self.es_to_en.insert("Repite la última palabra de otros antes de responder", vec!["Repeats others' last word before responding"]);
        self.es_to_en.insert("Se toca una cicatriz invisible cuando miente", vec!["Touches an invisible scar when lying"]);
        self.es_to_en.insert("Cierra los ojos antes de decisiones importantes", vec!["Closes eyes before important decisions"]);
        
        self.es_to_jp.insert("Cuenta cosas obsesivamente - pasos, latidos", vec!["強迫的に数える - 歩数、心拍"]);
        self.es_to_jp.insert("Habla de sí mismo en tercera persona bajo estrés", vec!["ストレス下で三人称で話す"]);
        self.es_to_jp.insert("Repite la última palabra de otros antes de responder", vec!["返答前に他人の最後の言葉を繰り返す"]);
        self.es_to_jp.insert("Se toca una cicatriz invisible cuando miente", vec!["嘘をつく時に見えない傷跡に触れる"]);
        self.es_to_jp.insert("Cierra los ojos antes de decisiones importantes", vec!["重要な決断の前に目を閉じる"]);
        
        // Cicatrices
        self.es_to_en.insert("Una quemadura en el antebrazo que oculta", vec!["A burn on the forearm that they hide"]);
        self.es_to_en.insert("Una línea fina en el cuello", vec!["A thin line on the neck"]);
        self.es_to_en.insert("Marcas en los nudillos", vec!["Marks on the knuckles"]);
        
        self.es_to_jp.insert("Una quemadura en el antebrazo que oculta", vec!["隠している前腕の火傷"]);
        self.es_to_jp.insert("Una línea fina en el cuello", vec!["首の細い線"]);
        self.es_to_jp.insert("Marcas en los nudillos", vec!["拳の跡"]);
        
        // Títulos
        self.es_to_en.insert("El Silencioso", vec!["The Silent One"]);
        self.es_to_en.insert("Manos Rojas", vec!["Red Hands"]);
        self.es_to_en.insert("El Último de su Nombre", vec!["The Last of His Name"]);
        self.es_to_en.insert("La Sombra", vec!["The Shadow"]);
        self.es_to_en.insert("Corazón de Hierro", vec!["Iron Heart"]);
        self.es_to_en.insert("El Errante", vec!["The Wanderer"]);
        self.es_to_en.insert("Sin Nombre", vec!["Nameless"]);
        self.es_to_en.insert("El Caído", vec!["The Fallen"]);
        self.es_to_en.insert("La Voz del Trueno", vec!["Voice of Thunder"]);
        self.es_to_en.insert("Ojos de Tormenta", vec!["Storm Eyes"]);
        self.es_to_en.insert("El Roto", vec!["The Broken"]);
        self.es_to_en.insert("La Esperanza Perdida", vec!["Lost Hope"]);
        
        self.es_to_jp.insert("El Silencioso", vec!["沈黙の者"]);
        self.es_to_jp.insert("Manos Rojas", vec!["赤い手"]);
        self.es_to_jp.insert("El Último de su Nombre", vec!["最後の名を持つ者"]);
        self.es_to_jp.insert("La Sombra", vec!["影"]);
        self.es_to_jp.insert("Corazón de Hierro", vec!["鉄の心"]);
        self.es_to_jp.insert("El Errante", vec!["放浪者"]);
        self.es_to_jp.insert("Sin Nombre", vec!["名無し"]);
        self.es_to_jp.insert("El Caído", vec!["堕ちた者"]);
        self.es_to_jp.insert("La Voz del Trueno", vec!["雷鳴の声"]);
        self.es_to_jp.insert("Ojos de Tormenta", vec!["嵐の目"]);
        self.es_to_jp.insert("El Roto", vec!["壊れた者"]);
        self.es_to_jp.insert("La Esperanza Perdida", vec!["失われた希望"]);

        // ─── ESTRUCTURA BIOGRÁFICA (TÍTULOS Y FASES) ───
        self.es_to_en.insert("EL MOMENTO CERO", vec!["POINT ZERO"]);
        self.es_to_en.insert("FLASHBACK: EL ORIGEN", vec!["FLASHBACK: THE ORIGIN"]);
        self.es_to_en.insert("La Secuela", vec!["The Aftermath"]);
        self.es_to_en.insert("El Ahora", vec!["The Now"]);
        self.es_to_en.insert("V. La Madurez", vec!["V. Maturity"]);
        self.es_to_en.insert("VI. El Legado", vec!["VI. Legacy"]);
        self.es_to_en.insert("VII. El Crepúsculo", vec!["VII. Twilight"]);
        
        self.es_to_en.insert("Estilo: Crónica Lineal", vec!["Style: Linear Chronicle"]);
        self.es_to_en.insert("Estilo: In Media Res (Fracturado)", vec!["Style: In Media Res (Fractured)"]);
        self.es_to_en.insert("Estilo: Psicológico (Introspectivo)", vec!["Style: Psychological (Introspective)"]);
        self.es_to_en.insert("Conflictos Latentes", vec!["Latent Conflicts"]);
        self.es_to_en.insert("Un Destello de Luz", vec!["A Glimpse of Light"]);
        
        // Frases puente de biografía
        self.es_to_en.insert("Y entonces llegó el quiebre.", vec!["And then the breaking point arrived."]);
        self.es_to_en.insert("Antes de eso, la vida de ", vec!["Before that, the life of "]);
        self.es_to_en.insert(" era otra historia.", vec![" was a different story."]);
        self.es_to_en.insert("Para entender el dolor, hay que mirar al principio.", vec!["To understand the pain, one must look at the beginning."]);
        self.es_to_en.insert("vive atrapado en una premisa:", vec!["lives trapped in a premise:"]);
        self.es_to_en.insert("La raíz no está en los hechos, sino en el impacto.", vec!["The root lies not in facts, but in impact."]);
        self.es_to_en.insert("Para sobrevivir,", vec!["To survive,"]);
        self.es_to_en.insert("construyó una armadura.", vec!["built an armor."]);

        // ─── FRASES DE CONFLICTO Y DINÁMICAS ───
        self.es_to_en.insert("Justicia vs Piedad", vec!["Justice vs Mercy"]);
        self.es_to_en.insert("Libertad vs Pertenencia", vec!["Freedom vs Belonging"]);
        self.es_to_en.insert("el castigo merecido", vec!["deserved punishment"]);
        self.es_to_en.insert("la compasión humana", vec!["human compassion"]);
        self.es_to_en.insert("ser libre sin ataduras", vec!["being free without tethers"]);
        self.es_to_en.insert("tener un lugar y gente", vec!["having a place and people"]);
        
        self.es_to_en.insert("vive en la tensión entre cumplir y querer.", vec!["lives in the tension between duty and desire."]);
        self.es_to_en.insert("el deber es un ancla pesada; el deseo, una marea que intenta arrastrarlo.", vec!["duty is a heavy anchor; desire, a tide trying to drag them away."]);
        self.es_to_en.insert("tiene gravedad propia.", vec!["has its own gravity."]);
        self.es_to_en.insert("Cada paso hacia adelante requiere soltar algo.", vec!["Every step forward requires letting go of something."]);
        self.es_to_en.insert("camina hacia adelante mirando hacia atrás.", vec!["walks forward while looking back."]);
        self.es_to_en.insert("oscila entre la necesidad de conexión y el instinto visceral", vec!["oscillates between the need for connection and the visceral instinct"]);
        self.es_to_en.insert("Bajar la guardia es peligroso.", vec!["Lowering one's guard is dangerous."]);
        self.es_to_en.insert("ha perfeccionado su máscara.", vec!["has perfected their mask."]);
        self.es_to_en.insert("conoce los límites de su zona segura.", vec!["knows the limits of their safe zone."]);
        self.es_to_en.insert("A veces la jaula es dorada, pero sigue siendo jaula.", vec!["Sometimes the cage is golden, but it remains a cage."]);
        self.es_to_en.insert("Crecer duele. Quedarse quieto asfixia.", vec!["Growing hurts. Staying still suffocates."]);
        self.es_to_en.insert("El horizonte llama a ", vec!["The horizon calls to "]); 
        self.es_to_en.insert("pero el suelo conocido sujeta sus pies con fuerza.", vec!["but the known ground holds their feet tightly."]);
        self.es_to_en.insert("ve el mundo en blanco y negro, pero su corazón a veces ve matices que la ley ignora.", vec!["sees the world in black and white, but their heart sometimes sees shades the law ignores."]);
        self.es_to_en.insert("La soledad es el precio de la libertad de ", vec!["Loneliness is the price of freedom for "]);
        
        self.es_to_en.insert("El Don de ", vec!["The Gift of "]);
        self.es_to_en.insert("La Victoria de ", vec!["The Victory of "]);
        self.es_to_en.insert("Su eco: ", vec!["Their echo: "]);
        self.es_to_en.insert("Lo impulsa hacia ", vec!["Drives them toward "]);

        // JP
        self.es_to_jp.insert("Justicia vs Piedad", vec!["正義 vs 慈悲"]);
        self.es_to_jp.insert("Libertad vs Pertenencia", vec!["自由 vs 所属"]);
        self.es_to_jp.insert("El Don de ", vec!["贈り物："]);
        self.es_to_jp.insert("La Victoria de ", vec!["勝利："]);

        self.es_to_jp.insert("EL MOMENTO CERO", vec!["ゼロ地点"]);
        self.es_to_jp.insert("FLASHBACK: EL ORIGEN", vec!["回想：起源"]);
        self.es_to_jp.insert("La Secuela", vec!["その後"]);
        self.es_to_jp.insert("El Ahora", vec!["現在"]);
        self.es_to_jp.insert("V. La Madurez", vec!["V. 成熟"]);
        self.es_to_jp.insert("VI. El Legado", vec!["VI. 遺産"]);
        self.es_to_jp.insert("VII. El Crepúsculo", vec!["VII. 黄昏"]);
        self.es_to_jp.insert("Estilo: Crónica Lineal", vec!["スタイル：年代記"]);
        self.es_to_jp.insert("Estilo: In Media Res (Fracturado)", vec!["スタイル：イン・メディア・レス（断片的）"]);
        self.es_to_jp.insert("Estilo: Psicológico (Introspectivo)", vec!["スタイル：心理的（内省的）"]);
        self.es_to_jp.insert("Conflictos Latentes", vec!["潜在的な対立"]);
        self.es_to_jp.insert("Un Destello de Luz", vec!["一筋の光"]);
        self.es_to_jp.insert("Y entonces llegó el quiebre.", vec!["そして、崩壊が訪れた。"]);
        self.es_to_jp.insert("Para entender el dolor, hay que mirar al principio.", vec!["痛みを理解するには、始まりを見なければならない。"]);
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// Inicializar templates literarios
    /// ═══════════════════════════════════════════════════════════════════════════
    fn init_templates(&mut self) {
        // ─── ENGLISH TEMPLATES ───
        self.english_templates = LiteraryTemplates {
            wounds: vec![
                "A wound that never quite healed",
                "The scar runs deeper than skin",
                "Some hurts become part of who we are",
                "The ghost of what was lost",
                "A fracture in the soul",
            ],
            masks: vec![
                "The face shown to the world",
                "A careful performance",
                "The armor of false confidence",
                "What others are allowed to see",
                "The lie made flesh",
            ],
            desires: vec![
                "What the heart thinks it wants",
                "The conscious pursuit",
                "The goal that drives all action",
                "Surface-level ambition",
                "The spoken dream",
            ],
            needs: vec![
                "What the soul truly craves",
                "The unspoken hunger",
                "The truth beneath the want",
                "What would truly heal",
                "The deeper current",
            ],
            lies: vec![
                "The comfortable fiction",
                "A truth too painful to question",
                "The foundation of false safety",
                "What must be believed to survive",
                "The story told to the mirror",
            ],
            truths: vec![
                "The revelation awaiting",
                "What must be faced",
                "The key to transformation",
                "Liberation through acceptance",
                "The path forward",
            ],
            biography_intros: vec![
                "Born into a world of",
                "From the earliest days",
                "The story begins with",
                "In the shadow of",
                "Beneath the weight of",
            ],
            connectors: vec![
                "And yet", "But beneath the surface", "In time", 
                "The truth was", "What remained was", "Until",
            ],
        };
        
        // ─── JAPANESE TEMPLATES ───
        self.japanese_templates = LiteraryTemplates {
            wounds: vec![
                "癒えない傷",
                "心に残る痛み",
                "過去の亡霊",
                "消えない記憶",
                "魂の裂け目",
            ],
            masks: vec![
                "外に見せる顔",
                "作られた仮面",
                "偽りの強さ",
                "見せかけの姿",
                "表の顔",
            ],
            desires: vec![
                "心が求めるもの",
                "意識の目標",
                "表の望み",
                "口にする夢",
                "追い求めるもの",
            ],
            needs: vec![
                "魂の渇き",
                "本当の飢え",
                "真の必要",
                "癒しへの道",
                "深い願い",
            ],
            lies: vec![
                "信じる嘘",
                "心地よい幻想",
                "疑えない真実",
                "生きるための物語",
                "鏡に語る虚構",
            ],
            truths: vec![
                "待つ啓示",
                "向き合うべきもの",
                "変化への鍵",
                "受容による解放",
                "前への道",
            ],
            biography_intros: vec![
                "物語の始まり",
                "幼き日より",
                "その運命は",
                "影の中で",
                "重荷の下で",
            ],
            connectors: vec![
                "しかし", "だが", "やがて", 
                "真実は", "残ったのは", "ついに",
            ],
        };
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// Adaptar texto español al idioma objetivo
    /// ═══════════════════════════════════════════════════════════════════════════
    pub fn adapt(&mut self, spanish_text: &str, target: TargetLanguage) -> String {
        if spanish_text.trim().is_empty() {
            return String::new();
        }
        
        match target {
            TargetLanguage::English => self.adapt_to_english(spanish_text),
            TargetLanguage::Japanese => self.adapt_to_japanese(spanish_text),
        }
    }
    
    /// Adaptar al inglés con estilo literario
    fn adapt_to_english(&mut self, text: &str) -> String {
        let mut result = text.to_string();
        let mut transformed = false;
        
        // Aplicar transformaciones del diccionario
        for (es_phrase, en_options) in &self.es_to_en {
            if result.contains(es_phrase) {
                let choice = en_options[self.rng.gen_range(0..en_options.len())];
                result = result.replace(es_phrase, choice);
                transformed = true;
            }
        }
        
        // Si no hubo transformaciones significativas o queremos asegurar variedad,
        // intentamos el generador procedural inteligente
        if !transformed || self.rng.gen_bool(0.3) {
            let seed = self.rng.next_u64();
            let mut proc_gen = ProceduralTextGenerator::with_seed(seed);
            
            // Si el texto sigue siendo muy similar al original (se asume español), forzamos intento procedural
            if result == text {
                 let procedural = proc_gen.adaptar_inteligente(text, "en");
                 if procedural != text {
                     return procedural;
                 }
            }
        }
        
        result
    }
    
    /// Adaptar al japonés con estilo literario
    fn adapt_to_japanese(&mut self, text: &str) -> String {
        let mut result = text.to_string();
        let mut transformed = false;
        
        // Aplicar transformaciones del diccionario
        for (es_phrase, jp_options) in &self.es_to_jp {
            if result.contains(es_phrase) {
                let choice = jp_options[self.rng.gen_range(0..jp_options.len())];
                result = result.replace(es_phrase, choice);
                transformed = true;
            }
        }
        
        // Fallback procedural inteligente para japonés
        // Priorizamos esto si no hubo match en el diccionario para evitar mezclar español con japonés
        if !transformed {
            let seed = self.rng.next_u64();
            let mut proc_gen = ProceduralTextGenerator::with_seed(seed);
            
            let procedural = proc_gen.adaptar_inteligente(text, "jp");
            if procedural != text {
                return procedural;
            }
        }
        
        result
    }
    
    /// ═══════════════════════════════════════════════════════════════════════════
    /// Obtener template literario aleatorio
    /// ═══════════════════════════════════════════════════════════════════════════
    pub fn get_wound_template(&mut self, target: TargetLanguage) -> &'static str {
        let templates = match target {
            TargetLanguage::English => &self.english_templates.wounds,
            TargetLanguage::Japanese => &self.japanese_templates.wounds,
        };
        templates[self.rng.gen_range(0..templates.len())]
    }
    
    pub fn get_mask_template(&mut self, target: TargetLanguage) -> &'static str {
        let templates = match target {
            TargetLanguage::English => &self.english_templates.masks,
            TargetLanguage::Japanese => &self.japanese_templates.masks,
        };
        templates[self.rng.gen_range(0..templates.len())]
    }
    
    pub fn get_desire_template(&mut self, target: TargetLanguage) -> &'static str {
        let templates = match target {
            TargetLanguage::English => &self.english_templates.desires,
            TargetLanguage::Japanese => &self.japanese_templates.desires,
        };
        templates[self.rng.gen_range(0..templates.len())]
    }
    
    pub fn get_need_template(&mut self, target: TargetLanguage) -> &'static str {
        let templates = match target {
            TargetLanguage::English => &self.english_templates.needs,
            TargetLanguage::Japanese => &self.japanese_templates.needs,
        };
        templates[self.rng.gen_range(0..templates.len())]
    }
    
    pub fn get_connector(&mut self, target: TargetLanguage) -> &'static str {
        let templates = match target {
            TargetLanguage::English => &self.english_templates.connectors,
            TargetLanguage::Japanese => &self.japanese_templates.connectors,
        };
        templates[self.rng.gen_range(0..templates.len())]
    }
}

/// ═══════════════════════════════════════════════════════════════════════════════
/// Función de conveniencia para adaptar texto
/// ═══════════════════════════════════════════════════════════════════════════════
pub fn adapt_text(spanish_text: &str, lang_code: &str, seed: u64) -> String {
    let target = match TargetLanguage::from_code(lang_code) {
        Some(t) => t,
        None => return spanish_text.to_string(), // Si es español, no adaptar
    };
    
    let mut adapter = LiteraryAdapter::with_seed(seed);
    adapter.adapt(spanish_text, target)
}

/// ═══════════════════════════════════════════════════════════════════════════════
/// Tests
/// ═══════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_language_detection() {
        assert!(matches!(TargetLanguage::from_code("en"), Some(TargetLanguage::English)));
        assert!(matches!(TargetLanguage::from_code("jp"), Some(TargetLanguage::Japanese)));
        assert!(TargetLanguage::from_code("es").is_none());
    }
    
    #[test]
    fn test_deterministic_output() {
        let text = "Nunca pudo perdonarse por lo que hizo";
        let result1 = adapt_text(text, "en", 12345);
        let result2 = adapt_text(text, "en", 12345);
        assert_eq!(result1, result2, "Same seed should produce same output");
    }
    
    #[test]
    fn test_english_adaptation() {
        let text = "Arrastra una culpa que nunca logró nombrar";
        let result = adapt_text(text, "en", 42);
        // Should contain English phrases, not Spanish
        assert!(!result.contains("Arrastra") || result != text);
    }
    
    #[test]
    fn test_japanese_adaptation() {
        let text = "Nunca pudo perdonarse";
        let result = adapt_text(text, "jp", 42);
        // Should contain Japanese characters
        assert!(result.chars().any(|c| c > '\u{3000}'));
    }
    
    #[test]
    fn test_spanish_passthrough() {
        let text = "Este es texto en español";
        let result = adapt_text(text, "es", 42);
        assert_eq!(result, text, "Spanish should pass through unchanged");
    }
}
