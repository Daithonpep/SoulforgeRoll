# LA CRÓNICA VISUAL
## El Tablero Cinematográfico

### PARTE A: LA ESTRUCTURA "BREATHING CANVAS"
La idea central: El tablero RESPIRA y REACCIONA a la narrativa.

text

┌─────────────────────────────────────────────────────────┐
│                    ZONA AMBIENTAL                       │
│              (partículas, clima, hora)                  │
├────────┬───────────────────────────────────┬────────────┤
│        │                                   │            │
│ FICHAS │      PERGAMINO CENTRAL            │  PANEL     │
│  (L)   │      (Mapa Vivo)                  │   DM (R)   │
│        │                                   │            │
│ hover  │   ← se despliega como rollo →     │  oculto    │
│ reveal │                                   │  default   │
│        │                                   │            │
├────────┴───────────────────────────────────┴────────────┤
│              BARRA NARRATIVA (typing del DM)            │
└─────────────────────────────────────────────────────────┘

### PARTE B: ANIMACIONES CORE
1. "INK REVEAL" - El Mapa que se Dibuja
CSS

/* El mapa aparece como si una pluma lo dibujara */
.map-region {
  stroke-dasharray: 1000;
  stroke-dashoffset: 1000;
  animation: ink-draw 3s ease-out forwards;
  filter: url(#ink-bleed); /* SVG filter para bordes de tinta */
}

@keyframes ink-draw {
  to {
    stroke-dashoffset: 0;
  }
}

/* Efecto de tinta expandiéndose en papel */
.region-fill {
  clip-path: circle(0% at center);
  animation: ink-spread 2s ease-out 1s forwards;
}

@keyframes ink-spread {
  to {
    clip-path: circle(100% at center);
  }
}
2. "BREATHING WORLD" - El Mapa Vive
CSS

/* Movimiento sutil constante - el mundo respira */
.map-container {
  animation: world-breath 8s ease-in-out infinite;
}

@keyframes world-breath {
  0%, 100% { 
    transform: scale(1); 
    filter: brightness(1);
  }
  50% { 
    transform: scale(1.003); 
    filter: brightness(1.02);
  }
}

/* Los árboles del bosque se mecen */
.forest-elements {
  transform-origin: bottom center;
  animation: tree-sway 4s ease-in-out infinite;
  animation-delay: calc(var(--tree-index) * 0.3s);
}

@keyframes tree-sway {
  0%, 100% { transform: rotate(-1deg); }
  50% { transform: rotate(1deg); }
}

### PARTE C: TRANSICIONES CINEMATOGRÁFICAS
🎥 "SCENE SHIFT" - Cambio de Escena Épico
Cuando el DM cambia de ubicación:

JavaScript

// Detecta palabras clave del DM
const sceneKeywords = {
  tavern: ['taberna', 'posada', 'bar'],
  forest: ['bosque', 'arboleda', 'selva'],
  dungeon: ['mazmorra', 'cueva', 'cripta'],
  battle: ['combate', 'ataque', 'iniciativa']
};

function triggerSceneTransition(type) {
  const transitions = {
    tavern: 'warmFade',      // Fade cálido anaranjado
    forest: 'leafWipe',      // Hojas que barren la pantalla
    dungeon: 'shadowClose',  // Sombras que cierran desde bordes
    battle: 'shatterZoom'    // Zoom rápido + efecto cristal
  };
  
  executeTransition(transitions[type]);
}
CSS

/* LEAF WIPE - Para transición a bosque */
.transition-leaf-wipe {
  position: fixed;
  inset: 0;
  background: 
    url('leaf-1.svg') -100% 20%,
    url('leaf-2.svg') -100% 50%,
    url('leaf-3.svg') -100% 80%;
  animation: leaves-sweep 1.5s ease-in-out forwards;
}

@keyframes leaves-sweep {
  0% { 
    transform: translateX(-100%); 
    opacity: 0;
  }
  30% { opacity: 1; }
  50% { transform: translateX(0%); }
  70% { opacity: 1; }
  100% { 
    transform: translateX(100%); 
    opacity: 0;
  }
}

/* SHADOW CLOSE - Para mazmorras */
.transition-shadow {
  --shadow-progress: 0%;
  background: radial-gradient(
    circle at center,
    transparent var(--shadow-progress),
    rgba(0,0,0,0.95) calc(var(--shadow-progress) + 20%)
  );
  animation: shadow-engulf 2s ease-in-out forwards;
}

@keyframes shadow-engulf {
  0% { --shadow-progress: 100%; }
  45% { --shadow-progress: 0%; }
  55% { --shadow-progress: 0%; }
  100% { --shadow-progress: 100%; }
}

### PARTE D: EFECTOS REACTIVOS A NARRATIVA
⚡ "EMOTION PULSE" - El Tablero Siente
JavaScript

// Sistema de detección emocional en texto del DM
const emotionTriggers = {
  tension: ['peligro', 'acecha', 'oscuridad', 'miedo'],
  epic: ['héroe', 'victoria', 'gloria', 'legendario'],
  mystery: ['extraño', 'antiguo', 'susurro', 'secreto'],
  death: ['muerte', 'caído', 'sangre', 'último aliento']
};

// Configuración de Prioridad (Optimizacion: Prioridad de Ánimo)
const emotionPriority = {
  death: 4,
  epic: 3,
  tension: 2,
  mystery: 1
};

// Implementación de Debounce y Prioridad
let typingTimeout;
function handleInput(text) {
  clearTimeout(typingTimeout);
  // Esperar 1s de inactividad o Enter para procesar
  typingTimeout = setTimeout(() => processNarrative(text), 1000);
}

function processNarrative(text) {
  const foundEmotions = [];
  
  // Buscar todas las coincidencias
  for (const [emotion, keywords] of Object.entries(emotionTriggers)) {
    if (keywords.some(k => text.toLowerCase().includes(k))) {
      foundEmotions.push(emotion);
    }
  }
  
  if (foundEmotions.length === 0) return;
  
  // Seleccionar la de mayor prioridad
  const dominantEmotion = foundEmotions.reduce((prev, current) => {
    return (emotionPriority[current] > emotionPriority[prev]) ? current : prev;
  });

  applyEmotionEffect(dominantEmotion);
}

function applyEmotionEffect(emotion) {
  document.body.setAttribute('data-mood', emotion);
}
CSS

/* El tablero cambia según la emoción */
[data-mood="tension"] {
  --ambient-color: #8b0000;
  --pulse-speed: 0.5s;
  animation: tension-pulse 0.5s ease-in-out 3;
}

@keyframes tension-pulse {
  0%, 100% { 
    box-shadow: inset 0 0 100px rgba(139, 0, 0, 0);
  }
  50% { 
    box-shadow: inset 0 0 100px rgba(139, 0, 0, 0.3);
  }
}

[data-mood="epic"] {
  --ambient-color: #ffd700;
}

[data-mood="epic"] .map-container {
  animation: epic-glow 2s ease-out;
}

@keyframes epic-glow {
  0% {
    filter: brightness(1) saturate(1);
  }
  30% {
    filter: brightness(1.5) saturate(1.3);
    transform: scale(1.02);
  }
  100% {
    filter: brightness(1) saturate(1);
    transform: scale(1);
  }
}

[data-mood="death"] .map-container {
  animation: death-desaturate 3s ease-out forwards;
}

@keyframes death-desaturate {
  0% { filter: grayscale(0) brightness(1); }
  50% { filter: grayscale(0.8) brightness(0.7); }
  100% { filter: grayscale(0.3) brightness(0.9); }
}

### PARTE E: FICHAS CON MICROANIMACIONES
👤 "LIVING PORTRAITS"
CSS

.character-card {
  position: relative;
  overflow: hidden;
  /* Asignar desde backend: style="--blink-delay: 2.5s" para evitar parpadeo sincronizado */
  --blink-delay: 0s; 
}

/* Efecto de respiración en el retrato */
.character-portrait {
  animation: portrait-breathe 4s ease-in-out infinite;
  /* Desfase para que no respiren todos igual */
  animation-delay: var(--breath-delay, 0s); 
}

@keyframes portrait-breathe {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.015); }
}

