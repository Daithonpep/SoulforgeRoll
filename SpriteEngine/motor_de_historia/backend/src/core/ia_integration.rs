use std::process::Command;
use serde_json::Value;
use std::env;
use std::path::Path;

/// Intenta enriquecer el personaje usando la integración de IA (Python + Groq)
pub fn enriquecer_personaje(personaje_json: serde_json::Value) -> serde_json::Value {
    // Verificar si existe la API key
    if env::var("GROQ_API_KEY").is_err() {
        println!("[RUST] No GROQ_API_KEY found, skipping enhancement.");
        return personaje_json;
    }

    // Ruta al script (asumiendo estructura de docker: /app/groq_integration/groq_enhancer.py)
    // O localmente: ../groq_integration/groq_enhancer.py
    
    let script_path = if Path::new("/app/groq_integration/groq_enhancer.py").exists() {
        "/app/groq_integration/groq_enhancer.py"
    } else if Path::new("./groq_integration/groq_enhancer.py").exists() {
        "./groq_integration/groq_enhancer.py"
    } else if Path::new("../groq_integration/groq_enhancer.py").exists() {
        "../groq_integration/groq_enhancer.py"
    } else {
        println!("[RUST] Python script not found, skipping enhancement.");
        return personaje_json;
    };

    // Crear script temporal wrapper para pasar el JSON
    let json_str = personaje_json.to_string();
    
    // Ejecutar python pasando el JSON como argumento o stdin
    // Para simplificar, crearemos un pequeño script inline en python que importa el módulo
    let python_code = format!(r#"
import sys
import json
import os

# Add directory to path
sys.path.append(os.path.dirname('{}'))

try:
    from groq_enhancer import safe_enhance
    
    # Read input from stdin
    data = json.loads(sys.stdin.read())
    
    # Enhance
    enhanced = safe_enhance(data)
    
    # Print result to stdout
    print(json.dumps(enhanced))
except Exception as e:
    # On error, print original data to stdout so we don't break the pipeline
    import traceback
    traceback.print_exc(file=sys.stderr)
    print(json.dumps(data))
"#, script_path);

    // Determinar comando python (python3 en linux/docker, python en windows)
    let python_cmd = if cfg!(target_os = "windows") { "python" } else { "python3" };

    println!("[RUST] Calling Groq Enhancer...");
    
    let mut child = Command::new(python_cmd)
        .arg("-c")
        .arg(&python_code)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit()) // Para ver errores en log
        .spawn();

    match child {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                if let Err(e) = stdin.write_all(json_str.as_bytes()) {
                    println!("[RUST] Error writing to python stdin: {}", e);
                    return personaje_json;
                }
            }

            let output = child.wait_with_output();
            
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let result_str = String::from_utf8_lossy(&output.stdout);
                        if let Ok(json_val) = serde_json::from_str::<Value>(&result_str) {
                            println!("[RUST] Enhancement successful!");
                            return json_val;
                        } else {
                            println!("[RUST] Failed to parse Python output as JSON");
                        }
                    } else {
                        println!("[RUST] Python script returned error code");
                    }
                }
                Err(e) => println!("[RUST] Failed to wait for python process: {}", e),
            }
        }
        Err(e) => println!("[RUST] Failed to spawn python process: {}", e),
    }

    // Si algo falló, devolver el original
    personaje_json
}
