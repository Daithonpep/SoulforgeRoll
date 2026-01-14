# SISTEMA OPTIMIZADO PARA RAILWAY
Arquitectura Ligera + Animaciones Fluidas

### FILOSOFÍA CENTRAL
text

┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   RAILWAY (Servidor)          NAVEGADOR (Cliente)           │
│   ════════════════           ══════════════════             │
│                                                             │
│   • Solo datos JSON          • TODAS las animaciones        │
│   • WebSocket mínimo         • Renderizado CSS/GPU          │
│   • Sin procesamiento        • Lógica de cámara             │
│     de gráficos              • Efectos visuales             │
│   • Eventos simples          • Fog of War                   │
│                                                             │
│   COSTO: ~$0.01/hora         COSTO: $0 (CPU del usuario)   │
│                                                             │
└─────────────────────────────────────────────────────────────┘

### PARTE 1: SERVIDOR RUST ULTRA-LIGERO
Rust

// main.rs - Servidor mínimo para Railway
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

// Eventos SIMPLES - solo datos, sin lógica pesada
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum GameEvent {
    // Movimiento: solo coordenadas
    Move { id: String, x: f32, y: f32 },
    
    // Cámara: solo comando + target
    Camera { action: String, target: Option<String> },
    
    // Niebla: solo punto + radio
    Reveal { x: f32, y: f32, radius: f32 },
    
    // Narrativa: solo texto
    Narrate { text: String, mood: Option<String> },
    
    // Escena: solo nombre
    Scene { name: String },
}

// Canal broadcast - sin almacenar estado pesado
type Tx = broadcast::Sender<String>;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(16); // Buffer pequeño
    let tx = Arc::new(tx);

    let app = Router::new()
        .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())));

    // Railway asigna el puerto
    let port = std::env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Server on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, tx: Arc<Tx>) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Arc<Tx>) {
    let mut rx = tx.subscribe();
    
    loop {
        tokio::select! {
            // Recibir del cliente -> broadcast a todos
            Some(Ok(msg)) = socket.recv() => {
                if let Ok(text) = msg.to_text() {
                    // NO procesar, solo reenviar
                    let _ = tx.send(text.to_string());
                }
            }
            // Recibir broadcast -> enviar al cliente
            Ok(msg) = rx.recv() => {
                let _ = socket.send(axum::extract::ws::Message::Text(msg)).await;
            }
        }
    }
}

toml

