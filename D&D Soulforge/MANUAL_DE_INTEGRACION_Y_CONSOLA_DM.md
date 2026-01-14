# MANUAL DE INTEGRACIÓN Y CONSOLA DE MANDO "SOULFORGE"
## El Director de Orquesta Visual

### FILOSOFÍA DE DISEÑO: "Menos Clics, Más Narrativa"
A diferencia de VTTs tradicionales llenos de menús anidados, **Soulforge** apuesta por una interfaz HUD (Heads-Up Display) cinematográfica. El DM no "configura" el juego, lo **dirige** como una película en tiempo real.

---

### PARTE 1: LA CONSOLA DEL DM (THE OMNI- COCKPIT)
Imagina una interfaz semi-transparente, superpuesta al tablero pero que no lo obstruye. Diseño "Glassmorphism" oscuro.

#### 1. "The Quill" - Barra de Comandos Inteligente (Bottom Center)
En lugar de buscar en menús, el DM escribe. El sistema entiende el contexto.
*   **Comportamiento**: Al escribir `/`, despliega sugerencias predictivas.
*   **Sintaxis Natural**: Reconoce lenguaje natural.
    *   *Input*: "lluvia fuerte y truenos"
    *   *Acción*: Activa `particles-rain` (alta intensidad) + `flash-white` aleatorio + `sound-thunder`.
*   **Echo Text**: Lo que el DM narra aparece sutilmente como texto flotante en las pantallas de los jugadores antes de desvanecerse (Subtítulos Cinematográficos).

#### 2. "The Elemental Wheel" - Control Radial (Hold Right-Click)
Menú radial contextual que aparece donde esté el mouse.
*   **Norte (Ambiente)**: Sol, Lluvia, Nieve, Niebla.
*   **Este (Cámara)**: Focus, Pan, Reset, Cinematic View.
*   **Sur (Música/Audio)**: Tension, Battle, Tavern, Silence.
*   **Oeste (Herramientas)**: Puntero, Medir, Dibujar Niebla.

#### 3. "Director's Cut" - Gestión de Escenas (Top Right)
Panel retráctil con "Keyframes" de la sesión.
*   Botones de escena pre-preparados (ej: "Entrada a la Cueva", "Revelación del Lich").
*   Al hacer clic, ejecuta una macro compleja: *Mover tokens a posición + Cambiar luz + Reproducir música + Zoom de cámara*.

#### 4. "Entity Monitor" - Títeres en Vivo (Left Side)
Lista compacta de entidades activas.
*   **Mood Injection**: Click derecho en un NPC -> "Inject Fear". El token del NPC tiembla y emite partículas moradas.
*   **Focus Lock**: Doble clic en un jugador -> La cámara de *todos* los espectadores se fija en él.

---

### PARTE 2: MECÁNICAS COMPETITIVAS ("KILLER FEATURES")
Lo que hace a Soulforge superior a Roll20/Foundry desde el diseño.

#### 1. "Narrative Scars" (Memoria del Tablero)
El tablero recuerda lo que pasó.
*   Si hubo una bola de fuego en la casilla B4, esa zona queda "quemada" (overlay oscuro) por el resto de la sesión.
*   Si un personaje murió en C7, queda una sutil "mancha espiritual" o flores etéreas.
*   **Implementación**: Persistencia ligera en JSON de capas de decal (calcomanías) sobre el mapa base.

#### 2. "Audio-Visual Synesthesia"
El tablero reacciona al audio.
*   Cuando la música tiene picos de bajos (bombos de guerra), la pantalla vibra imperceptiblemente.
*   Cuando hay notas altas (magia feérica), las partículas brillan más.
*   **Tech**: Web Audio API analizando frecuencias y enviando variables CSS (`--audio-low`, `--audio-high`) al `root`.

#### 3. "The 4th Wall Break" (Efectos de Interfaz)
Efectos que rompen la UI del jugador para inmersión total (Sanity Effects).
*   **Terror**: La interfaz de usuario del jugador se agrieta, los textos de sus menús se vuelven borrosos o cambian a runas ilegibles por unos segundos.
*   **Ceguera**: No solo se pone negro el mapa, sino que su chat se ve borroso.

#### 4. "Smart Fog" (Niebla con IA Táctica)
La niebla no es estática.
*   Si los jugadores tardan mucho, la niebla "invade" zonas ya exploradas (simulando antorchas apagándose o miedo).
*   "Whispering Fog": En zonas de niebla, aparecen textos flotantes fugaces ("te observan", "ayuda") solo para jugadores con Percepción alta (o baja Cordura).

---

### PARTE 3: INTEGRACIÓN TÉCNICA (El Pegamento)