/* Parpadeo sutil cada X segundos */
.character-eyes {
  animation: blink 6s ease-in-out infinite;
  animation-delay: var(--blink-delay);
}

@keyframes blink {
  0%, 96%, 100% { transform: scaleY(1); }
  98% { transform: scaleY(0.1); }
}

/* Hover = el personaje "despierta" */
.character-card:hover .character-portrait {
  animation: portrait-alert 0.3s ease-out forwards;
  filter: brightness(1.1);
}

@keyframes portrait-alert {
  0% { transform: scale(1); }
  50% { transform: scale(1.05); }
  100% { transform: scale(1.02); }
}

/* TRAUMA INDICATOR - pulso rojo sutil */
.trauma-marker {
  position: absolute;
  width: 8px;
  height: 8px;
  background: #660000;
  border-radius: 50%;
  animation: trauma-pulse 2s ease-in-out infinite;
  z-index: 10;
}

@keyframes trauma-pulse {
  0%, 100% { 
    box-shadow: 0 0 0 0 rgba(102, 0, 0, 0.7);
    transform: scale(1);
  }
  50% { 
    box-shadow: 0 0 0 8px rgba(102, 0, 0, 0);
    transform: scale(1.1);
  }
}

### PARTE F: PARTÍCULAS AMBIENTALES
JavaScript

