//! Exportadores - Múltiples formatos de salida

mod json;
mod markdown;

pub use json::*;
pub use markdown::*;

use crate::core::Alma;
use crate::constelacion::Constelacion;

/// Trait para exportadores
pub trait Exportador {
    fn exportar_alma(&self, alma: &Alma) -> String;
    fn exportar_constelacion(&self, constelacion: &Constelacion) -> String;
}
