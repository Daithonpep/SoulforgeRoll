// ============ DATOS DE PERSONAJES (BASADOS EN FICHAS REALES) ============
const personajesData = [
    {
        id: 1,
        nombre: "Aric Blackwood",
        clase: "Paladín Caído",
        avatar: "static/images/aric_thumb.png",
        mentira: "La divinidad nos ha abandonado; solo la fuerza bruta dicta la justicia.",
        herida: "El Silencio de Dios durante la masacre. Miedo existencial al abandono.",
        deseo: "Encontrar una causa real por la que valga la pena morir.",
        sombra: "La Ira Justiciera",
        sombraDesc: "La fe rota se convierte en fanatismo violento.",
        trauma: "El Asedio Silencioso",
        traumaDesc: "El día que rezó y nadie respondió.",
        perks: [
            { nombre: "Escudo del Mártir", tipo: "REACCIÓN", efecto: "Aliado gana +2 AC. Aric recibe mitad del daño." },
            { nombre: "Fobia al Silencio", tipo: "PASIVA", efecto: "Desventaja en Percepción si hay silencio absoluto." },
            { nombre: "Furia de la Fe Muerta", tipo: "1/DÍA", efecto: "Al caer a 0 HP, realiza ataque con Ventaja." }
        ],
        presionInicial: 75
    },
    {
        id: 2,
        nombre: "Elara Velo Nocturno",
        clase: "Tejedora de Hechizos",
        avatar: "static/images/elara_thumb.png",
        mentira: "El conocimiento vale cualquier sacrificio.",
        herida: "La Grieta de Arcanum que ella misma provocó.",
        deseo: "Controlar la magia que destruyó su hogar.",
        sombra: "Curiosidad Devoradora",
        sombraDesc: "Arriesga al grupo por obtener secretos arcanos.",
        trauma: "La Grieta de Arcanum",
        traumaDesc: "Devoró a su mentor frente a sus ojos.",
        perks: [
            { nombre: "Eco del Vacío", tipo: "PASIVA", efecto: "Puede escuchar susurros mágicos. Riesgo de locura." },
            { nombre: "Pacto Rechazado", tipo: "RESISTENCIA", efecto: "+2 Salvación vs Posesión demoníaca." }
        ],
        presionInicial: 40
    }
];

const semillasNarrativas = [
    { id: 1, nombre: "Ecos del Pasado", icono: "👻", descripcion: "Un NPC reconoce al personaje de los eventos de su trauma.", tipo: "trauma" },
    { id: 2, nombre: "Espejo Oscuro", icono: "🪞", descripcion: "Encuentro con alguien que cayó ante la misma sombra.", tipo: "sombra" },
    { id: 3, nombre: "Redención Imposible", icono: "⚖️", descripcion: "Oportunidad de arreglar el trauma original con gran costo.", tipo: "moral" },
    { id: 4, nombre: "La Pregunta Incómoda", icono: "❓", descripcion: "Un NPC confronta directamente al personaje sobre su sombra.", tipo: "confrontacion" }
];

