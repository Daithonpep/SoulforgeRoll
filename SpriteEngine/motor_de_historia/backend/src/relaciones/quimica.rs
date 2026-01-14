//! Sistema de Química Relacional
//! 
//! Calcula compatibilidad, conflicto y potencial narrativo entre dos almas
//! basándose en sus psicologías profundas.

use serde::{Deserialize, Serialize};
use crate::core::{Alma, TipoArquetipo, TipoHerida, EstiloApego};



/// La química entre dos personajes - EMERGENTE de sus psicologías
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quimica {
    /// Qué tan complementarias son sus heridas (sanadores mutuos potenciales)
    pub compatibilidad_heridas: f32,
    
    /// Atracción o repulsión entre arquetipos
    pub atraccion_arquetipos: f32,
    
    /// Conflicto generado por sus sombras proyectadas
    pub conflicto_sombras: f32,
    
    /// Tensión entre lo que ambos desean/necesitan
    pub deseo_necesidad_tension: f32,
    
    /// Qué tanto se complementan (uno tiene lo que el otro necesita)
    pub complementariedad: f32,
    
    /// Qué tanto se ven reflejados (espejo)
    pub espejo: f32,
    
    /// Potencial para catalizar el crecimiento mutuo
    pub potencial_catalitico: f32,
    
    /// Compatibilidad de estilos de apego
    pub compatibilidad_apego: f32,
    
    /// Razones narrativas del vínculo
    pub razones: Vec<String>,
}

impl Quimica {
    /// Calcula la química entre dos almas
    pub fn calcular(alma_a: &Alma, alma_b: &Alma) -> Self {
        let compatibilidad_heridas = Self::calc_compatibilidad_heridas(alma_a, alma_b);
        let atraccion_arquetipos = Self::calc_atraccion_arquetipos(alma_a, alma_b);
        let conflicto_sombras = Self::calc_conflicto_sombras(alma_a, alma_b);
        let deseo_necesidad_tension = Self::calc_tension_deseo_necesidad(alma_a, alma_b);
        let complementariedad = Self::calc_complementariedad(alma_a, alma_b);
        let espejo = Self::calc_espejo(alma_a, alma_b);
        let compatibilidad_apego = Self::calc_compatibilidad_apego(alma_a, alma_b);
        
        let potencial_catalitico = Self::calc_potencial_catalitico(
            compatibilidad_heridas, 
            complementariedad, 
            conflicto_sombras
        );
        
        let razones = Self::generar_razones(
            alma_a, alma_b,
            compatibilidad_heridas, atraccion_arquetipos,
            conflicto_sombras, espejo
        );
        
        Self {
            compatibilidad_heridas,
            atraccion_arquetipos,
            conflicto_sombras,
            deseo_necesidad_tension,
            complementariedad,
            espejo,
            potencial_catalitico,
            compatibilidad_apego,
            razones,
        }
    }
    
    /// Intensidad total de la relación (no necesariamente positiva)
    pub fn intensidad_total(&self) -> f32 {
        let positiva = self.compatibilidad_heridas + self.atraccion_arquetipos + 
                       self.complementariedad + self.potencial_catalitico;
        let negativa = self.conflicto_sombras + self.deseo_necesidad_tension;
        
        (positiva + negativa) / 6.0
    }
    
    /// Balance entre atracción y conflicto
    pub fn balance(&self) -> f32 {
        let atraccion = self.compatibilidad_heridas + self.atraccion_arquetipos + self.complementariedad;
        let conflicto = self.conflicto_sombras + self.deseo_necesidad_tension;
        
        (atraccion / 3.0) - (conflicto / 2.0)
    }
    
    // ==================== CÁLCULOS INTERNOS ====================
    
    /// Heridas complementarias pueden sanarse mutuamente
    fn calc_compatibilidad_heridas(a: &Alma, b: &Alma) -> f32 {
        let tipo_a = &a.capas.herida.tipo;
        let tipo_b = &b.capas.herida.tipo;
        
        // Algunas heridas se complementan (el cuidador sana al abandonado)
        let complementarias = vec![
            (TipoHerida::Abandono, TipoHerida::Negligencia),
            (TipoHerida::Traicion, TipoHerida::Culpa),
            (TipoHerida::Humillacion, TipoHerida::Rechazo),
            (TipoHerida::Impotencia, TipoHerida::Injusticia),
        ];
        
        for (h1, h2) in &complementarias {
            if (tipo_a == h1 && tipo_b == h2) || (tipo_a == h2 && tipo_b == h1) {
                return 0.8;
            }
        }
        
        // Misma herida = alto espejo pero no necesariamente sanación
        if tipo_a == tipo_b {
            return 0.5;
        }
        
        0.3
    }
    
    /// Atracción/repulsión entre arquetipos
    fn calc_atraccion_arquetipos(a: &Alma, b: &Alma) -> f32 {
        let arq_a = &a.capas.arquetipo.tipo;
        let arq_b = &b.capas.arquetipo.tipo;
        
        // Arquetipos que se atraen naturalmente
        let atraccion = vec![
            (TipoArquetipo::Guerrero, TipoArquetipo::Cuidador),
            (TipoArquetipo::Buscador, TipoArquetipo::Sabio),
            (TipoArquetipo::Creador, TipoArquetipo::Destructor),
            (TipoArquetipo::Gobernante, TipoArquetipo::Bufon),
            (TipoArquetipo::Amante, TipoArquetipo::Huerfano),
            (TipoArquetipo::Mago, TipoArquetipo::Inocente),
        ];
        
        for (a1, a2) in &atraccion {
            if (arq_a == a1 && arq_b == a2) || (arq_a == a2 && arq_b == a1) {
                return 0.8;
            }
        }
        
        // Mismo arquetipo = competencia
        if arq_a == arq_b {
            return 0.3;
        }
        
        0.5
    }
    
