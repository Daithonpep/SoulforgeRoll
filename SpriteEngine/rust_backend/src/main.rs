use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use image::{RgbaImage, Rgba, GenericImage, Pixel};
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Deserialize)]
struct SpriteRequest {
    prompt: String,
    style: SpriteStyle,
    size: u32,              // 16, 32, 64, 128
    palette_limit: u8,      // 4, 8, 16, 32 colores
    animation_frames: u8,   // 1 = estático, 4-12 = animado
    layers: Vec<LayerType>, // ["base", "shadow", "highlight", "outline"]
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SpriteStyle {
    Retro8Bit,
    Retro16Bit,
    Modern,
    Isometric,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum LayerType {
    Base,
    Shadow,
    Highlight,
    Outline,
}

#[derive(Serialize)]
struct SpriteResponse {
    sprite_sheet: String,      // Base64 PNG
    layers: HashMap<String, String>, // Cada capa separada
    metadata: SpriteMetadata,
}

#[derive(Serialize)]
struct SpriteMetadata {
    frame_width: u32,
    frame_height: u32,
    frame_count: u8,
    palette: Vec<String>,  // Colores hex usados
    animation_fps: u8,
}

// Motor de post-procesamiento ultra rápido en Rust
struct PixelProcessor;

impl PixelProcessor {
    /// Reduce paleta de colores manteniendo calidad visual
    fn reduce_palette(image: &mut RgbaImage, max_colors: u8) -> Vec<Rgba<u8>> {
        // Algoritmo de quantización de color optimizado
        let mut color_counts: HashMap<[u8; 4], u32> = HashMap::new();
        
        // Contar frecuencia de colores
        for pixel in image.pixels() {
            *color_counts.entry(pixel.0).or_insert(0) += 1;
        }
        
        // Median cut algorithm para seleccionar paleta óptima
        let palette = Self::median_cut(&color_counts, max_colors as usize);
        
        // Remapear imagen a paleta reducida
        for pixel in image.pixels_mut() {
            let nearest = Self::find_nearest_color(&palette, pixel);
            *pixel = nearest;
        }
        
        palette
    }
    
    fn median_cut(colors: &HashMap<[u8; 4], u32>, target: usize) -> Vec<Rgba<u8>> {
        // Implementación del algoritmo median cut
        let mut boxes: Vec<Vec<([u8; 4], u32)>> = vec![
            colors.iter().map(|(k, v)| (*k, *v)).collect()
        ];
        
        while boxes.len() < target {
            // Encontrar la caja con mayor rango de color
            let (idx, axis) = Self::find_box_to_split(&boxes);
            
            if let Some(box_to_split) = boxes.get(idx).cloned() {
                boxes.remove(idx);
                let (box1, box2) = Self::split_box(box_to_split, axis);
                boxes.push(box1);
                boxes.push(box2);
            } else {
                break;
            }
        }
        
        // Calcular color promedio de cada caja
        boxes.iter().map(|b| {
            let (r, g, b_val, a, count) = b.iter().fold(
                (0u64, 0u64, 0u64, 0u64, 0u64),
                |(r, g, b, a, c), (color, freq)| {
                    let f = *freq as u64;
                    (r + color[0] as u64 * f, 
                     g + color[1] as u64 * f, 
                     b + color[2] as u64 * f,
                     a + color[3] as u64 * f,
                     c + f)
                }
            );
            if count > 0 {
                Rgba([
                    (r / count) as u8,
                    (g / count) as u8,
                    (b_val / count) as u8,
                    (a / count) as u8,
                ])
            } else {
                Rgba([0, 0, 0, 255])
            }
        }).collect()
    }
    
    fn find_box_to_split(boxes: &[Vec<([u8; 4], u32)>]) -> (usize, usize) {
        let mut max_range = 0u8;
        let mut best_idx = 0;
        let mut best_axis = 0;
        
        for (idx, b) in boxes.iter().enumerate() {
            for axis in 0..3 {
                let min = b.iter().map(|(c, _)| c[axis]).min().unwrap_or(0);
                let max = b.iter().map(|(c, _)| c[axis]).max().unwrap_or(0);
                let range = max - min;
                if range > max_range {
                    max_range = range;
                    best_idx = idx;
                    best_axis = axis;
                }
            }
        }
        
        (best_idx, best_axis)
    }
    
    fn split_box(mut colors: Vec<([u8; 4], u32)>, axis: usize) 
        -> (Vec<([u8; 4], u32)>, Vec<([u8; 4], u32)>) {
            // Sort carefully to avoid issues with NaN (though u8 doesn't have NaN)
            colors.sort_by(|(c1, _), (c2, _)| c1[axis].cmp(&c2[axis]));
            let mid = colors.len() / 2;
            let second = colors.split_off(mid);
            (colors, second)
    }
    
    fn find_nearest_color(palette: &[Rgba<u8>], pixel: &Rgba<u8>) -> Rgba<u8> {
        palette.iter()
            .min_by_key(|p| {
                let dr = (p.0[0] as i32 - pixel.0[0] as i32).pow(2);
                let dg = (p.0[1] as i32 - pixel.0[1] as i32).pow(2);
                let db = (p.0[2] as i32 - pixel.0[2] as i32).pow(2);
                dr + dg + db
            })
            .copied()
            .unwrap_or(*pixel)
    }

    /// Genera outline automático para el sprite
    fn generate_outline(image: &RgbaImage, color: Rgba<u8>) -> RgbaImage {
        let (width, height) = image.dimensions();
        let mut outline = RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                
                // Si el pixel es transparente, verificar vecinos
                if pixel.0[3] < 128 {
                    let has_opaque_neighbor = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ].iter().any(|&(nx, ny)| {
                        if nx < width && ny < height {
                            image.get_pixel(nx, ny).0[3] >= 128
                        } else {
                            false
                        }
                    });
                    
                    if has_opaque_neighbor {
                        outline.put_pixel(x, y, color);
                    }
                }
            }
        }
        