const CONFIG_OPTS = {
    intensidad: {
        1: {
            nombre: "Brisa (Historia)",
            detalle: "Enfoque en la narrativa y exploración. <strong>Efecto en Mapa:</strong> Clima estable, rutas comerciales seguras. <strong>Mecánica:</strong> Los PJs recuperan 'Esperanza' tras cada descanso largo. La Sombra es un susurro manejable."
        },
        2: {
            nombre: "Marea (Estándar)",
            detalle: "Equilibrio entre riesgo y recompensa. <strong>Efecto en Mapa:</strong> Clima variable, caminos secundarios peligrosos. <strong>Mecánica:</strong> Los PJs enfrentarán recordatorios de su pasado cada sesión. Fallar tiradas clave genera 'Estrés'."
        },
        3: {
            nombre: "Tormenta (Desafío)",
            detalle: "Supervivencia y trauma. <strong>Efecto en Mapa:</strong> Recursos escasos, tormentas mágicas frecuentes. <strong>Mecánica:</strong> La curación es lenta. Los enemigos usarán activamente las fobias de los personajes (ver Matriz Psicológica) en combate."
        },
        4: {
            nombre: "Abismo (Hardcore)",
            detalle: "Horror cósmico y corrupción. <strong>Efecto en Mapa:</strong> La realidad se deforma, zonas de 'Vacío' intransitables. <strong>Mecánica:</strong> Sistema de Cordura activo. Muerte permanente probable. La corrupción se acelera al usar magia."
        }
    },
    modo: {
        sombra: {
            nombre: "Sombra Dominante",
            detalle: "El entorno físico refleja la oscuridad interior. <strong>Generación:</strong> Si un PJ teme al fuego, habrá volcanes activos. Si teme la soledad, habrá desiertos vacíos. Los monstruos son manifestaciones literales de los defectos de los PJs."
        },
        trauma: {
            nombre: "Trauma Resonante",
            detalle: "La historia es un ciclo. <strong>Generación:</strong> Los NPCs principales recordarán a las figuras del pasado de los PJs (Padres, mentores caídos). Las misiones obligarán a revivir los eventos del Trauma para romper el ciclo."
        },
        luz: {
            nombre: "Luz Buscada",
            detalle: "Un mundo en tinieblas donde los PJs son la única esperanza. <strong>Generación:</strong> Noches eternas o eclipses. Las fuentes de luz son zonas seguras sagradas. Los enemigos huyen de la luz o la atacan frenéticamente."
        },
        tejido: {
            nombre: "Tejido Grupal",
            detalle: "Destinos entrelazados. <strong>Generación:</strong> El mapa se divide en facciones que representan los ideales de los PJs. Misiones de lealtad obligatorias. El conflicto principal requiere que todos los PJs cooperen para ser resuelto."
        }
    }
};

let configuracionPartida = {
    personajesActivos: [],
    intensidad: null,
    modoGeneracion: null,
    hilosDestino: [],
    presionesAlma: {},
    semillasPlantadas: []
};

// ============ CLASES ============
class ParticleSystem {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.particles = [];
        this.resize();
        window.addEventListener('resize', () => this.resize());
    }
    resize() { this.canvas.width = window.innerWidth; this.canvas.height = window.innerHeight; }
    createParticle() {
        return {
            x: Math.random() * this.canvas.width, y: Math.random() * this.canvas.height,
            size: Math.random() * 2 + 0.5, speedX: (Math.random() - 0.5) * 0.5, speedY: (Math.random() - 0.5) * 0.5,
            opacity: Math.random() * 0.5 + 0.2, hue: Math.random() * 60 + 30
        };
    }
    init(count = 100) { for (let i = 0; i < count; i++) this.particles.push(this.createParticle()); this.animate(); }
    animate() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.particles.forEach(p => {
            p.x += p.speedX; p.y += p.speedY;
            if (p.x < 0) p.x = this.canvas.width; if (p.x > this.canvas.width) p.x = 0;
            if (p.y < 0) p.y = this.canvas.height; if (p.y > this.canvas.height) p.y = 0;
            this.ctx.beginPath(); this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
            this.ctx.fillStyle = `hsla(${p.hue}, 70%, 60%, ${p.opacity})`; this.ctx.fill();
        });
        requestAnimationFrame(() => this.animate());
    }
}