    /// Las sombras proyectadas causan conflicto
    fn calc_conflicto_sombras(a: &Alma, b: &Alma) -> f32 {
        // Si la sombra de uno es la luz del otro = conflicto alto
        // (ej: uno niega su crueldad, el otro la exhibe abiertamente)
        
        let tono_a = a.tono_moral.valor_numerico();
        let tono_b = b.tono_moral.valor_numerico();
        let diferencia_tono = (tono_a - tono_b).abs() as f32;
        
        // Cuanto más opuestos moralmente, más conflicto de sombras
        let por_tono = diferencia_tono / 4.0;
        
        // Si ambos tienen alta sombra, más conflicto
        let conflicto_base = 0.3;
        
        f32::min(por_tono + conflicto_base, 1.0)
    }
    
    /// Tensión cuando ambos desean lo mismo o sus necesidades chocan
    fn calc_tension_deseo_necesidad(a: &Alma, b: &Alma) -> f32 {
        // Si desean lo mismo = competencia
        if a.capas.deseo_necesidad.deseo_consciente == b.capas.deseo_necesidad.deseo_consciente {
            return 0.8;
        }
        
        // Si lo que uno desea es lo que el otro necesita = tensión interesante
        if a.capas.deseo_necesidad.deseo_consciente == b.capas.deseo_necesidad.necesidad_real {
            return 0.6;
        }
        
        0.3
    }
    
    /// Qué tanto se complementan
    fn calc_complementariedad(a: &Alma, b: &Alma) -> f32 {
        let mut score = 0.0;
        
        // Estilos de apego complementarios
        match (&a.capas.vinculos.estilo_apego, &b.capas.vinculos.estilo_apego) {
            (EstiloApego::Ansioso, EstiloApego::Seguro) |
            (EstiloApego::Seguro, EstiloApego::Ansioso) |
            (EstiloApego::Evitativo, EstiloApego::Seguro) |
            (EstiloApego::Seguro, EstiloApego::Evitativo) => {
                score += 0.4;
            },
            (EstiloApego::Ansioso, EstiloApego::Evitativo) |
            (EstiloApego::Evitativo, EstiloApego::Ansioso) => {
                // Trampa clásica pero intensa
                score += 0.2;
            },
            _ => score += 0.3,
        }
        
        // Arquetipos complementarios ya calculados
        score += Self::calc_atraccion_arquetipos(a, b) * 0.3;
        
        f32::min(score, 1.0)
    }
    
    /// Qué tanto son espejo uno del otro
    fn calc_espejo(a: &Alma, b: &Alma) -> f32 {
        let mut score = 0.0;
        
        // Misma herida = alto espejo
        if a.capas.herida.tipo == b.capas.herida.tipo {
            score += 0.4;
        }
        
        // Mismo arquetipo = espejo
        if a.capas.arquetipo.tipo == b.capas.arquetipo.tipo {
            score += 0.3;
        }
        
        // Misma mentira = espejo profundo
        if a.capas.mentira.la_mentira == b.capas.mentira.la_mentira {
            score += 0.3;
        }
        
        f32::min(score, 1.0)
    }
    
    /// Compatibilidad de estilos de apego
    fn calc_compatibilidad_apego(a: &Alma, b: &Alma) -> f32 {
        match (&a.capas.vinculos.estilo_apego, &b.capas.vinculos.estilo_apego) {
            (EstiloApego::Seguro, EstiloApego::Seguro) => 0.9,
            (EstiloApego::Seguro, _) | (_, EstiloApego::Seguro) => 0.7,
            (EstiloApego::Ansioso, EstiloApego::Evitativo) |
            (EstiloApego::Evitativo, EstiloApego::Ansioso) => 0.3, // Atrae pero destruye
            (EstiloApego::Desorganizado, _) | (_, EstiloApego::Desorganizado) => 0.2,
            _ => 0.5,
        }
    }
    
    /// Potencial para que se ayuden a crecer
    fn calc_potencial_catalitico(
        compatibilidad_heridas: f32,
        complementariedad: f32,
        conflicto_sombras: f32
    ) -> f32 {
        // Alto potencial = se complementan + algo de conflicto (que los reta)
        let base = (compatibilidad_heridas + complementariedad) / 2.0;
        let boost = if conflicto_sombras > 0.3 && conflicto_sombras < 0.7 {
            0.2 // Conflicto moderado cataliza crecimiento
        } else {
            0.0
        };
        
        f32::min(base + boost, 1.0)
    }
    
    /// Genera razones narrativas del vínculo
    fn generar_razones(
        a: &Alma, b: &Alma,
        compat_heridas: f32, atrac_arq: f32,
        conflicto: f32, espejo: f32
    ) -> Vec<String> {
        let mut razones = Vec::new();
        
        if compat_heridas > 0.6 {
            razones.push(format!(
                "{} entiende el dolor de {} de una forma que pocos pueden.",
                a.identidad.nombre, b.identidad.nombre
            ));
        }
        
        if atrac_arq > 0.6 {
            razones.push(format!(
                "Sus naturalezas se complementan - {} es todo lo que {} no es.",
                b.identidad.nombre, a.identidad.nombre
            ));
        }
        
        if conflicto > 0.6 {
            razones.push(format!(
                "{} representa todo lo que {} se niega a ver en sí mismo.",
                b.identidad.nombre, a.identidad.nombre
            ));
        }
        
        if espejo > 0.6 {
            razones.push(format!(
                "Mirarse es como verse en un espejo distorsionado.",
            ));
        }
        
        if razones.is_empty() {
            razones.push("Un vínculo que el tiempo definirá.".to_string());
        }
        
        razones
    }
}