        outline
    }

    /// Genera capa de sombra automática
    fn generate_shadow(image: &RgbaImage, offset: (i32, i32), opacity: u8) -> RgbaImage {
        let (width, height) = image.dimensions();
        let mut shadow = RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                
                if pixel.0[3] >= 128 {
                    let shadow_x = (x as i32 + offset.0) as u32;
                    let shadow_y = (y as i32 + offset.1) as u32;
                    
                    if shadow_x < width && shadow_y < height {
                        shadow.put_pixel(shadow_x, shadow_y, Rgba([0, 0, 0, opacity]));
                    }
                }
            }
        }
        
        shadow
    }
}

// Puente Rust <-> Python para el modelo IA
struct AIBridge;

impl AIBridge {
    fn generate_base_sprite(prompt: &str, style: &str, size: u32) 
        -> PyResult<Vec<u8>> {
        Python::with_gil(|py| {
            // Add current directory to sys.path so we can import our scripts
            let sys = py.import("sys")?;
            let path = sys.getattr("path")?;
            path.call_method1("append", ("../ai_core",))?; // Adjust path as needed

            let generator = py.import("sprite_generator")?;
            
             // Create an instance or call function directly depending on implementation
             // The python code has a `generate` function wrapper
            let result = generator
                .getattr("generate")?
                .call1((prompt, style, size))?;
            
            let image_bytes: Vec<u8> = result.extract()?;
            Ok(image_bytes)
        })
    }
    
    fn generate_animation_frames(
        base_sprite: &[u8], 
        animation_type: &str,
        frame_count: u8
    ) -> PyResult<Vec<Vec<u8>>> {
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let path = sys.getattr("path")?;
            path.call_method1("append", ("../ai_core",))?;

            let animator = py.import("sprite_generator")?; // Both in same file now or separate? Prompt said sprite_generator.py and sprite_animator logic inside
            // The prompt put classes in sprite_generator.py, and interface functions at the end
            
            let result = animator
                .getattr("animate")?
                .call1((base_sprite, animation_type, frame_count))?;
            
            let frames: Vec<Vec<u8>> = result.extract()?;
            Ok(frames)
        })
    }
}