#### Flujo de Datos (The Loop)
1.  **DM (Input)**: Escribe `/scene ancient_ruins`.
2.  **Server (Rust)**:
    *   Valida comando.
    *   Busca preset `ancient_ruins`.
    *   Emite evento `SceneUpdate` { light: '#2a2a40', particles: 'dust', music: 'track_ruins' }.
3.  **Cliente (JS/CSS)**:
    *   `LightCamera` inicia transición de zoom.
    *   `AmbientParticles` limpia partículas viejas e instancia 'dust'.
    *   `OptimizedFog` recalcula visibilidad basada en nuevas posiciones de luz.
    *   **GPU**: Todo se anima via `transform`/`opacity` sin tocar el layout.

---

### PARTE 4: SIMULACIÓN DE ESCENA - "LA SOMBRA DEL WYRM"
*Así se ve y se siente una sesión finalizada en Soulforge.*

**Contexto**: Los jugadores entran a una caverna donde duerme un dragón antiguo.

**PASO 1: LA ENTRADA (Setting the Stage)**
*   **DM**: Usa "The Quill" y escribe: `/ambience cave_cold`.
*   **Visual**:
    *   La pantalla se tiñe de azules grises.
    *   Partículas `drip` (gotas) caen ocasionalmente.
    *   Sonido de viento hueco.
    *   **Niebla Acuarela**: Los bordes de visión son irregulares y palpitantes.

**PASO 2: EL DESCUBRIMIENTO (Cinematic Reveal)**
*   **DM**: Selecciona al Jugador 1, presiona SHIFT + Click en el centro del mapa (Punto de Interés).
*   **Cámara**: Ejecuta comando `camera.revealPan()`.
    *   La cámara de TODOS los jugadores se despega de sus personajes.
    *   Viaja suavemente por la oscuridad "acuarela".
    *   La niebla se disuelve dramáticamente al llegar al centro.
    *   **Flash Reveal**: Un rayo de luz ilumina escamas doradas.
*   **Narrativa**: Texto flotante: *"Algo brilla en la oscuridad..."*

**PASO 3: EL DESPERTAR (Audio-Visual Sync)**
*   **DM**: Activa mood `/mood danger`.
*   **Efecto**:
    *   La música cambia a tambores profundos.
    *   La luz ambiental pulsa al ritmo de los tambores (Synesthesia).
    *   El mapa hace un `screen-shake` micro con cada "golpe" de tambor.
    *   El Dragón (Token) activa animación `breathing` (se infla y desinfla).

**PASO 4: COMBATE (High Performance Mode)**
*   **DM**: `/combat start`.
*   **Sistema**:
    *   UI de exploración desaparece, entra UI de combate minimalista.
    *   Activa `CombatFocus`: Fondo ligeramente desenfocado, tokens resaltados.
    *   Jugador lanza "Bola de Fuego".
    *   **Efecto**: Partículas de explosión (GPU). Cámara hace zoom rápido in-out. El suelo queda con una mancha negra ("Narrative Scar").

**PASO 5: EL TERROR (4th Wall)**
*   **DM**: El dragón usa "Rugido Aterrador".
*   **DM Action**: Click derecho en Dragón -> "Roar".
*   **Visual**:
    *   Onda expansiva distorsiona el mapa (filtro de desplazamiento SVG).
    *   La ventada de chat de los jugadores se "rompe" (CSS clip-path fracturado).
    *   La pantalla se desatura a casi blanco y negro por 3 segundos.

### RESUMEN DE CÓDIGO CLAVE PARA LA CONSOLA (ConsoleController.js)

```javascript
class SoulforgeConsole {
  constructor(socket) {
    this.socket = socket;
    this.quill = document.querySelector('#quill-input');
    this.setupCommmands();
  }

  // El cerebro del Quill
  processCommand(input) {
    // Regex para comandos naturales
    // "clima lluvia" -> { type: 'weather', value: 'rain' }
    if (input.match(/clima|tiempo/i)) {
      const type = this.extractWeatherType(input);
      this.socket.send({ type: 'Environment', action: 'weather', value: type });
      return; // Fin
    }

    if (input.startsWith('/focus')) {
      const targetName = input.split(' ')[1]; // "/focus Aragorn"
      this.socket.send({ type: 'Camera', action: 'focus', target: targetName });
    }
    
    // Si no es comando, es narrativa
    this.socket.send({ type: 'Narrate', text: input, broadcast: true });
  }

  // Activa efectos de locura
  triggerSanityEffect(playerId, level) {
    this.socket.send({ 
      type: 'Effect', 
      target: playerId, 
      effect: 'ui_glitch', 
      intensity: level 
    });
  }
}
```

Esta integración convierte a Soulforge no en un tablero, sino en un **motor de cine interactivo**.