# Cargo.toml - Dependencias mínimas
[package]
name = "rpg-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.6", features = ["ws"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[profile.release]
opt-level = "z"     # Optimizar tamaño
lto = true          # Link-time optimization
codegen-units = 1   # Mejor optimización
strip = true        # Eliminar símbolos debug

### PARTE 2: CLIENTE - ANIMACIONES 100% CSS (GPU)
Reglas de Oro para Performance
CSS

/* ═══════════════════════════════════════════════════════════
   REGLAS DE PERFORMANCE
   ═══════════════════════════════════════════════════════════
   
   ✅ USAR (GPU Accelerated):
      - transform
      - opacity
      - filter (con moderación)
      - will-change (solo cuando sea necesario)
   
   ❌ EVITAR (Causa reflow/repaint):
      - width, height
      - top, left, right, bottom
      - margin, padding
      - box-shadow animado
      - border-radius animado
   
   ═══════════════════════════════════════════════════════════ */

/* Forzar GPU en elementos animados */
.gpu-accelerated {
  transform: translateZ(0);
  backface-visibility: hidden;
}

/* Preparar elemento para animación */
.will-animate {
  will-change: transform, opacity;
}

/* Limpiar will-change después de animar */
.animation-done {
  will-change: auto;
}

Sistema de Calidad Adaptativa
JavaScript

// quality-system.js - Detecta capacidad del dispositivo
class QualityManager {
  constructor() {
    this.level = this.detectQuality();
    this.applyQuality();
    
    // Re-evaluar si hay lag
    this.fpsMonitor = new FPSMonitor(fps => {
      if (fps < 30 && this.level !== 'low') {
        this.level = 'low';
        this.applyQuality();
        this.notify('Calidad reducida para mejor rendimiento');
      }
    });
  }

  detectQuality() {
    // Detectar capacidad del dispositivo
    const dominated = navigator.hardwareConcurrency || 4;
    const memory = navigator.deviceMemory || 4;
    const isMobile = /Mobile|Android|iPhone/i.test(navigator.userAgent);
    
    // Verificar preferencia de movimiento reducido
    const prefersReduced = window.matchMedia(
      '(prefers-reduced-motion: reduce)'
    ).matches;
    
    if (prefersReduced) return 'minimal';
    if (isMobile || memory < 4 || cores < 4) return 'low';
    if (memory >= 8 && cores >= 8) return 'ultra';
    return 'medium';
  }

  applyQuality() {
    document.documentElement.setAttribute('data-quality', this.level);
    
    // Guardar preferencia
    localStorage.setItem('graphics-quality', this.level);
  }

  setQuality(level) {
    this.level = level;
    this.applyQuality();
  }
}

// Monitor de FPS simple
class FPSMonitor {
  constructor(callback) {
    this.callback = callback;
    this.frames = 0;
    this.lastTime = performance.now();
    this.measure();
  }

  measure() {
    this.frames++;
    const now = performance.now();
    
    if (now - this.lastTime >= 1000) {
      this.callback(this.frames);
      this.frames = 0;
      this.lastTime = now;
    }
    
    requestAnimationFrame(() => this.measure());
  }
}

// Inicializar
const quality = new QualityManager();

CSS

/* ═══════════════════════════════════════════════════════════
   ESTILOS POR NIVEL DE CALIDAD
   ═══════════════════════════════════════════════════════════ */

/* === ULTRA === */
[data-quality="ultra"] {
  --particle-count: 50;
  --blur-quality: 12px;
  --shadow-quality: 0 4px 20px rgba(0,0,0,0.3);
  --transition-speed: 0.4s;
  --enable-particles: 1;
  --enable-blur: 1;
  --enable-shadows: 1;
}

/* === MEDIUM (Default) === */
[data-quality="medium"] {
  --particle-count: 20;
  --blur-quality: 6px;
  --shadow-quality: 0 2px 10px rgba(0,0,0,0.2);
  --transition-speed: 0.3s;
  --enable-particles: 1;
  --enable-blur: 1;
  --enable-shadows: 1;
}

/* === LOW === */
[data-quality="low"] {
  --particle-count: 5;
  --blur-quality: 2px;
  --shadow-quality: none;
  --transition-speed: 0.15s;
  --enable-particles: 0;
  --enable-blur: 0;
  --enable-shadows: 0;
}

/* === MINIMAL (Sin animaciones) === */
[data-quality="minimal"] {
  --particle-count: 0;
  --transition-speed: 0s;
  --enable-particles: 0;
  --enable-blur: 0;
  --enable-shadows: 0;
}

[data-quality="minimal"] * {
  animation: none !important;
  transition: none !important;
}

### PARTE 3: FOG OF WAR OPTIMIZADO
CSS

/* fog-optimized.css - Niebla 100% CSS */

.fog-container {
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 100;
  
  /* Una sola capa con máscara */
  background: #1a1510;
  
  /* La máscara es donde se ve (blanco = visible) */
  -webkit-mask-image: var(--fog-mask);
  mask-image: var(--fog-mask);
  
  /* Transición suave del mask */
  transition: -webkit-mask-image var(--transition-speed) ease;
}

/* Textura sutil - solo en medium/ultra */
[data-quality="ultra"] .fog-container,
[data-quality="medium"] .fog-container {
  background: 
    url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><filter id="n"><feTurbulence baseFrequency="0.7"/></filter><rect width="100" height="100" filter="url(%23n)" opacity="0.1"/></svg>'),
    #1a1510;
}

JavaScript

// fog-optimized.js - Lógica mínima de niebla
class OptimizedFog {
  constructor(container) {
    this.container = container;
    this.revealed = []; // Solo guardamos {x, y, r}
    this.maskNeedsUpdate = false;
  }

  reveal(x, y, radius) {
    this.revealed.push({ x, y, r: radius });
    this.scheduleUpdate();
  }

  // Throttle: máximo 1 update por frame
  scheduleUpdate() {
    if (this.maskNeedsUpdate) return;
    
    this.maskNeedsUpdate = true;
    requestAnimationFrame(() => {
      this.updateMask();
      this.maskNeedsUpdate = false;
    });
  }

  updateMask() {
    // Generar gradientes radiales para cada área revelada
    const gradients = this.revealed.map(({ x, y, r }) => 
      `radial-gradient(circle ${r}px at ${x}px ${y}px, transparent 0%, transparent 70%, black 100%)`
    );

    // Combinar en una sola máscara
    if (gradients.length === 0) {
      this.container.style.setProperty('--fog-mask', 'none');
    } else {
      // Crear máscara combinada
      const mask = gradients.join(', ');
      this.container.style.setProperty('--fog-mask', mask);
    }
  }

  // Optimización: fusionar áreas cercanas
  optimizeRevealed() {
    // Cada 10 revelaciones, fusionar las cercanas
    if (this.revealed.length % 10 !== 0) return;
    
    const merged = [];
    const used = new Set();
    
    for (let i = 0; i < this.revealed.length; i++) {
      if (used.has(i)) continue;
      
      const a = this.revealed[i];
      let combined = { ...a };
      
      for (let j = i + 1; j < this.revealed.length; j++) {
        if (used.has(j)) continue;
        
        const b = this.revealed[j];
        const dist = Math.hypot(a.x - b.x, a.y - b.y);
        
        // Si están muy cerca, fusionar
        if (dist < (a.r + b.r) * 0.5) {
          combined.x = (a.x + b.x) / 2;
          combined.y = (a.y + b.y) / 2;
          combined.r = Math.max(a.r, b.r) + dist * 0.3;
          used.add(j);
        }
      }
      
      merged.push(combined);
    }
    
    this.revealed = merged;
  }
}

### PARTE 4: CÁMARA CINEMATOGRÁFICA LIGERA
JavaScript

// camera-light.js - Cámara optimizada
class LightCamera {
  constructor(viewport) {
    this.viewport = viewport;
    this.map = viewport.querySelector('.map-container');
    
    // Estado actual
    this.current = { x: 0, y: 0, scale: 1 };
    this.target = { x: 0, y: 0, scale: 1 };
    
    // Config
    this.easing = 0.1;
    this.isAnimating = false;
  }

  // ═══════════════════════════════════════
  // MOVIMIENTOS BÁSICOS (Todo en CSS)
  // ═══════════════════════════════════════

  panTo(x, y, duration = 500) {
    this.map.style.transition = `transform ${duration}ms cubic-bezier(0.4, 0, 0.2, 1)`;
    this.current.x = x;
    this.current.y = y;
    this.applyTransform();
    
    // Limpiar transition después
    setTimeout(() => {
      this.map.style.transition = '';
    }, duration);
  }

  zoomTo(scale, duration = 500) {
    this.map.style.transition = `transform ${duration}ms cubic-bezier(0.4, 0, 0.2, 1)`;
    this.current.scale = Math.max(0.5, Math.min(3, scale));
    this.applyTransform();
    
    setTimeout(() => {
      this.map.style.transition = '';
    }, duration);
  }

  focusOn(element, scale = 2, duration = 600) {
    const rect = element.getBoundingClientRect();
    const mapRect = this.map.getBoundingClientRect();
    
    const x = (rect.left - mapRect.left + rect.width / 2) - (this.viewport.clientWidth / 2);
    const y = (rect.top - mapRect.top + rect.height / 2) - (this.viewport.clientHeight / 2);
    
    this.map.style.transition = `transform ${duration}ms cubic-bezier(0.4, 0, 0.2, 1)`;
    this.current = { x, y, scale };
    this.applyTransform();
    
    setTimeout(() => {
      this.map.style.transition = '';
    }, duration);
  }

  applyTransform() {
    const { x, y, scale } = this.current;
    this.map.style.transform = `translate(${-x}px, ${-y}px) scale(${scale})`;
  }

  // ═══════════════════════════════════════
  // EFECTOS (Solo clases CSS)
  // ═══════════════════════════════════════

  dramatic(element) {
    this.viewport.classList.add('effect-dramatic');
    this.focusOn(element, 2.5, 1500);
    
    setTimeout(() => {
      this.viewport.classList.remove('effect-dramatic');
    }, 3000);
  }

  combat() {
    this.viewport.classList.add('effect-combat');
  }

  endCombat() {
    this.viewport.classList.remove('effect-combat');
    this.reset();
  }

  shake(intensity = 'medium') {
    this.viewport.classList.add(`shake-${intensity}`);
    
    setTimeout(() => {
      this.viewport.classList.remove(`shake-${intensity}`);
    }, 500);
  }

  flash(color = 'white') {
    this.viewport.classList.add(`flash-${color}`);
    
    setTimeout(() => {
      this.viewport.classList.remove(`flash-${color}`);
    }, 200);
  }

  reset(duration = 800) {
    this.map.style.transition = `transform ${duration}ms cubic-bezier(0.4, 0, 0.2, 1)`;
    this.current = { x: 0, y: 0, scale: 1 };
    this.applyTransform();
    
    setTimeout(() => {
      this.map.style.transition = '';
    }, duration);
  }
}

CSS

/* camera-effects.css - Todo en CSS puro */

/* ═══════════════════════════════════════
   VIÑETA Y OVERLAYS
   ═══════════════════════════════════════ */

.game-viewport::before {
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 500;
  opacity: 0;
  transition: opacity var(--transition-speed) ease;
}

/* Viñeta dramática */
.effect-dramatic::before {
  opacity: 1;
  background: radial-gradient(
    ellipse at center,
    transparent 30%,
    rgba(0, 0, 0, 0.4) 70%,
    rgba(0, 0, 0, 0.8) 100%
  );
}

.effect-dramatic {
  filter: saturate(0.7) contrast(1.1);
  transition: filter 1s ease;
}

/* Tensión de combate */
.effect-combat::before {
  opacity: 1;
  background: radial-gradient(
    ellipse at center,
    transparent 50%,
    rgba(80, 0, 0, 0.3) 100%
  );
  animation: combat-pulse-light 2s ease-in-out infinite;
}

@keyframes combat-pulse-light {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

/* ═══════════════════════════════════════
   SHAKE - Usando transform (GPU)
   ═══════════════════════════════════════ */

.shake-light {
  animation: shake-light 0.3s ease-out;
}

.shake-medium {
  animation: shake-medium 0.4s ease-out;
}

.shake-heavy {
  animation: shake-heavy 0.5s ease-out;
}

@keyframes shake-light {
  0%, 100% { transform: translate(0, 0); }
  20% { transform: translate(-2px, 1px); }
  40% { transform: translate(2px, -1px); }
  60% { transform: translate(-1px, 1px); }
  80% { transform: translate(1px, -1px); }
}

@keyframes shake-medium {
  0%, 100% { transform: translate(0, 0); }
  20% { transform: translate(-5px, 3px); }
  40% { transform: translate(5px, -3px); }
  60% { transform: translate(-3px, 2px); }
  80% { transform: translate(3px, -2px); }
}

@keyframes shake-heavy {
  0%, 100% { transform: translate(0, 0); }
  10% { transform: translate(-10px, 5px); }
  20% { transform: translate(10px, -5px); }
  30% { transform: translate(-8px, 4px); }
  40% { transform: translate(8px, -4px); }
  50% { transform: translate(-5px, 3px); }
  60% { transform: translate(5px, -3px); }
  70% { transform: translate(-3px, 2px); }
  80% { transform: translate(3px, -1px); }
  90% { transform: translate(-1px, 1px); }
}

/* ═══════════════════════════════════════
   FLASH - Solo opacity (GPU)
   ═══════════════════════════════════════ */

.game-viewport::after {
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 600;
  opacity: 0;
}

.flash-white::after {
  background: white;
  animation: flash-anim 0.2s ease-out;
}

.flash-red::after {
  background: #ff0000;
  animation: flash-anim 0.3s ease-out;
}

.flash-gold::after {
  background: #ffd700;
  animation: flash-anim 0.4s ease-out;
}

@keyframes flash-anim {
  0% { opacity: 0.8; }
  100% { opacity: 0; }
}

/* ═══════════════════════════════════════
   TRANSICIONES DE ESCENA
   ═══════════════════════════════════════ */

.scene-transition {
  position: fixed;
  inset: 0;
  z-index: 1000;
  pointer-events: none;
}

/* Fade simple */
.scene-fade {
  background: #0a0a0a;
  animation: scene-fade 1.5s ease-in-out;
}

@keyframes scene-fade {
  0% { opacity: 0; }
  40% { opacity: 1; }
  60% { opacity: 1; }
  100% { opacity: 0; }
}

/* Círculo que cierra (iris wipe) */
.scene-iris {
  background: #0a0a0a;
  clip-path: circle(150% at center);
  animation: iris-close 2s ease-in-out;
}

@keyframes iris-close {
  0% { clip-path: circle(150% at center); }
  45% { clip-path: circle(0% at center); }
  55% { clip-path: circle(0% at center); }
  100% { clip-path: circle(150% at center); }
}

### PARTE 5: WEBSOCKET OPTIMIZADO
JavaScript

// ws-client.js - Cliente WebSocket eficiente
class GameSocket {
  constructor(url) {
    this.url = url;
    this.ws = null;
    this.handlers = new Map();
    this.messageQueue = [];
    this.isThrottled = false;
    
    this.connect();
  }

  connect() {
    this.ws = new WebSocket(this.url);
    
    this.ws.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        this.handleMessage(data);
      } catch (err) {
        console.error('Parse error:', err);
      }
    };

    this.ws.onclose = () => {
      // Reconectar después de 2 segundos
      setTimeout(() => this.connect(), 2000);
    };
  }

  // Registrar handler para tipo de evento
  on(type, callback) {
    this.handlers.set(type, callback);
  }

  handleMessage(data) {
    const handler = this.handlers.get(data.type);
    if (handler) handler(data);
  }

  // Enviar con throttle (máx 1 mensaje cada 50ms)
  send(data) {
    this.messageQueue.push(data);
    
    if (!this.isThrottled) {
      this.flushQueue();
      this.isThrottled = true;
      
      setTimeout(() => {
        this.isThrottled = false;
        if (this.messageQueue.length > 0) {
          this.flushQueue();
        }
      }, 50);
    }
  }

  flushQueue() {
    if (this.ws.readyState !== WebSocket.OPEN) return;
    
    // Si hay múltiples mensajes del mismo tipo, enviar solo el último
    const latest = new Map();
    
    this.messageQueue.forEach(msg => {
      latest.set(msg.type, msg);
    });
    
    latest.forEach(msg => {
      this.ws.send(JSON.stringify(msg));
    });
    
    this.messageQueue = [];
  }
}