class Constelacion {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.nodos = [];
        this.conexiones = [];
        this.selectedNodo = null;
        this.hiloActivo = null;
        this.imagenes = {}; // Cache de imágenes cargadas
        this.resize();
        this.setupEventos();
        window.addEventListener('resize', () => this.resize());
    }

    resize() {
        if (!this.canvas.parentElement) return;
        this.canvas.width = this.canvas.parentElement.offsetWidth;
        this.canvas.height = 280; // Más alto para mejor visualización
        this.posicionarNodos();
    }

    posicionarNodos() {
        const count = configuracionPartida.personajesActivos.length;
        if (count === 0) return;

        // Distribución HORIZONTAL con espacio entre nodos
        const nodeRadius = 50; // Nodos más grandes
        const spacing = this.canvas.width / (count + 1);
        const centerY = this.canvas.height / 2;

        this.nodos = configuracionPartida.personajesActivos.map((p, i) => {
            // Precargar imagen del avatar
            if (!this.imagenes[p.id]) {
                const img = new Image();
                img.src = p.avatar;
                this.imagenes[p.id] = img;
            }
            return {
                id: p.id,
                x: spacing * (i + 1),
                y: centerY,
                radius: nodeRadius,
                personaje: p
            };
        });
    }

    // Eventos del Mouse
    setupEventos() {
        this.canvas.addEventListener('click', e => this.handleClick(e));
        // Feedback visual para borrar
        this.canvas.addEventListener('mousemove', e => {
            if (this.hiloActivo === 'borrar') {
                const rect = this.canvas.getBoundingClientRect();
                const mouse = { x: e.clientX - rect.left, y: e.clientY - rect.top };
                let hovering = false;

                for (let c of this.conexiones) {
                    if (this.distToSegment(mouse, c.from, c.to) < 10) {
                        this.canvas.style.cursor = 'crosshair';
                        hovering = true;
                        break;
                    }
                }
                if (!hovering) this.canvas.style.cursor = 'default';
            } else {
                this.canvas.style.cursor = 'default';
            }
        });
    }

    // Calcular distancia de un punto a un segmento de linea
    distToSegment(p, v, w) {
        function sqr(x) { return x * x }
        function dist2(v, w) { return sqr(v.x - w.x) + sqr(v.y - w.y) }
        var l2 = dist2(v, w);
        if (l2 == 0) return Math.sqrt(dist2(p, v));
        var t = ((p.x - v.x) * (w.x - v.x) + (p.y - v.y) * (w.y - v.y)) / l2;
        t = Math.max(0, Math.min(1, t));
        return Math.sqrt(dist2(p, { x: v.x + t * (w.x - v.x), y: v.y + t * (w.y - v.y) }));
    }

    handleClick(e) {
        const rect = this.canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const mouse = { x, y };

        // LÓGICA DE BORRADO DE HILOS
        if (this.hiloActivo === 'borrar') {
            const index = this.conexiones.findIndex(c => this.distToSegment(mouse, c.from, c.to) < 10);
            if (index !== -1) {
                const removed = this.conexiones.splice(index, 1)[0];
                // Actualizar modelo de datos
                configuracionPartida.hilosDestino = configuracionPartida.hilosDestino.filter(
                    h => !(h.desde === removed.from.personaje.id && h.hasta === removed.to.personaje.id && h.tipo === removed.tipo)
                );
                showNotification("✂️ Hilo cortado");
                actualizarResumen();
                return;
            }
        }

        const clicked = this.nodos.find(n => Math.sqrt((n.x - x) ** 2 + (n.y - y) ** 2) <= n.radius);

        if (clicked && this.hiloActivo && this.hiloActivo !== 'borrar') {
            if (!this.selectedNodo) {
                this.selectedNodo = clicked;
                showNotification("Seleccionado: " + clicked.personaje.nombre + " - Ahora selecciona otro personaje");
            } else if (this.selectedNodo.id !== clicked.id) {
                this.conexiones.push({ from: this.selectedNodo, to: clicked, tipo: this.hiloActivo });
                configuracionPartida.hilosDestino.push({
                    desde: this.selectedNodo.personaje.id,
                    hasta: clicked.personaje.id,
                    tipo: this.hiloActivo
                });
                showNotification("¡Hilo de " + this.hiloActivo + " creado!");
                this.selectedNodo = null;
                actualizarResumen();
            }
        } else if (clicked && !this.hiloActivo) {
            showNotification("Primero selecciona un tipo de hilo arriba");
        }
    }

    setHiloActivo(tipo) {
        this.hiloActivo = tipo;
        this.selectedNodo = null;
        if (tipo === 'borrar') {
            showNotification("Modo Cortar: Haz clic en un hilo para eliminarlo.");
        } else {
            showNotification("Hilo de " + tipo + " activado. Haz clic en dos personajes para conectarlos.");
        }
    }

    getColorHilo(tipo) {
        const colores = {
            conflicto: '#dc143c',
            alianza: '#1e90ff',
            destino: '#ffd700',
            secreto: '#8b008b'
        };
        return colores[tipo] || '#ffd700';
    }

    dibujar() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Dibujar conexiones (hilos)
        this.conexiones.forEach(c => {
            this.ctx.beginPath();
            this.ctx.moveTo(c.from.x, c.from.y);
            this.ctx.lineTo(c.to.x, c.to.y);
            this.ctx.strokeStyle = this.getColorHilo(c.tipo);
            this.ctx.lineWidth = 4;
            this.ctx.shadowColor = this.getColorHilo(c.tipo);
            this.ctx.shadowBlur = 10;
            this.ctx.stroke();
            this.ctx.shadowBlur = 0;
        });

        // Dibujar nodos con avatares
        this.nodos.forEach(n => {
            const isSelected = n === this.selectedNodo;

            // Borde exterior
            this.ctx.beginPath();
            this.ctx.arc(n.x, n.y, n.radius + 4, 0, Math.PI * 2);
            this.ctx.strokeStyle = isSelected ? '#ffd700' : '#b8860b';
            this.ctx.lineWidth = isSelected ? 4 : 2;
            if (isSelected) {
                this.ctx.shadowColor = '#ffd700';
                this.ctx.shadowBlur = 15;
            }
            this.ctx.stroke();
            this.ctx.shadowBlur = 0;

            // Clip circular para el avatar
            this.ctx.save();
            this.ctx.beginPath();
            this.ctx.arc(n.x, n.y, n.radius, 0, Math.PI * 2);
            this.ctx.clip();

            // Dibujar avatar
            const img = this.imagenes[n.id];
            if (img && img.complete) {
                this.ctx.drawImage(img, n.x - n.radius, n.y - n.radius, n.radius * 2, n.radius * 2);
            } else {
                // Fallback: círculo con inicial
                this.ctx.fillStyle = 'rgba(26,26,46,0.9)';
                this.ctx.fill();
                this.ctx.font = 'bold 28px Cinzel';
                this.ctx.textAlign = 'center';
                this.ctx.textBaseline = 'middle';
                this.ctx.fillStyle = '#ffd700';
                this.ctx.fillText(n.personaje.nombre.charAt(0), n.x, n.y);
            }
            this.ctx.restore();

            // Nombre debajo
            this.ctx.font = '14px Cinzel';
            this.ctx.textAlign = 'center';
            this.ctx.fillStyle = '#f4e4bc';
            this.ctx.fillText(n.personaje.nombre, n.x, n.y + n.radius + 20);

            // Clase debajo del nombre
            this.ctx.font = 'italic 11px Crimson Text';
            this.ctx.fillStyle = '#888';
            this.ctx.fillText(n.personaje.clase, n.x, n.y + n.radius + 35);
        });

        requestAnimationFrame(() => this.dibujar());
    }

    iniciar() {
        this.posicionarNodos();
        this.dibujar();
    }
}

