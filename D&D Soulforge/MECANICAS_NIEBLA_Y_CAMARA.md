# FOG OF WAR ACUARELA
"The Veil of Unknown"

### CONCEPTO CENTRAL
No es una niebla genérica negra. Es tinta viva que reacciona, se disuelve como acuarela mojada, y deja "manchas de memoria" donde ya exploraron.

text

┌─────────────────────────────────────────────────────────┐
│  ░░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░  │
│  ░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███          ███▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓██   REVELADO    ██▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓██    (acuarela)   ██▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓██   disuelta    ██▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███          ███▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│  ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████████████▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  │
│                                                         │
│  ░ = Zona olvidada (sepia)    ▓ = Niebla activa        │
│  █ = Borde difuminado         [vacío] = Explorado      │
└─────────────────────────────────────────────────────────┘

### PARTE A: LA NIEBLA BASE (SVG + CSS)
Textura de Niebla Orgánica
HTML

<svg class="fog-layer" viewBox="0 0 1000 1000">
  <defs>
    <!-- Filtro de turbulencia para textura orgánica -->
    <filter id="fog-texture" x="-50%" y="-50%" width="200%" height="200%">
      <feTurbulence 
        type="fractalNoise" 
        baseFrequency="0.015" 
        numOctaves="4" 
        seed="15"
        result="noise"
      >
        <animate 
          attributeName="seed" 
          values="1;100;1" 
          dur="60s" 
          repeatCount="indefinite"
        />
      </feTurbulence>
      
      <feDisplacementMap 
        in="SourceGraphic" 
        in2="noise" 
        scale="30" 
        xChannelSelector="R" 
        yChannelSelector="G"
      />
      
      <feGaussianBlur stdDeviation="8" />
      
      <feColorMatrix type="matrix" values="
        0.15 0 0 0 0.05
        0 0.12 0 0 0.03
        0 0 0.1 0 0.02
        0 0 0 0.95 0
      "/>
    </filter>

    <!-- Gradiente de borde acuarela -->
    <filter id="watercolor-edge">
      <feTurbulence 
        type="turbulence" 
        baseFrequency="0.05" 
        numOctaves="3"
        result="turbulence"
      />
      <feDisplacementMap 
        in="SourceGraphic" 
        in2="turbulence" 
        scale="15"
      />
      <feGaussianBlur stdDeviation="2" />
    </filter>

    <!-- Máscara de revelación circular -->
    <mask id="reveal-mask">
      <rect width="100%" height="100%" fill="white"/>
      <circle 
        class="reveal-circle" 
        cx="500" cy="500" r="0" 
        fill="black"
        filter="url(#watercolor-edge)"
      />
    </mask>
  </defs>

  <!-- Capa de niebla principal -->
  <rect 
    class="fog-main"
    width="100%" 
    height="100%" 
    fill="#1a1510"
    filter="url(#fog-texture)"
    mask="url(#reveal-mask)"
  />
</svg>

### PARTE B: ANIMACIÓN DE DISOLUCIÓN
El Efecto "Ink Dissolve"
CSS

/* Capa de niebla principal */
.fog-layer {
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 100;
  mix-blend-mode: multiply;
}

/* Círculo de revelación base */
.reveal-circle {
  transform-origin: center;
  transition: r 1.5s cubic-bezier(0.23, 1, 0.32, 1);
}

/* Cuando se activa la revelación */
.reveal-circle.revealing {
  animation: watercolor-dissolve 2s ease-out forwards;
}