// ═══════════════════════════════════════
// USO
// ═══════════════════════════════════════

const socket = new GameSocket('wss://tu-app.railway.app/ws');
const camera = new LightCamera(document.querySelector('.game-viewport'));
const fog = new OptimizedFog(document.querySelector('.fog-container'));

// Manejar eventos del servidor
socket.on('Move', (data) => {
  const token = document.getElementById(data.id);
  if (token) {
    token.style.transform = `translate(${data.x}px, ${data.y}px)`;
    
    // Si es el jugador local, revelar niebla
    if (data.id === localPlayerId) {
      fog.reveal(data.x, data.y, 120);
    }
  }
});

socket.on('Camera', (data) => {
  switch (data.action) {
    case 'focus':
      const target = document.getElementById(data.target);
      if (target) camera.focusOn(target);
      break;
    case 'dramatic':
      const el = document.getElementById(data.target);
      if (el) camera.dramatic(el);
      break;
    case 'shake':
      camera.shake(data.intensity || 'medium');
      break;
    case 'reset':
      camera.reset();
      break;
  }
});

socket.on('Scene', (data) => {
  // Transición de escena
  const transition = document.createElement('div');
  transition.className = `scene-transition scene-${data.style || 'fade'}`;
  document.body.appendChild(transition);
  
  setTimeout(() => transition.remove(), 2000);
});