// ============ UI LOGIC ============
function renderizarPersonajes() {
    const grid = document.getElementById('personajes-grid');
    if (!grid) return;
    grid.innerHTML = personajesData.map(p => `
        <div class="personaje-card" data-id="${p.id}">
            <img src="${p.avatar}" class="personaje-img" alt="${p.nombre}">
            <div class="personaje-info">
                <h3 class="personaje-nombre">${p.nombre}</h3>
                <div class="personaje-clase">${p.clase}</div>
                <div class="personaje-tags">
                    <span class="tag sombra">${p.sombra}</span>
                    <span class="tag trauma">${p.trauma}</span>
                </div>
            </div>
        </div>
    `).join('');

    document.querySelectorAll('.personaje-card').forEach(card => card.addEventListener('click', () => {
        card.classList.toggle('selected');
        const id = parseInt(card.dataset.id);
        const char = personajesData.find(p => p.id === id);

        if (card.classList.contains('selected')) {
            configuracionPartida.personajesActivos.push(char);
            configuracionPartida.presionesAlma[id] = char.presionInicial;
        } else {
            configuracionPartida.personajesActivos = configuracionPartida.personajesActivos.filter(p => p.id !== id);
            delete configuracionPartida.presionesAlma[id];
        }

        actualizarPresionesUI();
        actualizarForjadorUI();
        actualizarResumen();
        if (window.constelacion) window.constelacion.posicionarNodos();
    }));
}

