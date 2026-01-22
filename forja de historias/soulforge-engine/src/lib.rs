use wasm_bindgen::prelude::*;

mod soul_parser;
mod tension_engine;
mod destiny_director;
mod conflict_forge;
mod world_seed;

pub use soul_parser::*;
pub use tension_engine::*;
pub use destiny_director::*;
pub use conflict_forge::*;
pub use world_seed::*;

/// Inicialización del Motor SoulForge
#[wasm_bindgen(start)]
pub fn ignite_forge() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"⚔️ SoulForge Engine Ignited".into());
}

/// API Principal - Forja completa desde HTML
#[wasm_bindgen]
pub fn forge_soul_from_html(html_content: &str) -> JsValue {
    let soul = soul_parser::parse_living_sheet(html_content);
    serde_wasm_bindgen::to_value(&soul).unwrap_or(JsValue::NULL)
}

/// Análisis de tensión en tiempo real
#[wasm_bindgen]
pub fn analyze_narrative_tension(soul_json: &str, narrative_text: &str) -> JsValue {
    let result = tension_engine::calculate_tension(soul_json, narrative_text);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Auditoría de coherencia
#[wasm_bindgen]
pub fn audit_coherence(soul_json: &str, new_content: &str) -> JsValue {
    let violations = destiny_director::check_violations(soul_json, new_content);
    serde_wasm_bindgen::to_value(&violations).unwrap_or(JsValue::NULL)
}

/// Generación de puntos de inflexión
#[wasm_bindgen]
pub fn generate_inflection_points(soul_json: &str) -> JsValue {
    let points = conflict_forge::create_inflection_points(soul_json);
    serde_wasm_bindgen::to_value(&points).unwrap_or(JsValue::NULL)
}
