# SoulForge Frontend

Frontend web estático para el sistema SoulForge.

## Estructura
```
/
├── index.html              # Página principal
├── juego_en_linea.html    # VTT/Juego en línea  
├── hoja_personaje.html    # Template de personaje
├── constelacion.html      # Template de constelación
├── muro_caidos.html       # Muro de personajes caídos
├── informe_descarga.html  # Página de descarga
├── static/
│   └── images/           # Imágenes (logo, background)
└── vercel.json           # Configuración de Vercel

## Despliegue

Este directorio está diseñado para desplegarse directamente en Vercel.
Los endpoints de API se redirigen automáticamente al backend en Railway.

## Backend

El backend (Rust/Actix) está en `SpriteEngine/motor_de_historia/backend/` 
y se despliega independientemente en Railway.