function actualizarPresionesUI() {
    const grid = document.getElementById('presion-grid');
    if (!grid) return;
    grid.innerHTML = configuracionPartida.personajesActivos.map(p => {
        const val = configuracionPartida.presionesAlma[p.id] || 50;
        return `
            <div class="presion-personaje">
                <div class="presion-avatar-mini"><img src="${p.avatar}"></div>
                <div class="presion-info">
                    <div class="presion-nombre">${p.nombre}</div>
                    <div class="presion-barra-container"><div class="presion-marcador" style="left:${val}%" data-id="${p.id}"></div></div>
                </div>
                <div class="presion-valor"><div class="presion-porcentaje">${val}%</div></div>
            </div>`;
    }).join('');

    // DRAG FUNCTIONALITY - Permite al DM ajustar la presión arrastrando
    document.querySelectorAll('.presion-marcador').forEach(marcador => {
        marcador.addEventListener('mousedown', (e) => {
            e.preventDefault();
            const mover = (e) => {
                const barra = marcador.parentElement;
                const rect = barra.getBoundingClientRect();
                let pct = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
                marcador.style.left = pct + '%';
                marcador.closest('.presion-personaje').querySelector('.presion-porcentaje').innerText = Math.round(pct) + '%';
                configuracionPartida.presionesAlma[parseInt(marcador.dataset.id)] = Math.round(pct);
            };
            const soltar = () => {
                window.removeEventListener('mousemove', mover);
                window.removeEventListener('mouseup', soltar);
                actualizarResumen();
            };
            window.addEventListener('mousemove', mover);
            window.addEventListener('mouseup', soltar);
        });
    });
}

function actualizarForjadorUI() {
    const selector = document.getElementById('forjador-selector');
    if (!selector) return;
    selector.innerHTML = configuracionPartida.personajesActivos.map((p, i) => `
        <button class="forjador-personaje-btn ${i === 0 ? 'active' : ''}" data-id="${p.id}" onclick="mostrarDatosForjador(personajesData.find(x=>x.id==${p.id}))">
            <img src="${p.avatar}" style="width:20px; height:20px; border-radius:50%;">
            <span>${p.nombre}</span>
        </button>
    `).join('');

    if (configuracionPartida.personajesActivos.length > 0) {
        mostrarDatosForjador(configuracionPartida.personajesActivos[0]);
    } else {
        document.getElementById('forjador-datos').innerHTML = '<div style="padding:20px; opacity:0.6;">Selecciona personajes arriba para forjar sus tentaciones.</div>';
        document.getElementById('sugerencias-lista').innerHTML = '';
    }
    renderizarSemillas();
}

function mostrarDatosForjador(p) {
    const perksHtml = p.perks ? p.perks.map(perk => `
        <div class="perk-item">
            <span class="perk-nombre">${perk.nombre}</span>
            <span class="perk-tipo">${perk.tipo}</span>
            <div class="perk-efecto">${perk.efecto}</div>
        </div>
    `).join('') : '';

    document.getElementById('forjador-datos').innerHTML = `
        <div class="forjador-personaje-header">
            <img src="${p.avatar}" class="forjador-avatar">
            <div>
                <h3>${p.nombre}</h3>
                <div class="forjador-clase">${p.clase}</div>
            </div>
        </div>
        <div class="matriz-psicologica">
            <h4>Matriz Psicológica</h4>
            <div class="matriz-item mentira">
                <div class="matriz-label">La Mentira</div>
                <div class="matriz-valor">"${p.mentira}"</div>
            </div>
            <div class="matriz-item herida">
                <div class="matriz-label">La Herida</div>
                <div class="matriz-valor">${p.herida}</div>
            </div>
            <div class="matriz-item deseo">
                <div class="matriz-label">El Deseo</div>
                <div class="matriz-valor">${p.deseo}</div>
            </div>
        </div>
        <div class="sombra-trauma-section">
            <div class="dato-item sombra-dato">
                <div class="dato-label">Sombra</div>
                <div class="dato-valor">"${p.sombra}"</div>
                <div class="dato-desc">${p.sombraDesc}</div>
            </div>
            <div class="dato-item trauma-dato">
                <div class="dato-label">Trauma</div>
                <div class="dato-valor">"${p.trauma}"</div>
                <div class="dato-desc">${p.traumaDesc}</div>
            </div>
        </div>
        ${perksHtml ? '<div class="perks-section"><h4>Resonancia del Alma</h4>' + perksHtml + '</div>' : ''}
    `;
    generarSugerenciasNarrativas(p);
}

