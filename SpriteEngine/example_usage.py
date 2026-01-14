"""
╔══════════════════════════════════════════════════════════════════════════════╗
║                    EJEMPLO DE USO - SPRITE ENGINE                           ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  Este script demuestra cómo usar el sistema completo de sprites:             ║
║  1. Crear un sprite de prueba                                               ║
║  2. Segmentarlo para entender sus partes                                    ║
║  3. Generar capas automáticas                                               ║
║  4. Crear animaciones basadas en el skeleton detectado                      ║
║  5. Exportar para Godot                                                     ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""

import sys
from pathlib import Path
from PIL import Image, ImageDraw

# Agregar path
current_dir = Path(__file__).parent
sys.path.insert(0, str(current_dir / "ai_core"))

# Imports del sistema
from sprite_segmenter import SpriteSegmenter, SpritePart, visualize_segmentation
from layer_generator import SemanticLayerGenerator, LayerType
from sprite_engine_integrator import SpriteEngineIntegrator, create_spritesheet


def create_test_sprite(size: int = 64) -> Image.Image:
    """Crea un sprite de prueba para demostración"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Escalar proporciones al tamaño
    scale = size / 64
    
    # Cabeza (círculo arriba)
    head_x1 = int(24 * scale)
    head_y1 = int(5 * scale)
    head_x2 = int(40 * scale)
    head_y2 = int(20 * scale)
    draw.ellipse([head_x1, head_y1, head_x2, head_y2], fill=(255, 200, 150, 255))
    
    # Ojos
    eye_size = max(1, int(2 * scale))
    eye_y = int(10 * scale)
    draw.ellipse([int(28 * scale), eye_y, int(28 * scale) + eye_size, eye_y + eye_size], fill=(50, 50, 50, 255))
    draw.ellipse([int(34 * scale), eye_y, int(34 * scale) + eye_size, eye_y + eye_size], fill=(50, 50, 50, 255))
    
    # Torso (rectángulo centro)
    draw.rectangle([int(26 * scale), int(20 * scale), int(38 * scale), int(40 * scale)], 
                   fill=(100, 100, 200, 255))
    
    # Brazos
    draw.rectangle([int(18 * scale), int(22 * scale), int(26 * scale), int(38 * scale)], 
                   fill=(255, 200, 150, 255))
    draw.rectangle([int(38 * scale), int(22 * scale), int(46 * scale), int(38 * scale)], 
                   fill=(255, 200, 150, 255))
    
    # Piernas
    draw.rectangle([int(26 * scale), int(40 * scale), int(31 * scale), int(58 * scale)], 
                   fill=(50, 50, 150, 255))
    draw.rectangle([int(33 * scale), int(40 * scale), int(38 * scale), int(58 * scale)], 
                   fill=(50, 50, 150, 255))
    
    # Pies
    draw.rectangle([int(24 * scale), int(56 * scale), int(32 * scale), int(60 * scale)], 
                   fill=(80, 60, 40, 255))
    draw.rectangle([int(32 * scale), int(56 * scale), int(40 * scale), int(60 * scale)], 
                   fill=(80, 60, 40, 255))
    
    return img


def demo_segmentation():
    """Demuestra la segmentación semántica"""
    print("\n" + "="*60)
    print("DEMO 1: SEGMENTACIÓN SEMÁNTICA")
    print("="*60)
    
    # Crear sprite
    sprite = create_test_sprite(64)
    print("✓ Sprite de prueba creado (64x64)")
    
    # Segmentar
    segmenter = SpriteSegmenter()
    result = segmenter.segment(sprite)
    
    print("\n📊 Partes detectadas:")
    for part, confidence in result.confidence.items():
        if confidence > 0:
            part_name = SpritePart.to_animation_part_name(part)
            mask = result.masks[part]
            pixels = int(mask.sum())
            print(f"   • {part_name:12} - {confidence:.1%} confianza, {pixels} pixels")
    
    print(f"\n🦴 Skeleton con {len(result.skeleton.joints)} joints:")
    for name, joint in list(result.skeleton.joints.items())[:5]:
        print(f"   • {name}: ({joint.x:.2f}, {joint.y:.2f})")
    print("   ...")
    
    # Visualizar
    vis = visualize_segmentation(result)
    print(f"\n✓ Visualización generada: {vis.size}")
    
    return sprite, result


