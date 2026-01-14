//! Exportador JSON

use serde_json;
use super::Exportador;
use crate::core::Alma;
use crate::constelacion::Constelacion;

pub struct JsonExportador {
    pretty: bool,
}

impl JsonExportador {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl Exportador for JsonExportador {
    fn exportar_alma(&self, alma: &Alma) -> String {
        if self.pretty {
            serde_json::to_string_pretty(alma).unwrap_or_default()
        } else {
            serde_json::to_string(alma).unwrap_or_default()
        }
    }
    
    fn exportar_constelacion(&self, constelacion: &Constelacion) -> String {
        if self.pretty {
            serde_json::to_string_pretty(constelacion).unwrap_or_default()
        } else {
            serde_json::to_string(constelacion).unwrap_or_default()
        }
    }
}
