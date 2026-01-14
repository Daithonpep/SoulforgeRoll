//! Las 7 capas del alma - Sistema psicológico profundo

use rand::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// CAPA 1: ARQUETIPO JUNGUIANO
// ============================================================================

use super::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arquetipo {
    pub tipo: TipoArquetipo,
    pub manifestacion_luz: String,
    pub manifestacion_sombra: String,
    pub don_natural: String,
    pub debilidad: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoArquetipo {
    Inocente,
    Huerfano,
    Guerrero,
    Cuidador,
    Buscador,
    Destructor,
    Amante,
    Creador,
    Gobernante,
    Mago,
    Sabio,
    Bufon,
}

impl Arquetipo {
    pub fn generar(rng: &mut impl Rng, lang: &Language) -> Self {
        let tipos = vec![
            TipoArquetipo::Inocente, TipoArquetipo::Huerfano, TipoArquetipo::Guerrero,
            TipoArquetipo::Cuidador, TipoArquetipo::Buscador, TipoArquetipo::Destructor,
            TipoArquetipo::Amante, TipoArquetipo::Creador, TipoArquetipo::Gobernante,
            TipoArquetipo::Mago, TipoArquetipo::Sabio, TipoArquetipo::Bufon,
        ];
        let tipo = *tipos.choose(rng).unwrap();
        
        let (luz, sombra, don, debilidad) = Self::get_text(&tipo, lang);
        
        Self {
            tipo,
            manifestacion_luz: luz.to_string(),
            manifestacion_sombra: sombra.to_string(),
            don_natural: don.to_string(),
            debilidad: debilidad.to_string(),
        }
    }

    fn get_text(tipo: &TipoArquetipo, lang: &Language) -> (&'static str, &'static str, &'static str, &'static str) {
        match lang {
            Language::English => match tipo {
                TipoArquetipo::Guerrero => ("Courage to defend the weak", "Violence as first response", "Acting under extreme pressure", "Solving everything with confrontation"),
                TipoArquetipo::Cuidador => ("Genuine compassion and sacrifice", "Martyrdom and codependency", "Intuiting others' needs", "Neglecting own needs"),
                TipoArquetipo::Huerfano => ("Empathy with the marginalized", "Victimhood and inability to trust", "Surviving where others give up", "Expecting abandonment"),
                TipoArquetipo::Buscador => ("Courage to explore the unknown", "Inability to commit", "Finding paths where others see walls", "Never feeling at home"),
                TipoArquetipo::Destructor => ("Letting go of what no longer serves", "Nihilism without purpose", "Seeing truth beyond illusions", "Destroying what should be preserved"),
                TipoArquetipo::Amante => ("Passion that gives meaning", "Obsession and possessiveness", "Creating deep connections", "Depending on others to exist"),
                TipoArquetipo::Creador => ("Giving form to the non-existent", "Paralyzing perfectionism", "Seeing possibilities in the void", "Never being satisfied"),
                TipoArquetipo::Gobernante => ("Leadership serving the common good", "Tyranny and obsessive control", "Ordering chaos", "Inability to delegate"),
                TipoArquetipo::Mago => ("Transforming reality", "Manipulation, playing god", "Understanding hidden rules", "Arrogance before consequences"),
                TipoArquetipo::Sabio => ("Seeking truth regardless of cost", "Paralysis by analysis", "Seeing invisible patterns", "Knowing without acting"),
                TipoArquetipo::Bufon => ("Revealing truths with humor", "Evasion and inability to be serious", "Relieving tension in critical moments", "Hiding pain behind laughter"),
                TipoArquetipo::Inocente => ("Faith that inspires belief", "Denial of reality", "Seeing good in the worst", "Trusting when one shouldn't"),
            },
            Language::Japanese => match tipo {
                TipoArquetipo::Guerrero => ("弱者を守る勇気", "暴力的な解決", "極限状態での行動力", "全てを対立で解決しようとする"),
                TipoArquetipo::Cuidador => ("真の慈悲と犠牲", "自己犠牲と共依存", "他者の欲求を察知する", "自身の欲求を無視する"),
                TipoArquetipo::Huerfano => ("疎外された者への共感", "被害者意識と不信感", "他者が諦める場所での生存", "常に見捨てられることを予期する"),
                TipoArquetipo::Buscador => ("未知を探索する勇気", "定着できない", "壁の向こうに道を見出す", "どこにも居場所を感じない"),
                TipoArquetipo::Destructor => ("不要なものを手放す", "目的のない虚無主義", "幻想の裏にある真実を見る", "守るべきものまで破壊する"),
                TipoArquetipo::Amante => ("意味を与える情熱", "執着と独占欲", "深い絆を築く", "他者に存在意義を依存する"),
                TipoArquetipo::Creador => ("無から有を生む", "完璧主義による麻痺", "虚空に可能性を見る", "決して満足しない"),
                TipoArquetipo::Gobernante => ("公益に奉仕する統率力", "暴政と支配欲", "混沌に秩序をもたらす", "任せることができない"),
                TipoArquetipo::Mago => ("現実を変革する", "操作、神を演じる", "隠された法則を理解する", "結果を顧みない傲慢さ"),
                TipoArquetipo::Sabio => ("代償を問わず真理を求める", "分析による麻痺", "見えないパターンを見る", "行動なき知識"),
                TipoArquetipo::Bufon => ("ユーモアで真実を暴く", "逃避と真剣さの欠如", "緊張を緩和する", "笑いの裏に痛みを隠す"),
                TipoArquetipo::Inocente => ("信じる力を与える純粋さ", "現実逃避", "最悪の中に善を見る", "信じるべきでない時も信じる"),
            },
            Language::Espanol => match tipo {
                TipoArquetipo::Guerrero => ("Coraje para defender a los débiles", "Violencia como primera respuesta", "Actuar bajo presión extrema", "Resolver todo con confrontación"),
                TipoArquetipo::Cuidador => ("Compasión genuina y sacrificio", "Martirio y codependencia", "Intuir necesidades ajenas", "Descuidar las propias necesidades"),
                TipoArquetipo::Huerfano => ("Empatía con los marginados", "Victimismo e incapacidad de confiar", "Sobrevivir donde otros se rinden", "Esperar siempre el abandono"),
                TipoArquetipo::Buscador => ("Valentía para explorar lo desconocido", "Incapacidad de comprometerse", "Encontrar caminos donde otros ven muros", "Nunca sentirse en casa"),
                TipoArquetipo::Destructor => ("Dejar ir lo que ya no sirve", "Nihilismo sin propósito", "Ver la verdad tras las ilusiones", "Destruir lo que debería preservar"),
                TipoArquetipo::Amante => ("Pasión que da significado", "Obsesión y posesividad", "Crear conexiones profundas", "Depender del otro para existir"),
                TipoArquetipo::Creador => ("Dar forma a lo inexistente", "Perfeccionismo paralizante", "Ver posibilidades en el vacío", "Nunca estar satisfecho"),
                TipoArquetipo::Gobernante => ("Liderazgo al servicio del bien común", "Tiranía y control obsesivo", "Ordenar el caos", "Incapacidad de delegar"),
                TipoArquetipo::Mago => ("Transformar la realidad", "Manipulación, jugar a ser dios", "Entender reglas ocultas", "Arrogancia ante consecuencias"),
                TipoArquetipo::Sabio => ("Buscar la verdad sin importar el costo", "Parálisis por análisis", "Ver patrones invisibles", "Conocer sin actuar"),
                TipoArquetipo::Bufon => ("Revelar verdades con humor", "Evasión e incapacidad de seriedad", "Aliviar tensión en momentos críticos", "Esconder dolor tras la risa"),
                TipoArquetipo::Inocente => ("Fe que inspira a creer", "Negación de la realidad", "Ver el bien en lo peor", "Confiar cuando no debería"),
            },
        }
    }
}

// ============================================================================
// CAPA 2: LA HERIDA (THE GHOST)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Herida {
    pub tipo: TipoHerida,
    pub edad_cuando_ocurrio: EdadHerida,
    pub causante: String,
    pub circunstancia: String,
    pub como_lo_cambio: String,
    pub gatillo_emocional: String,
    pub mecanismo_defensa: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoHerida {
    Abandono,
    Traicion,
    Perdida,
    Humillacion,
    Injusticia,
    Impotencia,
    Culpa,
    Rechazo,
    Negligencia,
    Violencia,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EdadHerida {
    PrimeraInfancia,
    Infancia,
    Adolescencia,
    AdultezTemprana,
    Madurez,
}

impl Herida {
    pub fn generar(rng: &mut impl Rng, _mundo: &super::Mundo, lang: &Language) -> Self {
        let tipos = vec![
            TipoHerida::Abandono, TipoHerida::Traicion, TipoHerida::Perdida,
            TipoHerida::Humillacion, TipoHerida::Injusticia, TipoHerida::Impotencia,
            TipoHerida::Culpa, TipoHerida::Rechazo,
        ];
        let tipo = *tipos.choose(rng).unwrap();
        
        let edades = [EdadHerida::PrimeraInfancia, EdadHerida::Infancia, 
                      EdadHerida::Adolescencia, EdadHerida::AdultezTemprana];
        let edad = *edades.choose(rng).unwrap();
        
        let (mut causante, mut circunstancia, mut cambio, mut gatillo, mut defensa) = Self::generar_detalles(rng, &tipo);
        
        // Adaptar textos si no es español
        if *lang != Language::Espanol {
            let lang_code = match lang {
                Language::English => "en",
                Language::Japanese => "jp",
                _ => "es",
            };
            
            // Usar una semilla derivada para determinismo
            let seed = rng.next_u64();
            
            causante = super::adapter::adapt_text(&causante, lang_code, seed);
            circunstancia = super::adapter::adapt_text(&circunstancia, lang_code, seed.wrapping_add(1));
            cambio = super::adapter::adapt_text(&cambio, lang_code, seed.wrapping_add(2));
            gatillo = super::adapter::adapt_text(&gatillo, lang_code, seed.wrapping_add(3));
            defensa = super::adapter::adapt_text(&defensa, lang_code, seed.wrapping_add(4));
        }

        Self {
            tipo,
            edad_cuando_ocurrio: edad,
            causante,
            circunstancia,
            como_lo_cambio: cambio,
            gatillo_emocional: gatillo,
            mecanismo_defensa: defensa,
        }
    }
    
    fn generar_detalles(rng: &mut impl Rng, tipo: &TipoHerida) -> (String, String, String, String, String) {
        use super::gramatica::*;
        
        let tipo_str = match tipo {
            TipoHerida::Abandono => "Abandono",
            TipoHerida::Traicion => "Traicion",
            TipoHerida::Humillacion => "Humillacion",
            TipoHerida::Culpa => "Culpa",
            _ => "General",
        };
        
        // Usar pools expandidos del módulo gramática
        let causante = sujetos_causantes().choose(rng).unwrap().to_string();
        let circunstancia = generar_circunstancia_herida(rng, tipo_str);
        let cambio = cambios_internos().choose(rng).unwrap().to_string();
        let gatillo = gatillos_emocionales().choose(rng).unwrap().to_string();
        let defensa = mecanismos_defensa().choose(rng).unwrap().to_string();
        
        (causante, circunstancia, cambio, gatillo, defensa)
    }
}

// ============================================================================
// CAPA 3: LA MÁSCARA
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mascara {
    pub comportamiento_publico: String,
    pub imagen_proyectada: String,
    pub frase_tipica: String,
    pub sentimiento_oculto: String,
    pub miedo_central: String,
    pub deseo_secreto: String,
    pub trigger_que_la_rompe: String,
    pub costo_de_mantenerla: String,
}

impl Mascara {
    pub fn generar(rng: &mut impl Rng, herida: &Herida) -> Self {
        let mascaras = vec![
            ("Confiado y carismático", "Alguien que tiene todo bajo control", 
             "No hay problema sin solución", "Soledad profunda",
             "Que descubran que no sabe lo que hace", "Ser amado sin condiciones"),
            ("Frío y calculador", "Alguien que no necesita a nadie",
             "Las emociones son debilidad", "Vergüenza constante",
             "Que no es suficiente", "Poder descansar de probar su valía"),
            ("Alegre y despreocupado", "Alguien a quien nada afecta",
             "¿Por qué preocuparse?", "Rabia contenida",
             "Que si la libera destruirá todo", "Que alguien entienda su dolor"),
            ("Servicial y abnegado", "Alguien indispensable",
             "Tus problemas son más importantes", "Vacío existencial",
             "Que nada de lo que hace importa", "Encontrar significado"),
            ("Cínico y sarcástico", "Alguien demasiado listo para esperanzas",
             "Ya lo veía venir", "Terror constante",
             "Que todo será arrebatado de nuevo", "Poder amar sin miedo"),
        ];
        
        let m = mascaras.choose(rng).unwrap();
        
        Self {
            comportamiento_publico: m.0.to_string(),
            imagen_proyectada: m.1.to_string(),
            frase_tipica: m.2.to_string(),
            sentimiento_oculto: m.3.to_string(),
            miedo_central: m.4.to_string(),
            deseo_secreto: m.5.to_string(),
            trigger_que_la_rompe: format!("Cuando {}", herida.gatillo_emocional.to_lowercase()),
            costo_de_mantenerla: Self::generar_costo(rng),
        }
    }
    
    fn generar_costo(rng: &mut impl Rng) -> String {
        let costos = [
            "Insomnio y pesadillas",
            "Estallidos de ira inexplicables",
            "Episodios de desconexión",
            "Relaciones que se deterioran",
            "Agotamiento emocional constante",
        ];
        costos.choose(rng).unwrap().to_string()
    }
}

// ============================================================================
// CAPA 4: DESEO VS NECESIDAD
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeseoNecesidad {
    pub deseo_consciente: String,
    pub motivacion_del_deseo: String,
    pub estrategia: String,
    pub necesidad_real: String,
    pub por_que_no_la_ve: String,
    pub conflicto: String,
    pub ironia: String,
}

impl DeseoNecesidad {
    pub fn generar(rng: &mut impl Rng, _herida: &Herida) -> Self {
        let combos = vec![
            ("Poder absoluto", "Nunca más será vulnerable", "Acumular recursos y aliados",
             "Perdonarse a sí mismo", "Cree que no merece perdón"),
            ("Venganza", "Cree que le dará paz", "Dedicar cada recurso a ese fin",
             "Dejar ir el pasado", "Cree que olvidar es traicionar"),
            ("Reconocimiento", "Probar que vale algo", "Hazañas cada vez más peligrosas",
             "Aceptar que ya es suficiente", "Ha construido su identidad en probarlo"),
            ("Proteger a alguien", "No repetir el fracaso pasado", "Control obsesivo del entorno",
             "Aceptar que no puede controlarlo todo", "Lo asocia con debilidad"),
            ("Un lugar donde pertenecer", "Llenar el vacío", "Moldearse a lo que otros quieren",
             "Conexión genuina", "Lo ve como peligroso"),
        ];
        
        let c = combos.choose(rng).unwrap();
        
        let conflicto = format!(
            "Persigue {} creyendo que le dará paz, pero necesita {}.",
            c.0.to_lowercase(), c.3.to_lowercase()
        );
        
        let ironia = format!(
            "Si obtuviera {} sin trabajar en {}, seguiría vacío.",
            c.0.to_lowercase(), c.3.to_lowercase()
        );
        
        Self {
            deseo_consciente: c.0.to_string(),
            motivacion_del_deseo: c.1.to_string(),
            estrategia: c.2.to_string(),
            necesidad_real: c.3.to_string(),
            por_que_no_la_ve: c.4.to_string(),
            conflicto,
            ironia,
        }
    }
}

// ============================================================================
// CAPA 5: LA SOMBRA
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sombra {
    pub rasgo_negado: String,
    pub como_se_filtra: Vec<String>,
    pub que_la_despierta: String,
    pub potencial_integrado: String,
    pub peligro_si_domina: String,
}

impl Sombra {
    pub fn generar(rng: &mut impl Rng) -> Self {
        let sombras = vec![
            (
                "La crueldad que es capaz de ejercer",
                vec!["Frialdad bajo presión", "Fantasías violentas", "Placer oculto ante el sufrimiento enemigo"],
                "Cuando alguien que ama es amenazado",
                "Hacer lo necesario sin romperse",
                "Convertirse en lo que jura combatir"
            ),
            (
                "El egoísmo tras el altruismo",
                vec!["Resentimiento cuando no aprecian su ayuda", "Llevar cuenta de favores", "Hacer sentir culpables"],
                "Cuando su sacrificio es ignorado",
                "Ayudar sin expectativas",
                "Manipulación disfrazada de bondad"
            ),
            (
                "El miedo que lo paraliza",
                vec!["Agresividad cuando se siente acorralado", "Evitar situaciones incontrolables", "Despreciar a los miedosos"],
                "Situaciones de impotencia pasada",
                "Valor que reconoce el miedo pero actúa",
                "Parálisis o reacciones desproporcionadas"
            ),
            (
                "La envidia que corroe",
                vec!["Críticas sutiles a quienes tienen lo que quiere", "Satisfacción ante fracasos ajenos"],
                "El éxito de otros donde ha fallado",
                "Ambición sana sin destruir",
                "Sabotaje para sentirse mejor"
            ),
            (
                "La necesidad de control",
                vec!["Manipulación sutil", "Ansiedad ante lo impredecible", "Incapacidad de delegar"],
                "Cualquier situación impredecible",
                "Liderazgo que empodera",
                "Tiranía disfrazada de responsabilidad"
            ),
        ];
        
        let s = sombras.choose(rng).unwrap();
        
        Self {
            rasgo_negado: s.0.to_string(),
            como_se_filtra: s.1.iter().map(|x| x.to_string()).collect(),
            que_la_despierta: s.2.to_string(),
            potencial_integrado: s.3.to_string(),
            peligro_si_domina: s.4.to_string(),
        }
    }
}

// ============================================================================
// CAPA 6: VÍNCULOS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapaVinculos {
    pub patron: PatronVincular,
    pub estilo_apego: EstiloApego,
    pub rol_en_grupos: String,
    pub como_expresa_afecto: String,
    pub que_busca_en_otros: String,
    pub que_ofrece: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatronVincular {
    DistanciaPorDefecto,
    IntensidadInicial,
    PruebasConstantes,
    DarParaRecibir,
    SacrificioTotal,
    IndependenciaFeroz,
    CuidadorCompulsivo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EstiloApego {
    Seguro,
    Ansioso,
    Evitativo,
    Desorganizado,
}

impl CapaVinculos {
    pub fn generar(rng: &mut impl Rng, herida: &Herida) -> Self {
        let patron = match herida.tipo {
            TipoHerida::Abandono => *[PatronVincular::DistanciaPorDefecto, PatronVincular::PruebasConstantes].choose(rng).unwrap(),
            TipoHerida::Traicion => PatronVincular::PruebasConstantes,
            TipoHerida::Negligencia => *[PatronVincular::IndependenciaFeroz, PatronVincular::CuidadorCompulsivo].choose(rng).unwrap(),
            _ => *[PatronVincular::DistanciaPorDefecto, PatronVincular::DarParaRecibir].choose(rng).unwrap(),
        };
        
        let estilo = match herida.tipo {
            TipoHerida::Abandono => EstiloApego::Ansioso,
            TipoHerida::Traicion => EstiloApego::Evitativo,
            TipoHerida::Violencia => EstiloApego::Desorganizado,
            _ => *[EstiloApego::Ansioso, EstiloApego::Evitativo].choose(rng).unwrap(),
        };
        
        Self {
            patron,
            estilo_apego: estilo,
            rol_en_grupos: Self::generar_rol(&patron),
            como_expresa_afecto: Self::generar_expresion(&patron),
            que_busca_en_otros: Self::generar_busqueda(rng, herida),
            que_ofrece: Self::generar_oferta(&patron),
        }
    }
    
    fn generar_rol(patron: &PatronVincular) -> String {
        match patron {
            PatronVincular::CuidadorCompulsivo => "El que mantiene al grupo unido",
            PatronVincular::IndependenciaFeroz => "El lobo solitario que ayuda a distancia",
            PatronVincular::IntensidadInicial => "El corazón apasionado",
            PatronVincular::DistanciaPorDefecto => "El observador que interviene cuando importa",
            PatronVincular::PruebasConstantes => "El escéptico eterno",
            PatronVincular::DarParaRecibir => "El negociador",
            PatronVincular::SacrificioTotal => "El mártir",
        }.to_string()
    }
    
    fn generar_expresion(patron: &PatronVincular) -> String {
        match patron {
            PatronVincular::CuidadorCompulsivo => "Actos de servicio abrumadores",
            PatronVincular::DistanciaPorDefecto => "Gestos pequeños que pasan desapercibidos",
            PatronVincular::IntensidadInicial => "Declaraciones apasionadas",
            _ => "Acciones más que palabras",
        }.to_string()
    }
    
    fn generar_busqueda(_rng: &mut impl Rng, herida: &Herida) -> String {
        match herida.tipo {
            TipoHerida::Abandono => "Alguien que nunca se irá",
            TipoHerida::Traicion => "Lealtad incuestionable",
            TipoHerida::Humillacion => "Respeto incondicional",
            TipoHerida::Rechazo => "Aceptación total",
            _ => "Alguien que entienda sin juzgar",
        }.to_string()
    }
    
    fn generar_oferta(patron: &PatronVincular) -> String {
        match patron {
            PatronVincular::CuidadorCompulsivo => "Cuidado incondicional",
            PatronVincular::IndependenciaFeroz => "Ayuda competente sin ataduras",
            PatronVincular::IntensidadInicial => "Pasión y devoción total",
            PatronVincular::DistanciaPorDefecto => "Lealtad probada con el tiempo",
            PatronVincular::PruebasConstantes => "Verdad brutal pero honesta",
            PatronVincular::DarParaRecibir => "Intercambio justo",
            PatronVincular::SacrificioTotal => "Todo lo que tiene",
        }.to_string()
    }
}

// ============================================================================
// CAPA 7: LA MENTIRA
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mentira {
    pub la_mentira: String,
    pub como_nacio: String,
    pub como_distorsiona: String,
    pub decisiones_que_causa: String,
    pub verdad_necesaria: String,
    pub catalizador_potencial: String,
    pub costo_de_la_verdad: String,
}

impl Mentira {
    pub fn generar(rng: &mut impl Rng, _herida: &Herida) -> Self {
        let mentiras = vec![
            (
                "Estoy solo porque todos me traicionarán",
                "Una traición que generalizó al mundo",
                "Ve amenazas donde hay indiferencia",
                "Nunca confía, siempre tiene plan de escape",
                "La vulnerabilidad es el precio de la conexión",
                "Alguien lo elija sobre su propio beneficio",
                "Arriesgar ser herido de nuevo"
            ),
            (
                "Si no soy fuerte, no valgo nada",
                "Su debilidad tuvo consecuencias terribles",
                "Ve cualquier limitación como fracaso moral",
                "Nunca pide ayuda",
                "La fortaleza incluye aceptar límites",
                "Fallar y aun así ser aceptado",
                "Enfrentar la vergüenza evitada"
            ),
            (
                "No merezco ser feliz",
                "Culpa por un error magnificado",
                "Ve la felicidad propia como algo robado",
                "Sabotea cualquier posibilidad de felicidad",
                "El perdón es posible",
                "Perdonar a alguien que le hizo algo similar",
                "Soltar la identidad construida sobre culpa"
            ),
            (
                "El mundo es injusto, debo serlo también",
                "Injusticias que sufrió o presenció",
                "Ve la bondad como ingenuidad",
                "Justifica acciones cuestionables",
                "Puede elegir ser diferente al mundo que lo hirió",
                "Ver el impacto de sus acciones en un inocente",
                "Admitir que ha causado daño"
            ),
            (
                "Si sacrifico todo, seré suficiente",
                "Nunca fue suficiente para alguien importante",
                "Confunde su valor con su utilidad",
                "Se destruye probando su valor",
                "Ya es suficiente sin probarlo",
                "Ser amado sin hacer nada extraordinario",
                "Enfrentar el vacío de no saber quién es sin servir"
            ),
        ];
        
        let m = mentiras.choose(rng).unwrap();
        
        Self {
            la_mentira: m.0.to_string(),
            como_nacio: m.1.to_string(),
            como_distorsiona: m.2.to_string(),
            decisiones_que_causa: m.3.to_string(),
            verdad_necesaria: m.4.to_string(),
            catalizador_potencial: m.5.to_string(),
            costo_de_la_verdad: m.6.to_string(),
        }
    }
}

/// Las 7 Capas completas del Alma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SietCapas {
    pub arquetipo: Arquetipo,
    pub herida: Herida,
    pub mascara: Mascara,
    pub deseo_necesidad: DeseoNecesidad,
    pub sombra: Sombra,
    pub vinculos: CapaVinculos,
    pub mentira: Mentira,
}

impl SietCapas {
    pub fn generar(rng: &mut impl Rng, mundo: &super::Mundo, lang: &Language) -> Self {
        let arquetipo = Arquetipo::generar(rng, lang);
        let herida = Herida::generar(rng, mundo, lang);
        // TODO: Adaptar el resto de capas
        let mut mascara = Mascara::generar(rng, &herida);
        let mut deseo_necesidad = DeseoNecesidad::generar(rng, &herida);
        let mut sombra = Sombra::generar(rng);
        let vinculos = CapaVinculos::generar(rng, &herida);
        let mut mentira = Mentira::generar(rng, &herida);
        
        // Adaptación de idioma para las capas restantes
        if *lang != Language::Espanol {
            let lang_code = match lang { Language::English => "en", Language::Japanese => "jp", _ => "es" };
            let seed = rng.next_u64();
            
            // Mascara
            let img = mascara.imagen_proyectada.clone();
            mascara.imagen_proyectada = super::adapter::adapt_text(&img, lang_code, seed.wrapping_add(10));
            let frase = mascara.frase_tipica.clone();
            mascara.frase_tipica = super::adapter::adapt_text(&frase, lang_code, seed.wrapping_add(11));
            
            // Deseo/Necesidad
            let deseo = deseo_necesidad.deseo_consciente.clone();
            deseo_necesidad.deseo_consciente = super::adapter::adapt_text(&deseo, lang_code, seed.wrapping_add(20));
            let necesidad = deseo_necesidad.necesidad_real.clone();
            deseo_necesidad.necesidad_real = super::adapter::adapt_text(&necesidad, lang_code, seed.wrapping_add(21));
            
            // Sombra
            let rasgo = sombra.rasgo_negado.clone();
            sombra.rasgo_negado = super::adapter::adapt_text(&rasgo, lang_code, seed.wrapping_add(30));
            let peligro = sombra.peligro_si_domina.clone();
            sombra.peligro_si_domina = super::adapter::adapt_text(&peligro, lang_code, seed.wrapping_add(31));
            
            // Mentira
            let men = mentira.la_mentira.clone();
            mentira.la_mentira = super::adapter::adapt_text(&men, lang_code, seed.wrapping_add(40));
            let ver = mentira.verdad_necesaria.clone();
            mentira.verdad_necesaria = super::adapter::adapt_text(&ver, lang_code, seed.wrapping_add(41));
        }
        
        Self {
            arquetipo,
            herida,
            mascara,
            deseo_necesidad,
            sombra,
            vinculos,
            mentira,
        }
    }
}