// Generador de Sprite Sheets optimizado
struct SpriteSheetGenerator;

impl SpriteSheetGenerator {
    fn create_sheet(
        frames: Vec<RgbaImage>, 
        columns: u32
    ) -> (RgbaImage, SpriteMetadata) {
        if frames.is_empty() {
            return (RgbaImage::new(1, 1), SpriteMetadata {
                frame_width: 0,
                frame_height: 0,
                frame_count: 0,
                palette: vec![],
                animation_fps: 12,
            });
        }
        
        let frame_width = frames[0].width();
        let frame_height = frames[0].height();
        let frame_count = frames.len() as u32;
        
        let rows = (frame_count + columns - 1) / columns;
        let sheet_width = frame_width * columns;
        let sheet_height = frame_height * rows;
        
        let mut sheet = RgbaImage::new(sheet_width, sheet_height);
        
        for (i, frame) in frames.iter().enumerate() {
            let col = (i as u32) % columns;
            let row = (i as u32) / columns;
            let x_offset = col * frame_width;
            let y_offset = row * frame_height;
            
            for (x, y, pixel) in frame.enumerate_pixels() {
                sheet.put_pixel(x + x_offset, y + y_offset, *pixel);
            }
        }
        
        // Extraer paleta de colores única
        let mut colors: Vec<String> = sheet.pixels()
            .filter(|p| p.0[3] > 0)
            .map(|p| format!("#{:02X}{:02X}{:02X}", p.0[0], p.0[1], p.0[2]))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        colors.sort();
        
        let metadata = SpriteMetadata {
            frame_width,
            frame_height,
            frame_count: frame_count as u8,
            palette: colors,
            animation_fps: 12,
        };
        
        (sheet, metadata)
    }
}

// API Endpoints
async fn generate_sprite(
    Json(request): Json<SpriteRequest>
) -> Json<SpriteResponse> {
    // 1. Generar sprite base con IA (Python)
    let style_str = match request.style {
        SpriteStyle::Retro8Bit => "8bit",
        SpriteStyle::Retro16Bit => "16bit",
        SpriteStyle::Modern => "modern",
        SpriteStyle::Isometric => "isometric",
    };
    
    // Call Python bridge
    let base_image_bytes = match AIBridge::generate_base_sprite(&request.prompt, style_str, request.size) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error invoking Python: {:?}", e);
            // Return empty/error response (simplification)
            return Json(SpriteResponse {
               sprite_sheet: "".to_string(),
               layers: HashMap::new(),
               metadata: SpriteMetadata { frame_width: 0, frame_height: 0, frame_count: 0, palette: vec![], animation_fps: 0 } 
            });
        }
    };

    // Load image from bytes
    let mut base_image = image::load_from_memory(&base_image_bytes).unwrap().to_rgba8();

    // 2. Procesar (Reduce palette, etc)
    if request.palette_limit > 0 {
         PixelProcessor::reduce_palette(&mut base_image, request.palette_limit);
    }

    // 3. Generate Layers (Example)
    let mut layers = HashMap::new();
    // Convert base image to base64
    // (Skipping implementation details of image->base64 for brevity, but would go here)
    
    // 4. Generate Animation Frames if needed
    // ...

    // Mock response for now to complete the structure
    Json(SpriteResponse {
        sprite_sheet: String::new(), // Would be the base64 of the final sheet
        layers: HashMap::new(),
        metadata: SpriteMetadata {
            frame_width: request.size,
            frame_height: request.size,
            frame_count: request.animation_frames,
            palette: vec![],
            animation_fps: 12,
        },
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", post(generate_sprite));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🎮 Sprite Generator running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