// Sistema de partículas según ambiente
class AmbientParticles {
  constructor(type) {
    this.types = {
      forest: { 
        sprite: '🍃', 
        count: 20, 
        speed: 'slow',
        direction: 'diagonal'
      },
      dungeon: { 
        sprite: '✦', 
        count: 30, 
        speed: 'float',
        direction: 'up',
        opacity: 0.3
      },
      snow: { 
        sprite: '❄', 
        count: 50, 
        speed: 'medium',
        direction: 'down'
      },
      fire: { 
        sprite: '🔥', 
        count: 15, 
        speed: 'fast',
        direction: 'up',
        glow: true
      },
      rain: {
        sprite: '│',
        count: 100,
        speed: 'fast',
        direction: 'down-diagonal'
      }
    };
  }
}
CSS

/* Partícula base 
   Optimization: Z-Index 50 (above map, below UI)
   Optimization: will-change for GPU hints
*/
.particle {
  position: fixed;
  pointer-events: none;
  opacity: 0.7;
  z-index: 50; 
  will-change: transform;
  animation: particle-float var(--duration) linear infinite;
  animation-delay: var(--delay);
}

/* Lluvia */
.particle-rain {
  color: rgba(150, 180, 255, 0.4);
  font-size: 20px;
  animation: rain-fall 0.8s linear infinite;
}

/* Optimization: translate3d for GPU acceleration */
@keyframes rain-fall {
  from { 
    transform: translate3d(0, -100vh, 0); 
    opacity: 0;
  }
  10% { opacity: 0.5; }
  90% { opacity: 0.5; }
  to { 
    transform: translate3d(-50px, 100vh, 0); 
    opacity: 0;
  }
}

/* Hojas cayendo */
.particle-leaf {
  animation: 
    leaf-fall 8s linear infinite,
    leaf-sway 2s ease-in-out infinite;
}

/* Optimization: translate3d for GPU acceleration */
@keyframes leaf-fall {
  from { transform: translate3d(0, -10vh, 0); }
  to { transform: translate3d(0, 110vh, 0); }
}

@keyframes leaf-sway {
  0%, 100% { margin-left: -30px; rotate: -15deg; }
  50% { margin-left: 30px; rotate: 15deg; }
}

### PARTE G: EL "PERGAMINO REVEAL"
El mapa se despliega como un pergamino real:

CSS

.scroll-map {
  --unroll: 0%;
  clip-path: inset(0 var(--unroll) 0 var(--unroll));
  transition: clip-path 1.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.scroll-map.revealed {
  --unroll: 0%;
}

.scroll-map::before,
.scroll-map::after {
  content: '';
  position: absolute;
  width: 30px;
  height: 100%;
  background: 
    linear-gradient(90deg, 
      #3d2817 0%, 
      #5c3d2a 50%, 
      #3d2817 100%);
  border-radius: 50%;
  box-shadow: inset 0 0 10px rgba(0,0,0,0.5);
  transition: transform 1.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.scroll-map::before { left: 0; }
.scroll-map::after { right: 0; }

/* Al revelar, los "rollos" se mueven a los lados */
.scroll-map.revealed::before { transform: translateX(-100%); }
.scroll-map.revealed::after { transform: translateX(100%); }
