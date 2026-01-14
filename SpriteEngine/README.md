# Superior AI Sprite Generator

Sistema avanzado de generación y animación de sprites con **comprensión semántica**.

## 🎯 Problema Resuelto

| Sistema Anterior | Sistema Nuevo |
|------------------|---------------|
| Mueve rectángulos sin entender | Reconoce cabeza, torso, brazos, piernas |
| Sin consistencia entre partes | Skeleton con 16 joints para proporciones |
| Capas sin sentido | Capas semánticas (shading, outline, highlights) |
| Bordes borrosos | Post-proceso pixel-perfect |
| Paleta inconsistente | Reducción optimizada de paleta |

## 📁 Estructura

```
SpriteEngine/
├── ai_core/
│   ├── sprite_segmenter.py      # Segmentación semántica
│   ├── layer_generator.py       # Generación de capas
│   ├── sprite_engine_integrator.py  # Integración con AnimationAmber
│   ├── sprite_generator.py      # Generación con Stable Diffusion
│   └── train_sprite_model.py    # Fine-tuning de modelos
├── rust_backend/                # Backend de alto rendimiento (opcional)
│   ├── Cargo.toml
│   └── src/main.rs
└── data/                        # Datos de entrenamiento
    └── sprite_dataset/
```

## 🚀 Uso Rápido

### Procesar un Sprite Existente

```python
from SpriteEngine.ai_core.sprite_engine_integrator import SpriteEngineIntegrator, quick_process
from PIL import Image

# Opción 1: Archivo
result = quick_process("mi_sprite.png", "./output")

# Opción 2: Imagen PIL
integrator = SpriteEngineIntegrator()
sprite = Image.open("mi_sprite.png")
result = integrator.process_sprite(
    sprite,
    generate_layers=True,
    generate_animations=True,
    palette_limit=16,
    character_id="mi_personaje"
)

# Guardar todo
result.save_all("./output")

# Usar con animation_generator.py existente
skeleton = result.get_compatible_skeleton_for_animation()
parts = result.get_compatible_parts_for_animation()
```

### Generar Capas de un Sprite

```python
from SpriteEngine.ai_core.layer_generator import SemanticLayerGenerator, LayerType
from PIL import Image

generator = SemanticLayerGenerator()
sprite = Image.open("mi_sprite.png")

result = generator.process_existing_sprite(
    sprite,
    generate_layers=[
        LayerType.BASE_COLOR,
        LayerType.SHADING,
        LayerType.OUTLINE,
        LayerType.HIGHLIGHTS,
    ],
    palette_limit=16
)

# Guardar capas individuales
result.layers[LayerType.BASE_COLOR].save("base.png")
result.layers[LayerType.OUTLINE].save("outline.png")
result.layers[LayerType.SHADING].save("shading.png")

# O guardar todo
result.save_all("./output", "mi_sprite")
```

### Crear Spritesheet Animado

```python
from SpriteEngine.ai_core.sprite_engine_integrator import create_spritesheet
from PIL import Image

sprite = Image.open("sprite_estatico.png")

# Generar spritesheet de 4 frames
spritesheet = create_spritesheet(
    sprite,
    animation_type="idle",  # idle, walk, attack
    frame_count=4,
    columns=4
)

spritesheet.save("spritesheet.png")
```

### Segmentar y Visualizar

```python
from SpriteEngine.ai_core.sprite_segmenter import SpriteSegmenter, visualize_segmentation
from PIL import Image

segmenter = SpriteSegmenter()
sprite = Image.open("mi_sprite.png")

result = segmenter.segment(sprite)

# Visualizar partes detectadas
vis = visualize_segmentation(result)
vis.save("segmentation_debug.png")

# Ver partes detectadas
for part, confidence in result.confidence.items():
    if confidence > 0:
        print(f"{part.name}: {confidence:.2%}")

# Convertir a formato para AnimationAmber
skeleton_dict, parts_dict = result.to_animation_generator_format()
```

## 🔗 Integración con Sistemas Existentes

### Con AnimationAmber.py

```python
from SpriteEngine.ai_core.sprite_engine_integrator import SpriteEngineIntegrator
from AnimationAmber import AnimadorProceduralAvanzado

integrator = SpriteEngineIntegrator()
result = integrator.process_sprite(sprite)

# El skeleton_adapter es compatible con AnimationAmber
animator = AnimadorProceduralAvanzado(result.skeleton_adapter)
animator.params.velocidad = 0.5
animator.actualizar(delta_time=0.016)
```

### Con animation_generator.py

```python
from SpriteEngine.ai_core.sprite_engine_integrator import SpriteEngineIntegrator
from animation_generator import AnimationGenerator

integrator = SpriteEngineIntegrator()
result = integrator.process_sprite(sprite)

# Ya tiene las animaciones generadas
print(result.animations.keys())  # idle, walk, run, jump, etc.

# O generar manualmente
gen = AnimationGenerator()
skeleton, parts = result.segmentation.to_animation_generator_format()
animations = gen.generate_all(skeleton, parts, "mi_personaje")
```

## 🎨 Capas Generadas

| Capa | Descripción |
|------|-------------|
| `BASE_COLOR` | Colores originales del sprite |
| `SHADING` | Sombras basadas en dirección de luz |
| `OUTLINE` | Contorno pixel-perfect |
| `HIGHLIGHTS` | Brillos (opuesto a sombras) |
| `AMBIENT_OCCLUSION` | Oscurecimiento en uniones |
| `NORMAL_MAP` | Mapa de normales para iluminación dinámica |

## 🦴 Partes Semánticas Detectadas

- `HEAD` - Cabeza
- `TORSO` - Torso
- `ARM_LEFT/RIGHT` - Brazos
- `HAND_LEFT/RIGHT` - Manos
- `LEG_LEFT/RIGHT` - Piernas
- `FOOT_LEFT/RIGHT` - Pies
- `WEAPON` - Arma
- `SHIELD` - Escudo
- `ACCESSORY` - Accesorios (capa, alas, etc.)

## 🔧 Requisitos

```bash
pip install torch numpy pillow
pip install opencv-python  # Opcional, mejora calidad
pip install diffusers transformers accelerate  # Para generación con IA
```

## 📊 Flujo de Procesamiento

```
SPRITE ORIGINAL
      │
      ▼
┌─────────────────┐
│ SEGMENTADOR     │ → Detecta partes semánticas
│ SEMÁNTICO       │ → Genera skeleton (16 joints)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ GENERADOR DE    │ → Shading automático
│ CAPAS           │ → Outline pixel-perfect
│                 │ → Highlights
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ INTEGRADOR      │ → Compatible con AnimationAmber
│                 │ → Compatible con animation_generator
│                 │ → Exporta .tres para Godot
└────────┬────────┘
         │
         ▼
    OUTPUT COMPLETO
    ├── sprite.png
    ├── layers/
    │   ├── base.png
    │   ├── shading.png
    │   └── outline.png
    ├── parts/
    │   ├── head.png
    │   ├── torso.png
    │   └── ...
    ├── skeleton.json
    └── animations.tres
```

## 🎮 Exportación para Godot

El sistema genera archivos `.tres` directamente importables en Godot:

```gdscript
# En Godot
var anim_lib = load("res://sprites/character_animations.tres")
$AnimationPlayer.add_animation_library("", anim_lib)
$AnimationPlayer.play("idle")
```