def demo_layer_generation(sprite: Image.Image):
    """Demuestra la generación de capas"""
    print("\n" + "="*60)
    print("DEMO 2: GENERACIÓN DE CAPAS")
    print("="*60)
    
    generator = SemanticLayerGenerator()
    
    # Configurar dirección de luz
    generator.set_light_direction(-1, -1)  # Arriba-izquierda
    
    result = generator.process_existing_sprite(
        sprite,
        generate_layers=[
            LayerType.BASE_COLOR,
            LayerType.SHADING,
            LayerType.OUTLINE,
            LayerType.HIGHLIGHTS,
            LayerType.AMBIENT_OCCLUSION,
        ],
        palette_limit=16
    )
    
    print("\n🎨 Capas generadas:")
    for layer_type, layer_img in result.layers.items():
        print(f"   • {layer_type.value:12} - {layer_img.size}")
    
    print(f"\n✓ Composite final: {result.composite.size}")
    
    return result


def demo_animation_integration(sprite: Image.Image):
    """Demuestra la integración con el sistema de animación"""
    print("\n" + "="*60)
    print("DEMO 3: INTEGRACIÓN CON ANIMACIÓN")
    print("="*60)
    
    integrator = SpriteEngineIntegrator()
    
    result = integrator.process_sprite(
        sprite,
        generate_layers=True,
        generate_animations=True,
        palette_limit=16,
        character_id="demo_character"
    )
    
    print(f"\n🎬 Animaciones generadas: {len(result.animations)}")
    if result.animations:
        for name, anim in list(result.animations.items())[:5]:
            tracks = len(anim.tracks) if hasattr(anim, 'tracks') else '?'
            duration = anim.duration if hasattr(anim, 'duration') else '?'
            print(f"   • {name:15} - {tracks} tracks, {duration}s")
        if len(result.animations) > 5:
            print(f"   ... y {len(result.animations) - 5} más")
    
    # Skeleton adapter para AnimationAmber
    print(f"\n🦴 Skeleton adapter: {len(result.skeleton_adapter.huesos)} huesos")
    for name in list(result.skeleton_adapter.huesos.keys())[:5]:
        bone = result.skeleton_adapter.huesos[name]
        print(f"   • {name}: pos=({bone.local.x:.1f}, {bone.local.y:.1f})")
    
    return result


def demo_spritesheet():
    """Demuestra la generación de spritesheet"""
    print("\n" + "="*60)
    print("DEMO 4: SPRITESHEET ANIMADO")
    print("="*60)
    
    sprite = create_test_sprite(32)  # Más pequeño para spritesheet
    
    spritesheet = create_spritesheet(
        sprite,
        animation_type="idle",
        frame_count=4,
        columns=4
    )
    
    print(f"✓ Spritesheet generado: {spritesheet.size}")
    print(f"   • Frames: 4")
    print(f"   • Frame size: 32x32")
    print(f"   • Layout: 4x1")
    
    return spritesheet


def demo_save_output():
    """Demuestra cómo guardar todo el output"""
    print("\n" + "="*60)
    print("DEMO 5: GUARDAR OUTPUT COMPLETO")
    print("="*60)
    
    sprite = create_test_sprite(64)
    integrator = SpriteEngineIntegrator()
    
    result = integrator.process_sprite(
        sprite,
        generate_layers=True,
        generate_animations=True,
        character_id="example_character"
    )
    
    # Crear directorio de output
    output_dir = Path(__file__).parent / "example_output"
    
    print(f"\n📁 Guardando en: {output_dir}")
    result.save_all(str(output_dir))
    
    # Listar archivos creados
    if output_dir.exists():
        print("\n📄 Archivos creados:")
        for file in sorted(output_dir.rglob("*")):
            if file.is_file():
                rel_path = file.relative_to(output_dir)
                size = file.stat().st_size
                print(f"   • {rel_path} ({size} bytes)")


def main():
    """Ejecuta todas las demos"""
    print("""
╔══════════════════════════════════════════════════════════════════╗
║         SPRITE ENGINE - DEMOSTRACIÓN COMPLETA                    ║
╚══════════════════════════════════════════════════════════════════╝
    """)
    
    # Demo 1: Segmentación
    sprite, seg_result = demo_segmentation()
    
    # Demo 2: Capas
    layers_result = demo_layer_generation(sprite)
    
    # Demo 3: Animación
    anim_result = demo_animation_integration(sprite)
    
    # Demo 4: Spritesheet
    spritesheet = demo_spritesheet()
    
    # Demo 5: Guardar
    demo_save_output()
    
    print("\n" + "="*60)
    print("✅ TODAS LAS DEMOS COMPLETADAS")
    print("="*60)
    print("""
Próximos pasos:
1. Revisa la carpeta 'example_output' para ver los archivos generados
2. Importa el .tres en Godot para usar las animaciones
3. Usa las capas separadas para edición avanzada
4. Integra con AnimationAmber.py para animación procedural
    """)


if __name__ == "__main__":
    main()