### PARTE 6: HTML ESTRUCTURA COMPLETA
HTML

<!DOCTYPE html>
<html lang="es" data-quality="medium">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>RPG Chronicle</title>
  
  <style>
    /* Reset mínimo */
    *, *::before, *::after {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      background: #0a0908;
      color: #e8e2d9;
      font-family: 'Crimson Text', Georgia, serif;
      overflow: hidden;
      height: 100vh;
    }

    /* Viewport principal */
    .game-viewport {
      position: relative;
      width: 100vw;
      height: 100vh;
      overflow: hidden;
      background: #12100e;
    }

    /* Contenedor del mapa - GPU accelerated */
    .map-container {
      position: absolute;
      width: 2000px;
      height: 2000px;
      left: 50%;
      top: 50%;
      margin-left: -1000px;
      margin-top: -1000px;
      transform-origin: center center;
      will-change: transform;
    }

    /* Mapa base */
    .map-layer {
      position: absolute;
      inset: 0;
      background: 
        url('map-texture.jpg') center/cover,
        #2a231c;
    }

    /* Tokens de personajes */
    .token {
      position: absolute;
      width: 48px;
      height: 48px;
      border-radius: 50%;
      background: #333;
      border: 3px solid #c9a959;
      transform-origin: center;
      transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      will-change: transform;
      cursor: pointer;
    }

    .token img {
      width: 100%;
      height: 100%;
      border-radius: 50%;
      object-fit: cover;
    }

    .token:hover {
      transform: scale(1.1);
      z-index: 10;
    }

    /* Niebla optimizada */
    .fog-container {
      position: absolute;
      inset: 0;
      background: #1a1510;
      pointer-events: none;
      z-index: 100;
      /* Máscara dinámica via CSS variable */
      -webkit-mask-image: var(--fog-mask, none);
      mask-image: var(--fog-mask, none);
      -webkit-mask-composite: destination-out;
      mask-composite: exclude;
    }

    /* Panel lateral */
    .side-panel {
      position: fixed;
      top: 0;
      left: 0;
      width: 280px;
      height: 100vh;
      background: linear-gradient(90deg, rgba(15,12,10,0.95) 0%, transparent 100%);
      padding: 20px;
      transform: translateX(-240px);
      transition: transform 0.3s ease;
      z-index: 200;
    }

    .side-panel:hover {
      transform: translateX(0);
    }

    /* Barra narrativa */
    .narrative-bar {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      padding: 20px 40px;
      background: linear-gradient(transparent, rgba(10,9,8,0.95));
      z-index: 200;
    }

    .narrative-text {
      font-size: 1.1em;
      line-height: 1.6;
      color: #d4c8b8;
      font-style: italic;
      opacity: 0.9;
    }

    /* Indicador de carga */
    .loading {
      position: fixed;
      inset: 0;
      background: #0a0908;
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 9999;
      transition: opacity 0.5s ease;
    }

    .loading.hidden {
      opacity: 0;
      pointer-events: none;
    }

    .loading-spinner {
      width: 40px;
      height: 40px;
      border: 3px solid #333;
      border-top-color: #c9a959;
      border-radius: 50%;
      animation: spin 1s linear infinite;
    }

    @keyframes spin {
      to { transform: rotate(360deg); }
    }
  </style>
  
  <link rel="stylesheet" href="camera-effects.css">
</head>

<body>
  <!-- Loading -->
  <div class="loading" id="loading">
    <div class="loading-spinner"></div>
  </div>

  <!-- Viewport principal -->
  <div class="game-viewport" id="viewport">
    
    <!-- Mapa -->
    <div class="map-container" id="map">
      <div class="map-layer"></div>
      
      <!-- Tokens se añaden dinámicamente -->
      
      <!-- Niebla -->
      <div class="fog-container" id="fog"></div>
    </div>
    
  </div>

  <!-- Panel lateral (fichas) -->
  <aside class="side-panel" id="characters-panel">
    <h3>Personajes</h3>
    <!-- Fichas aquí -->
  </aside>

  <!-- Narrativa -->
  <footer class="narrative-bar">
    <p class="narrative-text" id="narrative">
      Esperando al Director de Juego...
    </p>
  </footer>

  <script src="quality-system.js"></script>
  <script src="fog-optimized.js"></script>
  <script src="camera-light.js"></script>
  <script src="ws-client.js"></script>
  
  <script>
    // Inicialización
    window.addEventListener('load', () => {
      // Ocultar loading
      document.getElementById('loading').classList.add('hidden');
    });
  </script>
</body>
</html>