@keyframes watercolor-dissolve {
  0% {
    r: 0;
    opacity: 1;
    filter: url(#watercolor-edge) blur(0px);
  }
  30% {
    filter: url(#watercolor-edge) blur(3px);
  }
  60% {
    filter: url(#watercolor-edge) blur(5px);
    opacity: 0.8;
  }
  100% {
    r: var(--reveal-radius, 150);
    opacity: 0;
    filter: url(#watercolor-edge) blur(8px);
  }
}

/* Efecto de borde "mojado" que gotea */
.reveal-circle::after {
  content: '';
  position: absolute;
  inset: -20px;
  background: radial-gradient(
    circle,
    transparent 60%,
    rgba(26, 21, 16, 0.3) 70%,
    rgba(26, 21, 16, 0.6) 85%,
    rgba(26, 21, 16, 0.9) 100%
  );
  animation: edge-drip 3s ease-out infinite;
}

@keyframes edge-drip {
  0%, 100% {
    transform: scale(1);
    filter: blur(5px);
  }
  50% {
    transform: scale(1.02);
    filter: blur(7px);
  }
}

### PARTE C: SISTEMA JAVASCRIPT DE REVELACIÓN
JavaScript

class WatercolorFog {
  constructor(mapElement) {
    this.map = mapElement;
    this.revealedAreas = [];
    this.fogCanvas = this.createFogCanvas();
    this.ctx = this.fogCanvas.getContext('2d');
    
    // Configuración de estilo acuarela
    this.watercolorConfig = {
      baseColor: { r: 26, g: 21, b: 16, a: 0.95 },
      edgeVariation: 30,
      dissolveSpeed: 0.02,
      dripsCount: 8,
      memoryFade: 0.15  // Opacidad de "zona olvidada"
    };
    
    this.init();
  }

  init() {
    this.drawInitialFog();
    this.startAmbientAnimation();
  }

  // Dibujar niebla inicial con textura
  drawInitialFog() {
    const { width, height } = this.fogCanvas;
    
    // Capa base
    this.ctx.fillStyle = this.rgbaString(this.watercolorConfig.baseColor);
    this.ctx.fillRect(0, 0, width, height);
    
    // Añadir textura de ruido
    this.addNoiseTexture();
  }

  // Textura de papel/pergamino
  addNoiseTexture() {
    const imageData = this.ctx.getImageData(
      0, 0, 
      this.fogCanvas.width, 
      this.fogCanvas.height
    );
    const data = imageData.data;
    
    for (let i = 0; i < data.length; i += 4) {
      const noise = (Math.random() - 0.5) * 20;
      data[i] += noise;     // R
      data[i + 1] += noise; // G
      data[i + 2] += noise; // B
    }
    
    this.ctx.putImageData(imageData, 0, 0);
  }

  // ⭐ REVELACIÓN ACUARELA
  revealArea(x, y, radius, type = 'normal') {
    const revealData = {
      x, y, radius,
      progress: 0,
      drips: this.generateDrips(x, y, radius),
      type
    };

    this.animateReveal(revealData);
  }

  // Generar "goteos" de tinta aleatorios
  generateDrips(x, y, radius) {
    const drips = [];
    const count = this.watercolorConfig.dripsCount;
    
    for (let i = 0; i < count; i++) {
      const angle = (Math.PI * 2 / count) * i + Math.random() * 0.5;
      const length = radius * (0.3 + Math.random() * 0.4);
      
      drips.push({
        angle,
        length,
        width: 3 + Math.random() * 8,
        speed: 0.5 + Math.random() * 0.5,
        progress: 0
      });
    }
    
    return drips;
  }

  // Animación frame-by-frame
  animateReveal(revealData) {
    const animate = () => {
      revealData.progress += this.watercolorConfig.dissolveSpeed;
      
      if (revealData.progress >= 1) {
        this.finalizeReveal(revealData);
        return;
      }
      
      this.drawRevealFrame(revealData);
      requestAnimationFrame(animate);
    };
    
    animate();
  }

  // Dibujar cada frame de la revelación
  drawRevealFrame(revealData) {
    const { x, y, radius, progress, drips } = revealData;
    
    this.ctx.save();
    this.ctx.globalCompositeOperation = 'destination-out';
    
    // Círculo principal con borde irregular
    const currentRadius = radius * this.easeOutCubic(progress);
    
    // Crear gradiente de borde difuso
    const gradient = this.ctx.createRadialGradient(
      x, y, currentRadius * 0.7,
      x, y, currentRadius
    );
    gradient.addColorStop(0, 'rgba(0, 0, 0, 1)');
    gradient.addColorStop(0.7, 'rgba(0, 0, 0, 0.8)');
    gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');
    
    // Dibujar forma irregular (no círculo perfecto)
    this.ctx.beginPath();
    this.drawIrregularCircle(x, y, currentRadius, 12);
    this.ctx.fillStyle = gradient;
    this.ctx.fill();
    
    // Dibujar goteos de tinta
    drips.forEach(drip => {
      drip.progress = Math.min(drip.progress + drip.speed * 0.02, 1);
      this.drawDrip(x, y, currentRadius, drip);
    });
    
    this.ctx.restore();
  }

  // Círculo con bordes irregulares (efecto acuarela)
  drawIrregularCircle(x, y, radius, points) {
    const variation = this.watercolorConfig.edgeVariation;
    
    for (let i = 0; i <= points; i++) {
      const angle = (Math.PI * 2 / points) * i;
      const randomRadius = radius + (Math.random() - 0.5) * variation;
      const px = x + Math.cos(angle) * randomRadius;
      const py = y + Math.sin(angle) * randomRadius;
      
      if (i === 0) {
        this.ctx.moveTo(px, py);
      } else {
        // Curva bezier para suavidad
        const cp1x = x + Math.cos(angle - 0.2) * randomRadius * 1.1;
        const cp1y = y + Math.sin(angle - 0.2) * randomRadius * 1.1;
        this.ctx.quadraticCurveTo(cp1x, cp1y, px, py);
      }
    }
    this.ctx.closePath();
  }

  // Goteo de tinta
  drawDrip(originX, originY, baseRadius, drip) {
    const startX = originX + Math.cos(drip.angle) * baseRadius;
    const startY = originY + Math.sin(drip.angle) * baseRadius;
    const endX = startX + Math.cos(drip.angle) * drip.length * drip.progress;
    const endY = startY + Math.sin(drip.angle) * drip.length * drip.progress;
    
    const gradient = this.ctx.createLinearGradient(startX, startY, endX, endY);
    gradient.addColorStop(0, 'rgba(0, 0, 0, 0.8)');
    gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');
    
    this.ctx.beginPath();
    this.ctx.moveTo(startX, startY);
    
    // Curva ondulada para el goteo
    const midX = (startX + endX) / 2 + (Math.random() - 0.5) * 10;
    const midY = (startY + endY) / 2 + (Math.random() - 0.5) * 10;
    
    this.ctx.quadraticCurveTo(midX, midY, endX, endY);
    this.ctx.lineWidth = drip.width * (1 - drip.progress * 0.5);
    this.ctx.strokeStyle = gradient;
    this.ctx.lineCap = 'round';
    this.ctx.stroke();
  }

  // Easing suave
  easeOutCubic(t) {
    return 1 - Math.pow(1 - t, 3);
  }

  rgbaString({ r, g, b, a }) {
    return `rgba(${r}, ${g}, ${b}, ${a})`;
  }
}

// ⭐ USO
const fog = new WatercolorFog(document.querySelector('.map-container'));

// Cuando el personaje se mueve
playerToken.addEventListener('move', (e) => {
  fog.revealArea(e.x, e.y, 120, 'normal');
});

// Revelación épica (antorcha, hechizo de luz)
function epicReveal(x, y) {
  fog.revealArea(x, y, 250, 'epic');
}

### PARTE D: ZONAS DE MEMORIA (Lo que ya vieron pero no ven ahora)
CSS

/* Zona que fue revelada pero ya no tiene visión directa */
.memory-zone {
  position: absolute;
  background: rgba(139, 119, 101, 0.4);
  filter: 
    sepia(0.5) 
    saturate(0.6) 
    blur(1px);
  animation: memory-pulse 8s ease-in-out infinite;
}

@keyframes memory-pulse {
  0%, 100% {
    opacity: 0.4;
    filter: sepia(0.5) saturate(0.6) blur(1px);
  }
  50% {
    opacity: 0.5;
    filter: sepia(0.6) saturate(0.5) blur(2px);
  }
}

/* Efecto de "recuerdo borroso" */
.memory-zone::before {
  content: '';
  position: absolute;
  inset: 0;
  background: url('data:image/svg+xml,...') repeat;
  opacity: 0.1;
  mix-blend-mode: overlay;
  animation: memory-static 0.5s steps(4) infinite;
}

@keyframes memory-static {
  0% { transform: translate(0, 0); }
  25% { transform: translate(-1px, 1px); }
  50% { transform: translate(1px, -1px); }
  75% { transform: translate(-1px, -1px); }
  100% { transform: translate(1px, 1px); }
}

# 🎥 SISTEMA 2: ZOOM DRAMÁTICO
"The Director's Eye"

### CONCEPTO CENTRAL
Una cámara virtual cinematográfica que el DM controla para crear momentos épicos. No es solo zoom, es un sistema completo de dirección visual.

text

┌──────────────────────────────────────────────────────────────┐
│                                                              │
│    MODOS DE CÁMARA:                                          │
│                                                              │
│    [1] OVERVIEW    - Vista general del mapa                  │
│    [2] FOCUS       - Centrado en un personaje/objeto         │
│    [3] DRAMATIC    - Zoom lento + viñeta + desaturación      │
│    [4] COMBAT      - Seguimiento dinámico de acción          │
│    [5] REVEAL      - Paneo cinematográfico a nuevo lugar     │
│    [6] FLASHBACK   - Efecto sepia + bordes quemados          │
│                                                              │
└──────────────────────────────────────────────────────────────┘

### PARTE A: SISTEMA DE CÁMARA BASE
JavaScript

class CinematicCamera {
  constructor(viewport) {
    this.viewport = viewport;
    this.container = viewport.querySelector('.map-container');
    
    this.state = {
      x: 0,
      y: 0,
      zoom: 1,
      rotation: 0,
      mode: 'overview'
    };
    
    this.target = { ...this.state };
    
    this.config = {
      smoothness: 0.08,        // Suavidad de movimiento
      zoomLimits: [0.5, 4],    // Min/Max zoom
      dramaPauseDuration: 500, // Pausa antes de momento dramático
    };

    this.effects = new CameraEffects(viewport);
    this.startRenderLoop();
  }

  // Loop de renderizado suave
  startRenderLoop() {
    const render = () => {
      this.interpolateToTarget();
      this.applyTransform();
      requestAnimationFrame(render);
    };
    render();
  }

  // Interpolación suave hacia el objetivo
  interpolateToTarget() {
    const lerp = (current, target, factor) => {
      return current + (target - current) * factor;
    };

    this.state.x = lerp(this.state.x, this.target.x, this.config.smoothness);
    this.state.y = lerp(this.state.y, this.target.y, this.config.smoothness);
    this.state.zoom = lerp(this.state.zoom, this.target.zoom, this.config.smoothness);
    this.state.rotation = lerp(this.state.rotation, this.target.rotation, this.config.smoothness);
  }

  // Aplicar transformación CSS
  applyTransform() {
    const { x, y, zoom, rotation } = this.state;
    
    this.container.style.transform = `
      translate(${-x}px, ${-y}px) 
      scale(${zoom}) 
      rotate(${rotation}deg)
    `;
  }

  // ═══════════════════════════════════════
  // MODOS DE CÁMARA
  // ═══════════════════════════════════════

  // [1] OVERVIEW - Vista general
  overview(duration = 1000) {
    this.setMode('overview');
    this.effects.clear();
    
    this.animateTo({
      x: 0,
      y: 0,
      zoom: 1,
      rotation: 0
    }, duration, 'easeOutCubic');
  }

  // [2] FOCUS - Centrar en elemento
  focus(element, zoomLevel = 2, duration = 800) {
    this.setMode('focus');
    
    const rect = element.getBoundingClientRect();
    const containerRect = this.container.getBoundingClientRect();
    
    const targetX = (rect.left + rect.width / 2) - (containerRect.width / 2);
    const targetY = (rect.top + rect.height / 2) - (containerRect.height / 2);
    
    this.animateTo({
      x: targetX,
      y: targetY,
      zoom: zoomLevel,
      rotation: 0
    }, duration, 'easeOutQuart');
    
    // Resaltar elemento
    element.classList.add('camera-focused');
    this.effects.spotlight(element);
  }

  // [3] DRAMATIC ZOOM - Momento épico
  async dramaticZoom(element, options = {}) {
    const {
      initialPause = 500,
      zoomDuration = 2000,
      holdDuration = 1500,
      zoomLevel = 2.5,
      vignette = true,
      desaturate = true,
      slowMotion = true
    } = options;

    this.setMode('dramatic');

    // Paso 1: Pausa dramática
    if (slowMotion) {
      this.effects.slowMotion(true);
    }
    
    await this.wait(initialPause);

    // Paso 2: Aplicar efectos visuales
    if (vignette) {
      this.effects.vignette(true, 0.7);
    }
    
    if (desaturate) {
      this.effects.desaturate(0.4);
    }

    // Paso 3: Zoom lento
    await this.animateTo({
      x: this.getElementCenter(element).x,
      y: this.getElementCenter(element).y,
      zoom: zoomLevel,
      rotation: 0
    }, zoomDuration, 'easeInOutQuart');

    // Paso 4: Mantener el momento
    await this.wait(holdDuration);

    // Paso 5: Restaurar
    this.effects.clear(1000);
    this.overview(1500);
  }

  // [4] COMBAT CAMERA - Seguimiento de combate
  combatMode(combatants) {
    this.setMode('combat');
    
    // Calcular bounding box de todos los combatientes
    const bounds = this.calculateBounds(combatants);
    const padding = 100;
    
    // Zoom para que quepan todos
    const requiredZoom = Math.min(
      this.viewport.clientWidth / (bounds.width + padding * 2),
      this.viewport.clientHeight / (bounds.height + padding * 2),
      2 // máximo zoom
    );

    this.animateTo({
      x: bounds.centerX,
      y: bounds.centerY,
      zoom: Math.max(requiredZoom, 0.8),
      rotation: 0
    }, 600, 'easeOutQuart');

    // Efecto de tensión
    this.effects.combatTension(true);
  }

  // Cuando un combatiente ataca
  combatAction(attacker, target, type = 'attack') {
    const actionEffects = {
      attack: () => this.combatAttackShot(attacker, target),
      critical: () => this.combatCriticalShot(attacker, target),
      spell: () => this.combatSpellShot(attacker, target),
      death: () => this.combatDeathShot(target)
    };

    actionEffects[type]?.();
  }

  async combatAttackShot(attacker, target) {
    // Zoom rápido al atacante
    await this.animateTo({
      ...this.getElementCenter(attacker),
      zoom: 2.2,
    }, 300, 'easeOutQuart');

    // Shake pequeño
    this.effects.shake(5, 200);

    // Paneo rápido al objetivo
    await this.wait(150);
    await this.animateTo({
      ...this.getElementCenter(target),
      zoom: 2.0,
    }, 250, 'easeOutQuart');

    // Volver a vista de combate
    await this.wait(400);
  }

  async combatCriticalShot(attacker, target) {
    // ¡CRÍTICO! - Efecto especial
    
    // Flash blanco
    this.effects.flash('white', 100);
    
    // Zoom extremo + slowmo
    this.effects.slowMotion(true);
    
    await this.animateTo({
      ...this.getElementCenter(target),
      zoom: 3,
    }, 800, 'easeOutQuart');

    // Shake fuerte
    this.effects.shake(15, 400);
    
    // Efecto de impacto radial
    this.effects.impactRing(target);

    await this.wait(600);
    this.effects.slowMotion(false);
  }

  // [5] REVEAL - Revelar nueva ubicación
  async revealPan(fromElement, toElement, options = {}) {
    const {
      duration = 3000,
      style = 'cinematic', // 'cinematic', 'quick', 'mysterious'
      narration = null
    } = options;

    this.setMode('reveal');

    const styles = {
      cinematic: {
        curve: 'easeInOutQuart',
        initialZoom: 1.5,
        travelZoom: 0.9,
        finalZoom: 1.8,
        rotation: 0
      },
      mysterious: {
        curve: 'easeInOutSine',
        initialZoom: 2,
        travelZoom: 0.7,
        finalZoom: 2.2,
        rotation: -2,
        effects: ['fog', 'desaturate']
      },
      quick: {
        curve: 'easeOutCubic',
        initialZoom: 1.2,
        travelZoom: 1,
        finalZoom: 1.5,
        rotation: 0
      }
    };

    const style_config = styles[style];

    // Efectos según estilo
    if (style === 'mysterious') {
      this.effects.vignette(true, 0.8);
      this.effects.desaturate(0.5);
    }

    // Fase 1: Enfocar origen
    await this.animateTo({
      ...this.getElementCenter(fromElement),
      zoom: style_config.initialZoom
    }, duration * 0.2, style_config.curve);

    // Fase 2: Zoom out durante el viaje
    const midpoint = {
      x: (this.getElementCenter(fromElement).x + this.getElementCenter(toElement).x) / 2,
      y: (this.getElementCenter(fromElement).y + this.getElementCenter(toElement).y) / 2
    };

    await this.animateTo({
      ...midpoint,
      zoom: style_config.travelZoom,
      rotation: style_config.rotation
    }, duration * 0.4, style_config.curve);

    // Fase 3: Zoom in al destino
    await this.animateTo({
      ...this.getElementCenter(toElement),
      zoom: style_config.finalZoom,
      rotation: 0
    }, duration * 0.4, style_config.curve);

    // Limpiar efectos
    if (style === 'mysterious') {
      await this.wait(1000);
      this.effects.clear(800);
    }
  }

  // [6] FLASHBACK - Recuerdo
  async flashback(element, duration = 3000) {
    this.setMode('flashback');

    // Efecto de transición a flashback
    this.effects.flashbackTransition();
    
    await this.wait(500);

    // Aplicar estilo flashback
    this.effects.sepia(0.9);
    this.effects.vignette(true, 0.9);
    this.effects.burnedEdges(true);
    this.effects.filmGrain(true);

    // Zoom lento
    await this.animateTo({
      ...this.getElementCenter(element),
      zoom: 1.8
    }, duration, 'easeInOutSine');
  }

  exitFlashback() {
    this.effects.flashbackExit();
    this.effects.clear(800);
    this.overview(1000);
  }

  // ═══════════════════════════════════════
  // UTILIDADES
  // ═══════════════════════════════════════

  animateTo(target, duration, easing) {
    return new Promise(resolve => {
      const startState = { ...this.state };
      const startTime = performance.now();
      
      const animate = (currentTime) => {
        const elapsed = currentTime - startTime;
        const progress = Math.min(elapsed / duration, 1);
        const easedProgress = this.ease(progress, easing);
        
        this.target.x = startState.x + (target.x - startState.x) * easedProgress;
        this.target.y = startState.y + (target.y - startState.y) * easedProgress;
        this.target.zoom = startState.zoom + ((target.zoom || startState.zoom) - startState.zoom) * easedProgress;
        this.target.rotation = startState.rotation + ((target.rotation || 0) - startState.rotation) * easedProgress;
        
        if (progress < 1) {
          requestAnimationFrame(animate);
        } else {
          resolve();
        }
      };
      
      requestAnimationFrame(animate);
    });
  }

  ease(t, type) {
    const easings = {
      linear: t => t,
      easeOutCubic: t => 1 - Math.pow(1 - t, 3),
      easeOutQuart: t => 1 - Math.pow(1 - t, 4),
      easeInOutQuart: t => t < 0.5 ? 8 * t * t * t * t : 1 - Math.pow(-2 * t + 2, 4) / 2,
      easeInOutSine: t => -(Math.cos(Math.PI * t) - 1) / 2
    };
    return easings[type]?.(t) ?? t;
  }

  getElementCenter(element) {
    const rect = element.getBoundingClientRect();
    const containerRect = this.container.getBoundingClientRect();
    return {
      x: (rect.left - containerRect.left) + rect.width / 2,
      y: (rect.top - containerRect.top) + rect.height / 2
    };
  }

  wait(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  setMode(mode) {
    this.state.mode = mode;
    this.viewport.setAttribute('data-camera-mode', mode);
  }
}

### PARTE B: EFECTOS DE CÁMARA
JavaScript

class CameraEffects {
  constructor(viewport) {
    this.viewport = viewport;
    this.effectLayers = this.createEffectLayers();
  }

  createEffectLayers() {
    const layers = {
      vignette: this.createLayer('vignette'),
      flash: this.createLayer('flash'),
      overlay: this.createLayer('overlay'),
      grain: this.createLayer('grain'),
      shake: this.createLayer('shake')
    };
    
    Object.values(layers).forEach(layer => {
      this.viewport.appendChild(layer);
    });
    
    return layers;
  }

  createLayer(name) {
    const layer = document.createElement('div');
    layer.className = `effect-layer effect-${name}`;
    layer.style.cssText = `
      position: absolute;
      inset: 0;
      pointer-events: none;
      z-index: 1000;
      opacity: 0;
      transition: opacity 0.3s ease;
    `;
    return layer;
  }

  // Viñeta cinematográfica
  vignette(active, intensity = 0.6) {
    const layer = this.effectLayers.vignette;
    
    if (active) {
      layer.style.background = `
        radial-gradient(
          ellipse at center,
          transparent 20%,
          rgba(0, 0, 0, ${intensity * 0.3}) 50%,
          rgba(0, 0, 0, ${intensity * 0.7}) 80%,
          rgba(0, 0, 0, ${intensity}) 100%
        )
      `;
      layer.style.opacity = '1';
    } else {
      layer.style.opacity = '0';
    }
  }

  // Desaturación
  desaturate(amount) {
    this.viewport.style.filter = `saturate(${1 - amount})`;
  }

  // Sepia (para flashbacks)
  sepia(amount) {
    this.viewport.style.filter = `sepia(${amount}) saturate(0.8) contrast(1.1)`;
  }

  // Flash de impacto
  flash(color, duration) {
    const layer = this.effectLayers.flash;
    layer.style.background = color;
    layer.style.opacity = '0.8';
    
    setTimeout(() => {
      layer.style.opacity = '0';
    }, duration);
  }

  // Screen shake
  shake(intensity, duration) {
    const container = this.viewport.querySelector('.map-container');
    const startTime = performance.now();
    
    const shakeFrame = () => {
      const elapsed = performance.now() - startTime;
      const progress = elapsed / duration;
      
      if (progress >= 1) {
        container.style.transform = container.style.transform.replace(/translate\([^)]+\)\s*/, '');
        return;
      }
      
      const decay = 1 - progress;
      const offsetX = (Math.random() - 0.5) * intensity * decay;
      const offsetY = (Math.random() - 0.5) * intensity * decay;
      
      // Añadir shake al transform existente
      const currentTransform = container.style.transform || '';
      const baseTransform = currentTransform.replace(/translate\([^)]+\)\s*/, '');
      container.style.transform = `translate(${offsetX}px, ${offsetY}px) ${baseTransform}`;
      
      requestAnimationFrame(shakeFrame);
    };
    
    requestAnimationFrame(shakeFrame);
  }

  // Slow motion visual
  slowMotion(active) {
    this.viewport.classList.toggle('slow-motion', active);
  }

  // Anillo de impacto
  impactRing(element) {
    const ring = document.createElement('div');
    ring.className = 'impact-ring';
    
    const rect = element.getBoundingClientRect();
    ring.style.cssText = `
      position: absolute;
      left: ${rect.left + rect.width / 2}px;
      top: ${rect.top + rect.height / 2}px;
      transform: translate(-50%, -50%);
    `;
    
    this.viewport.appendChild(ring);
    
    setTimeout(() => ring.remove(), 1000);
  }

  // Tensión de combate
  combatTension(active) {
    if (active) {
      this.viewport.classList.add('combat-tension');
    } else {
      this.viewport.classList.remove('combat-tension');
    }
  }

  // Bordes quemados (flashback)
  burnedEdges(active) {
    this.viewport.classList.toggle('burned-edges', active);
  }

  // Grano de película
  filmGrain(active) {
    this.effectLayers.grain.classList.toggle('active', active);
  }

  // Transición de flashback
  flashbackTransition() {
    const flash = this.effectLayers.flash;
    flash.style.background = 'white';
    flash.style.opacity = '1';
    flash.style.transition = 'opacity 0.1s ease';
    
    setTimeout(() => {
      flash.style.transition = 'opacity 0.5s ease';
      flash.style.opacity = '0';
    }, 100);
  }

  // Limpiar todos los efectos
  clear(duration = 300) {
    this.viewport.style.transition = `filter ${duration}ms ease`;
    this.viewport.style.filter = 'none';
    
    Object.values(this.effectLayers).forEach(layer => {
      layer.style.transition = `opacity ${duration}ms ease`;
      layer.style.opacity = '0';
    });
    
    this.viewport.classList.remove(
      'slow-motion', 
      'combat-tension', 
      'burned-edges'
    );
  }
}

### PARTE C: CSS DE EFECTOS
CSS

/* ═══════════════════════════════════════ */
/* EFECTOS CINEMATOGRÁFICOS                */
/* ═══════════════════════════════════════ */

/* Slow Motion */
.slow-motion .map-container * {
  animation-duration: 3s !important;
  transition-duration: 0.8s !important;
}

.slow-motion::after {
  content: '';
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 20, 0.1);
  pointer-events: none;
}

/* Tensión de Combate */
.combat-tension {
  animation: combat-pulse 1.5s ease-in-out infinite;
}

@keyframes combat-pulse {
  0%, 100% {
    filter: saturate(1) contrast(1);
  }
  50% {
    filter: saturate(1.2) contrast(1.05);
    box-shadow: inset 0 0 50px rgba(150, 0, 0, 0.2);
  }
}

.combat-tension::before {
  content: '';
  position: absolute;
  inset: 0;
  border: 3px solid transparent;
  animation: combat-border-pulse 1s ease-in-out infinite;
  pointer-events: none;
}

@keyframes combat-border-pulse {
  0%, 100% {
    border-color: rgba(180, 0, 0, 0);
  }
  50% {
    border-color: rgba(180, 0, 0, 0.5);
  }
}

/* Anillo de Impacto */
.impact-ring {
  width: 20px;
  height: 20px;
  border: 3px solid rgba(255, 255, 255, 0.9);
  border-radius: 50%;
  animation: impact-expand 0.6s ease-out forwards;
  pointer-events: none;
}

@keyframes impact-expand {
  0% {
    width: 20px;
    height: 20px;
    opacity: 1;
    border-width: 3px;
  }
  100% {
    width: 200px;
    height: 200px;
    opacity: 0;
    border-width: 1px;
  }
}

/* Bordes Quemados (Flashback) */
.burned-edges::after {
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  box-shadow: 
    inset 0 0 100px rgba(0, 0, 0, 0.8),
    inset 0 0 200px rgba(30, 20, 10, 0.6);
  border-radius: 0;
  animation: burn-flicker 4s ease-in-out infinite;
}

@keyframes burn-flicker {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.9;
  }
  25%, 75% {
    opacity: 0.95;
  }
}

/* Grano de Película */
.effect-grain.active {
  opacity: 0.05 !important;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%' height='100%' filter='url(%23noise)'/%3E%3C/svg%3E");
  animation: grain-shift 0.1s steps(1) infinite;
}

@keyframes grain-shift {
  0% { transform: translate(0, 0); }
  25% { transform: translate(-2%, 2%); }
  50% { transform: translate(2%, -2%); }
  75% { transform: translate(-1%, -1%); }
  100% { transform: translate(1%, 1%); }
}

/* Elemento enfocado por cámara */
.camera-focused {
  position: relative;
  z-index: 100;
  animation: focus-pulse 1.5s ease-in-out infinite;
}

@keyframes focus-pulse {
  0%, 100% {
    filter: drop-shadow(0 0 10px rgba(255, 220, 100, 0.5));
  }
  50% {
    filter: drop-shadow(0 0 20px rgba(255, 220, 100, 0.8));
  }
}

/* Spotlight en elemento */
.spotlight-overlay {
  position: absolute;
  inset: 0;
  background: radial-gradient(
    circle at var(--spotlight-x, 50%) var(--spotlight-y, 50%),
    transparent 0%,
    transparent var(--spotlight-size, 100px),
    rgba(0, 0, 0, 0.7) calc(var(--spotlight-size, 100px) + 50px),
    rgba(0, 0, 0, 0.85) 100%
  );
  pointer-events: none;
  transition: all 0.5s ease;
}

/* Modos de cámara - indicadores visuales */
[data-camera-mode="dramatic"] {
  cursor: none;
}

[data-camera-mode="combat"] {
  cursor: crosshair;
}

[data-camera-mode="flashback"] {
  cursor: help;
}

### PARTE D: USO PRÁCTICO (API para el DM)
JavaScript

// Inicializar
const camera = new CinematicCamera(document.querySelector('.game-viewport'));

// ═══════════════════════════════════════
// COMANDOS PARA EL DM
// ═══════════════════════════════════════

// Ver todo el mapa
camera.overview();

// Enfocar en un personaje
const heroToken = document.querySelector('#hero-token');
camera.focus(heroToken, 2);

// ¡Momento dramático! (un jefe aparece)
const bossToken = document.querySelector('#boss-token');
camera.dramaticZoom(bossToken, {
  vignette: true,
  desaturate: true,
  zoomLevel: 2.8
});

// Combate iniciado
const combatants = document.querySelectorAll('.combatant');
camera.combatMode(combatants);

// Alguien ataca
camera.combatAction(
  document.querySelector('#player-1'),
  document.querySelector('#enemy-1'),
  'attack'
);

// ¡CRÍTICO!
camera.combatAction(
  document.querySelector('#player-1'),
  document.querySelector('#enemy-1'),
  'critical'
);

// Revelar nueva ubicación
camera.revealPan(
  document.querySelector('#tavern'),
  document.querySelector('#dark-forest'),
  { style: 'mysterious', duration: 4000 }
);

// Flashback a un recuerdo
camera.flashback(document.querySelector('#old-village'));
// ... (escena del recuerdo)
camera.exitFlashback();