function generarSugerenciasNarrativas(p) {
    const lista = document.getElementById('sugerencias-lista');
    if (!lista) return;

    const ganchoTrama = 'El personaje busca "' + p.deseo + '", pero su herida "' + p.herida + '" lo saboteará.';
    const encuentroSombra = 'Un enemigo que encarna "' + p.sombra + '" intentará validar su mentira para corromperlo.';
    const giroNarrativo = p.id === 1
        ? "El Silencio de Dios no fue abandono, fue protección. Algo peor escuchaba."
        : "La Grieta no se abrió por accidente; el mentor de Elara la usó a ella como llave.";

    lista.innerHTML = `
        <div class="sugerencia-card">
            <div class="sugerencia-tipo">⚓ Gancho Principal</div>
            <div class="sugerencia-texto">${ganchoTrama}</div>
        </div>
        <div class="sugerencia-card">
            <div class="sugerencia-tipo">👹 Encuentro de Sombra</div>
            <div class="sugerencia-texto">${encuentroSombra}</div>
        </div>
        <div class="sugerencia-card">
            <div class="sugerencia-tipo">🌪️ Giro Potencial</div>
            <div class="sugerencia-texto">${giroNarrativo}</div>
        </div>
    `;
}

function renderizarSemillas() {
    const container = document.getElementById('semillas-container');
    if (!container) return;
    container.innerHTML = semillasNarrativas.map(s => `
        <div class="semilla-card" data-id="${s.id}">
            <div class="semilla-header"><span class="semilla-icono">${s.icono}</span><span class="semilla-nombre">${s.nombre}</span></div>
            <div class="semilla-desc">${s.descripcion}</div>
            <div class="semilla-aplicar">${configuracionPartida.personajesActivos.map(p => `
                <span class="semilla-target" data-pid="${p.id}" onclick="event.stopPropagation(); this.classList.toggle('selected'); plantSemilla(${s.id}, ${p.id})">
                    <img src="${p.avatar}" title="${p.nombre}">
                </span>`).join('')}
            </div>
        </div>
    `).join('');
}

window.plantSemilla = (sid, pid) => {
    const exists = configuracionPartida.semillasPlantadas.find(x => x.sid === sid && x.pid === pid);
    if (exists) {
        configuracionPartida.semillasPlantadas = configuracionPartida.semillasPlantadas.filter(x => !(x.sid === sid && x.pid === pid));
    } else {
        configuracionPartida.semillasPlantadas.push({ sid, pid });
    }
    actualizarResumen();
};

function actualizarResumen() {
    const set = (id, val) => { const el = document.getElementById(id); if (el) el.innerText = val; };
    set('resumen-personajes', configuracionPartida.personajesActivos.length);
    set('resumen-intensidad', configuracionPartida.intensidad ? configuracionPartida.intensidad.nombre : '-');
    set('resumen-modo', configuracionPartida.modoGeneracion ? configuracionPartida.modoGeneracion.nombre : '-');
    set('resumen-hilos', configuracionPartida.hilosDestino.length);
    set('resumen-semillas', configuracionPartida.semillasPlantadas.length);
}

function showNotification(msg) {
    const n = document.getElementById('notification');
    const t = document.getElementById('notification-text');
    if (n && t) { t.innerText = msg; n.classList.add('show'); setTimeout(() => n.classList.remove('show'), 3000); }
}

