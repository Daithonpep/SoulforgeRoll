//! Tipos de vínculos y relaciones específicas

use serde::{Deserialize, Serialize};


/// Historia compartida entre personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoriaCompartida {
    pub evento: String,
    pub impacto_en_a: String,
    pub impacto_en_b: String,
    pub secretos: Vec<String>,
}

/// Tensión activa entre personajes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionActiva {
    pub descripcion: String,
    pub origen: String,
    pub como_podria_estallar: String,
    pub como_podria_resolverse: String,
}

/// Dinámica específica de poder en la relación
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DinamicaPoder {
    Equilibrada,
    ADomina,
    BDomina,
    Fluctuante,
    Tóxica,
}

/// Estado actual de la relación
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EstadoRelacion {
    Floreciente,
    Estable,
    Tensa,
    Deteriorando,
    Rota,
    Reconciliando,
}
