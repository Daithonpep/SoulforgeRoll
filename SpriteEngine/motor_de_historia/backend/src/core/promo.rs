use std::sync::Mutex;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::fs::{self, File};
use std::io::{Read, Write};

// Almacenamiento en memoria protegido por Mutex, persistido en archivo JSON
struct PromoStore {
    tokens_validos: HashSet<String>,
    tokens_usados: HashSet<String>,
}

static STORE: Lazy<Mutex<PromoStore>> = Lazy::new(|| {
    let mut store = PromoStore {
        tokens_validos: HashSet::new(),
        tokens_usados: HashSet::new(),
    };

    // Intentar cargar estado previo
    if let Ok(data) = fs::read_to_string("promo_state.json") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(arr) = json["validos"].as_array() {
                for v in arr { store.tokens_validos.insert(v.as_str().unwrap().to_string()); }
            }
            if let Some(arr) = json["usados"].as_array() {
                for v in arr { store.tokens_usados.insert(v.as_str().unwrap().to_string()); }
            }
            return Mutex::new(store);
        }
    }

    // Inicialización Default (5 pases + Master)
    store.tokens_validos.insert("PASE-VIP-ALPHA-01".into());
    store.tokens_validos.insert("PASE-VIP-BETA-02".into());
    store.tokens_validos.insert("PASE-VIP-GAMMA-03".into());
    store.tokens_validos.insert("PASE-VIP-DELTA-04".into());
    store.tokens_validos.insert("PASE-VIP-EPSILON-05".into());
    store.tokens_validos.insert("PASE-VIP-ZETA-06".into());
    store.tokens_validos.insert("PASE-VIP-ETA-07".into());
    store.tokens_validos.insert("PASE-VIP-THETA-08".into());
    
    // Master Key (nunca expira, pero la guardamos para validación)
    // No la metemos en validos para que no se borre al usar, la hardcodeamos en lógica.
    
    Mutex::new(store)
});

pub fn validar_acceso(token: &str) -> bool {
    let token_norm = token.trim();
    
    // 1. Master Key (Para el dueño del video)
    if token_norm == "MASTER-GENESIS-KEY-2026" {
        return true;
    }

    let mut store = STORE.lock().unwrap();

    // 2. Verificar si es válido
    if store.tokens_validos.contains(token_norm) {
        // CONSUMIR TOKEN (Un solo uso)
        store.tokens_validos.remove(token_norm);
        store.tokens_usados.insert(token_norm.to_string());
        
        // Guardar estado (Persistencia básica)
        guardar_estado(&store);
        
        return true;
    }

    false
}

fn guardar_estado(store: &PromoStore) {
    let json = serde_json::json!({
        "validos": store.tokens_validos,
        "usados": store.tokens_usados
    });
    let _ = fs::write("promo_state.json", json.to_string());
}

pub fn listar_tokens_disponibles() -> Vec<String> {
    let store = STORE.lock().unwrap();
    store.tokens_validos.iter().cloned().collect()
}