// ============ INIT ============
document.addEventListener('DOMContentLoaded', () => {
    const particlesCanvas = document.getElementById('particles-canvas');
    if (particlesCanvas) new ParticleSystem(particlesCanvas).init(50);

    renderizarPersonajes();

    // AUTO-SELECCIONAR TODOS LOS PERSONAJES AL CARGAR
    // Esto llena automáticamente todas las secciones para la demo
    setTimeout(() => {
        document.querySelectorAll('.personaje-card').forEach(card => {
            card.click(); // Simula click para seleccionar
        });
    }, 100);

    document.querySelectorAll('.intensidad-nivel').forEach(n => n.addEventListener('click', () => {
        document.querySelectorAll('.intensidad-nivel').forEach(el => el.classList.remove('active'));
        n.classList.add('active');
        const nivel = n.dataset.nivel;
        const config = CONFIG_OPTS.intensidad[nivel];
        configuracionPartida.intensidad = { nivel: nivel, nombre: config.nombre };
        const infoContainer = document.getElementById('intensidad-info') || document.querySelector('.intensidad-panel');
        if (infoContainer) {
            let info = document.getElementById('intensidad-info');
            if (!info) { info = document.createElement('div'); info.id = 'intensidad-info'; info.className = 'info-desplegable'; infoContainer.appendChild(info); }
            info.innerHTML = '<h4>' + config.nombre + '</h4><p>' + config.detalle + '</p>';
        }
        actualizarResumen();
    }));

    document.querySelectorAll('.modo-card').forEach(n => n.addEventListener('click', () => {
        document.querySelectorAll('.modo-card').forEach(el => el.classList.remove('active'));
        n.classList.add('active');
        const tipo = n.dataset.modo;
        const config = CONFIG_OPTS.modo[tipo];
        configuracionPartida.modoGeneracion = { tipo: tipo, nombre: config.nombre };
        const infoContainer = document.getElementById('generacion-info') || document.querySelector('.generacion-panel');
        if (infoContainer) {
            let info = document.getElementById('generacion-info');
            if (!info) { info = document.createElement('div'); info.id = 'generacion-info'; info.className = 'info-desplegable'; infoContainer.appendChild(info); }
            info.innerHTML = '<h4>' + config.nombre + '</h4><p>' + config.detalle + '</p>';
        }
        actualizarResumen();
        actualizarResumen();
    }));

    // El listener de .hilo-btn se ha reemplazado por onclick="seleccionarHiloUI()" en el HTML
    // Inicializar canvas si no existe
    const canvas = document.getElementById('constelacion-canvas');
    if (canvas && !window.constelacion) {
        window.constelacion = new Constelacion(canvas);
        window.constelacion.iniciar();
    }
});

const INFO_HILOS = {
    conflicto: {
        titulo: "⚔️ Conflicto (Rivalidad)",
        desc: "Los personajes tienen asuntos pendientes, deudas de sangre o ideologías opuestas. <strong>Efecto:</strong> Si ambos están en la misma escena, la dificultad de las tiradas sociales aumenta. Otorga bonos de daño si pelean entre sí."
    },
    alianza: {
        titulo: "🤝 Alianza (Lealtad)",
        desc: "Un vínculo forjado en batalla o juramento sagrado. <strong>Efecto:</strong> Pueden usar la acción 'Ayudar' como reacción (una vez por sesión). Si uno cae, el otro entra en estado de 'Furia' o 'Pánico'."
    },
    destino: {
        titulo: "⭐ Destino (Profecía)",
        desc: "Sus caminos están entrelazados por fuerzas superiores. <strong>Efecto:</strong> Comparten sueños y visiones. El Director de Juego usará este vínculo para forzar encuentros incluso si intentan separarse."
    },
    secreto: {
        titulo: "🔮 Secreto (Oculto)",
        desc: "Comparten un crimen, un amor prohibido o un conocimiento peligroso. <strong>Efecto:</strong> Solo ellos conocen este vínculo. Revelarlo públicamente causa 1d6 de Daño Mental a ambos (Vergüenza/Miedo)."
    },
    borrar: {
        titulo: "✂️ Cortar Hilo",
        desc: "Elimina un vínculo existente. <strong>Efecto:</strong> El lazo deja de tener impacto mecánico. Haz clic sobre la línea que une a dos personajes para romperla."
    }
};

function seleccionarHiloUI(tipo) {
    // Activar botón visualmente
    document.querySelectorAll('.hilo-btn').forEach(b => b.classList.remove('active'));
    document.querySelector(`.hilo-btn[data-tipo="${tipo}"]`).classList.add('active');

    // Inicializar singleton si no existe
    if (!window.constelacion) {
        window.constelacion = new Constelacion(document.getElementById('constelacion-canvas'));
        window.constelacion.iniciar();
    }

    // Activar herramienta
    window.constelacion.setHiloActivo(tipo);

    // Actualizar Panel de Info Dinámico
    const panel = document.getElementById('hilo-info-panel');
    const titulo = document.getElementById('hilo-info-titulo');
    const desc = document.getElementById('hilo-info-desc');
    const info = INFO_HILOS[tipo];

    if (info) {
        panel.style.display = 'block';
        // Hack para borde de color
        const colores = { conflicto: '#dc143c', alianza: '#1e90ff', destino: '#ffd700', secreto: '#8b008b', borrar: '#aaa' };
        panel.style.borderLeftColor = colores[tipo] || '#fff';
        titulo.innerText = info.titulo;
        desc.innerHTML = info.desc;
    }
}
