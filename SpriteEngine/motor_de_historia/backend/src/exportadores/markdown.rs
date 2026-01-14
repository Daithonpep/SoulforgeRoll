//! Exportador Markdown

use super::Exportador;
use crate::core::Alma;
use crate::constelacion::Constelacion;

pub struct MarkdownExportador;

impl MarkdownExportador {
    pub fn new() -> Self {
        Self
    }
}

impl Exportador for MarkdownExportador {
    fn exportar_alma(&self, alma: &Alma) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("# {}\n\n", alma.identidad.nombre));
        
        if let Some(ref titulo) = alma.identidad.titulo {
            md.push_str(&format!("*\"{}\"*\n\n", titulo));
        }
        
        md.push_str(&format!("**Rol:** {:?} | **Tono:** {:?} | **Mundo:** {:?}\n\n", 
            alma.rol, alma.tono_moral, alma.mundo));
        
        md.push_str("## Primera Impresión\n\n");
        md.push_str(&format!("- {}\n", alma.identidad.rasgo_distintivo));
        md.push_str(&format!("- Viste: {}\n", alma.identidad.vestimenta));
        md.push_str(&format!("- Voz: {}\n\n", alma.identidad.voz));
        
        md.push_str("## La Máscara\n\n");
        md.push_str(&format!("**Comportamiento:** {}\n\n", alma.capas.mascara.comportamiento_publico));
        md.push_str(&format!("**Quiere ser visto como:** {}\n\n", alma.capas.mascara.imagen_proyectada));
        md.push_str(&format!("> \"{}\"\n\n", alma.capas.mascara.frase_tipica));
        
        md.push_str("## La Herida\n\n");
        md.push_str(&format!("{} {}.\n\n", alma.capas.herida.causante, alma.capas.herida.circunstancia));
        md.push_str(&format!("**Consecuencia:** {}\n\n", alma.capas.herida.como_lo_cambio));
        
        md.push_str("## El Conflicto Central\n\n");
        md.push_str(&format!("**Desea:** {}\n\n", alma.capas.deseo_necesidad.deseo_consciente));
        md.push_str(&format!("**Necesita:** {}\n\n", alma.capas.deseo_necesidad.necesidad_real));
        md.push_str(&format!("*{}*\n\n", alma.capas.deseo_necesidad.conflicto));
        
        md.push_str("## La Mentira\n\n");
        md.push_str(&format!("> \"{}\"\n\n", alma.capas.mentira.la_mentira));
        md.push_str(&format!("**Verdad que necesita:** {}\n\n", alma.capas.mentira.verdad_necesaria));
        
        md.push_str("## Arco Narrativo\n\n");
        md.push_str(&format!("**Tipo:** {:?}\n\n", alma.arco.tipo));
        md.push_str(&format!("- **Inicio:** {}\n", alma.arco.estado_inicial));
        md.push_str(&format!("- **Quiebre:** {}\n", alma.arco.punto_de_quiebre));
        md.push_str(&format!("- **Si triunfa:** {}\n", alma.arco.resolucion_positiva));
        md.push_str(&format!("- **Si falla:** {}\n\n", alma.arco.resolucion_tragica));
        
        md.push_str("## Ganchos Narrativos\n\n");
        for g in &alma.ganchos_narrativos {
            md.push_str(&format!("- {}\n", g));
        }
        
        md.push_str(&format!("\n---\n*Semilla: {}*\n", alma.semilla));
        
        md
    }
    
    fn exportar_constelacion(&self, constelacion: &Constelacion) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("# Constelación: {}\n\n", constelacion.nombre));
        md.push_str(&format!("**Mundo:** {:?}\n\n", constelacion.mundo));
        
        md.push_str("## Resumen\n\n");
        md.push_str(&constelacion.resumen_narrativo);
        md.push_str("\n\n");
        
        md.push_str("## Tensiones Centrales\n\n");
        for t in &constelacion.tensiones_centrales {
            md.push_str(&format!("### {:?}\n\n", t.tipo));
            md.push_str(&format!("{}\n\n", t.descripcion));
            md.push_str(&format!("**Stakes:** {}\n\n", t.stakes));
        }
        
        md.push_str("---\n\n");
        md.push_str("## Personajes\n\n");
        
        for alma in &constelacion.almas {
            md.push_str(&self.exportar_alma(alma));
            md.push_str("\n\n---\n\n");
        }
        
        md
    }
}
