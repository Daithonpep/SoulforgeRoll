// ============ DM PANEL CORE V4 ============
// Panel del Director del Destino - Con Sistema de Turnos

const DMPanel = {
    biblioteca: {
        npcs: [
            { id: 1, icono: '👴', nombre: 'Anciano Sabio', tipo: 'npc', rareza: 'raro', nivel: 5 },
            { id: 2, icono: '🧙', nombre: 'Hechicero', tipo: 'npc', rareza: 'epico', nivel: 15 },
            { id: 3, icono: '⚒️', nombre: 'Herrero', tipo: 'npc', rareza: 'comun', nivel: 3 },
            { id: 4, icono: '🏪', nombre: 'Mercader', tipo: 'npc', rareza: 'comun', nivel: 1 },
            { id: 5, icono: '💂', nombre: 'Guardia', tipo: 'npc', rareza: 'comun', nivel: 5 },
            { id: 6, icono: '👸', nombre: 'Princesa', tipo: 'npc', rareza: 'legendario', nivel: 10 },
        ],
        criaturas: [
            { id: 10, icono: '🐺', nombre: 'Lobo', tipo: 'monstruo', rareza: 'comun', nivel: 3 },
            { id: 11, icono: '👹', nombre: 'Goblin', tipo: 'monstruo', rareza: 'comun', nivel: 2 },
            { id: 12, icono: '💀', nombre: 'Esqueleto', tipo: 'monstruo', rareza: 'comun', nivel: 4 },
            { id: 13, icono: '🧟', nombre: 'Zombi', tipo: 'monstruo', rareza: 'comun', nivel: 3 },
            { id: 14, icono: '🕷️', nombre: 'Araña Gigante', tipo: 'monstruo', rareza: 'raro', nivel: 8 },
            { id: 15, icono: '👻', nombre: 'Espectro', tipo: 'monstruo', rareza: 'raro', nivel: 12 },
            { id: 16, icono: '🧌', nombre: 'Troll', tipo: 'monstruo', rareza: 'epico', nivel: 20 },
        ],
        jefes: [
            { id: 20, icono: '🐉', nombre: 'Dragón', tipo: 'jefe', rareza: 'legendario', nivel: 50 },
            { id: 21, icono: '👿', nombre: 'Señor Demonio', tipo: 'jefe', rareza: 'legendario', nivel: 45 },
            { id: 22, icono: '🦴', nombre: 'Rey Liche', tipo: 'jefe', rareza: 'legendario', nivel: 40 },
        ],
        estructuras: [
            { id: 30, icono: '🏰', nombre: 'Castillo', tipo: 'estructura', rareza: 'epico', nivel: 1 },
            { id: 31, icono: '🏠', nombre: 'Casa', tipo: 'estructura', rareza: 'comun', nivel: 1 },
            { id: 32, icono: '⛪', nombre: 'Templo', tipo: 'estructura', rareza: 'epico', nivel: 1 },
            { id: 33, icono: '🏛️', nombre: 'Ruinas', tipo: 'estructura', rareza: 'raro', nivel: 1 },
            { id: 34, icono: '⛺', nombre: 'Campamento', tipo: 'estructura', rareza: 'comun', nivel: 1 },
        ]
    },

    estado: {
        herramienta: 'seleccionar',
        categoria: 'todos',
        elementoSeleccionado: null,
        elementosEnMapa: [],
        zoom: 1,
        isPaintingFog: false,
        consoleType: 'narrar'
    },

    elementoArrastrado: null,
    fogCtx: null,
    contextTarget: null,

    init() {
        this.initCanvas();
        this.initFogCanvas();
        this.renderBiblioteca();
        this.setupToolbar();
        this.setupCategorias();
        this.setupDragDrop();
        this.setupMapa();
        this.setupZoom();
        this.setupFogPainting();
        this.setupContextMenu();
        this.setupConsole();
        this.setupAmbience();
        this.setupTurnSystem();
        console.log('🎭 DM Panel V4 Ready');
    },

    // ========== TURN SYSTEM ==========
    setupTurnSystem() {
        // Botones de dar turno
        document.querySelectorAll('.turn-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                const jugadorId = parseInt(btn.dataset.jugador);
                this.darTurno(jugadorId);
            });
        });

        // Fase buttons
        document.getElementById('phase-narrativa')?.addEventListener('click', () => this.setFase('narrativa'));
        document.getElementById('phase-combate')?.addEventListener('click', () => this.setFase('combate'));
        document.getElementById('combat-end')?.addEventListener('click', () => this.terminarCombate());
        document.getElementById('continue-story')?.addEventListener('click', () => this.continuarHistoria());

        // Usar TurnManager si existe
        if (window.TurnManager) {
            TurnManager.actualizarUI = () => this.renderTurnState();
            TurnManager.onTurnoTerminado = (jugador) => {
                this.mostrarNotificacion(`✅ ${jugador.nombre} terminó su turno`);
                this.checkAllTurnsDone();
            };
        }
    },

    darTurno(jugadorId) {
        // Quitar turno activo anterior
        document.querySelectorAll('.jugador-card').forEach(card => {
            card.classList.remove('has-turn');
        });

        // Dar turno al jugador
        const card = document.querySelector(`.jugador-card[data-jugador="${jugadorId}"]`);
        if (card && !card.classList.contains('turn-done')) {
            card.classList.add('has-turn');
            const nombre = card.querySelector('.jugador-nombre').textContent;
            this.mostrarNotificacion(`🎯 Turno de ${nombre}`);

            // Desactivar su botón
            card.querySelector('.turn-btn').disabled = true;

            if (window.TurnManager) {
                TurnManager.darTurno(jugadorId);
            }
        }
    },

    // Llamado cuando jugador termina turno (simulado o via WebSocket)
    jugadorTerminaTurno(jugadorId) {
        const card = document.querySelector(`.jugador-card[data-jugador="${jugadorId}"]`);
        if (card) {
            card.classList.remove('has-turn');
            card.classList.add('turn-done');
            this.checkAllTurnsDone();
        }
    },

    checkAllTurnsDone() {
        const cards = document.querySelectorAll('.jugador-card');
        const allDone = Array.from(cards).every(c => c.classList.contains('turn-done'));

        const continueBtn = document.getElementById('continue-story');
        if (continueBtn) {
            continueBtn.disabled = !allDone;
        }

        if (allDone) {
            this.mostrarNotificacion('✅ Todos han jugado su turno');
        }
    },

    setFase(fase) {
        document.querySelectorAll('.turn-phase-btn').forEach(btn => btn.classList.remove('active'));
        document.getElementById(`phase-${fase}`)?.classList.add('active');

        if (window.TurnManager) {
            TurnManager.estado.fase = fase;
        }
        this.mostrarNotificacion(`📖 Fase: ${fase.toUpperCase()}`);
    },

    terminarCombate() {
        // Reset todos los turnos
        document.querySelectorAll('.jugador-card').forEach(card => {
            card.classList.remove('has-turn', 'turn-done');
            card.querySelector('.turn-btn').disabled = false;
        });

        document.getElementById('continue-story').disabled = false;
        this.setFase('narrativa');
        this.mostrarNotificacion('⚔️ ¡Combate terminado!');

        if (window.TurnManager) {
            TurnManager.terminarCombate();
        }
    },

    continuarHistoria() {
        // Nueva ronda
        const rondaEl = document.getElementById('ronda-num');
        if (rondaEl) {
            rondaEl.textContent = parseInt(rondaEl.textContent) + 1;
        }

        // Reset turnos
        document.querySelectorAll('.jugador-card').forEach(card => {
            card.classList.remove('has-turn', 'turn-done');
            card.querySelector('.turn-btn').disabled = false;
        });

        document.getElementById('continue-story').disabled = true;
        this.mostrarNotificacion('📖 Nueva ronda iniciada');

        if (window.TurnManager) {
            TurnManager.nuevaRonda();
        }
    },

    renderTurnState() {
        // Sincronizar UI con TurnManager
        if (!window.TurnManager) return;

        TurnManager.estado.jugadores.forEach(jugador => {
            const card = document.querySelector(`.jugador-card[data-jugador="${jugador.id}"]`);
            if (card) {
                card.classList.toggle('turn-done', jugador.turnoCompletado);
                card.classList.toggle('has-turn', TurnManager.estado.turnoActual === jugador.id);
                card.classList.toggle('tension-critical', jugador.tension >= 80);
            }
        });
    },

    // ========== CANVAS ==========
    initCanvas() {
        const canvas = document.getElementById('mapa-canvas');
        const container = document.getElementById('canvas-container');
        const resize = () => {
            canvas.width = container.offsetWidth;
            canvas.height = container.offsetHeight;
            this.dibujarMapa();
        };
        resize();
        window.addEventListener('resize', resize);
    },

    initFogCanvas() {
        const fog = document.getElementById('fog-canvas');
        const container = document.getElementById('canvas-container');
        fog.width = container.offsetWidth;
        fog.height = container.offsetHeight;
        this.fogCtx = fog.getContext('2d');
        this.fogCtx.fillStyle = 'rgba(0, 0, 0, 0.85)';
        this.fogCtx.fillRect(0, 0, fog.width, fog.height);
    },

    dibujarMapa() {
        const canvas = document.getElementById('mapa-canvas');
        const ctx = canvas.getContext('2d');
        const gradient = ctx.createRadialGradient(canvas.width / 2, canvas.height / 2, 0, canvas.width / 2, canvas.height / 2, canvas.width / 1.5);
        gradient.addColorStop(0, '#2a2015');
        gradient.addColorStop(1, '#1a1510');
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        for (let i = 0; i < 300; i++) {
            ctx.fillStyle = `rgba(184, 134, 11, ${Math.random() * 0.05})`;
            ctx.beginPath();
            ctx.arc(Math.random() * canvas.width, Math.random() * canvas.height, Math.random() * 2, 0, Math.PI * 2);
            ctx.fill();
        }
    },

    // ========== FOG ==========
    setupFogPainting() {
        const container = document.getElementById('canvas-container');
        container.addEventListener('mousedown', (e) => {
            if (this.estado.herramienta !== 'niebla') return;
            this.estado.isPaintingFog = true;
            this.paintFog(e);
        });
        container.addEventListener('mousemove', (e) => {
            if (this.estado.isPaintingFog) this.paintFog(e);
        });
        container.addEventListener('mouseup', () => this.estado.isPaintingFog = false);
        container.addEventListener('mouseleave', () => this.estado.isPaintingFog = false);

        document.getElementById('fog-clear')?.addEventListener('click', () => {
            const fog = document.getElementById('fog-canvas');
            this.fogCtx.clearRect(0, 0, fog.width, fog.height);
            this.mostrarNotificacion('☀️ Niebla despejada');
        });

        document.getElementById('fog-toggle')?.addEventListener('click', () => {
            const fog = document.getElementById('fog-canvas');
            fog.style.display = fog.style.display === 'none' ? 'block' : 'none';
        });
    },

    paintFog(e) {
        const container = document.getElementById('canvas-container');
        const rect = container.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        this.fogCtx.globalCompositeOperation = 'destination-out';
        this.fogCtx.beginPath();
        this.fogCtx.arc(x, y, 35, 0, Math.PI * 2);
        this.fogCtx.fill();
        this.fogCtx.globalCompositeOperation = 'source-over';
    },

    // ========== BIBLIOTECA ==========
    renderBiblioteca() {
        const container = document.getElementById('biblioteca-items');
        let items = this.estado.categoria === 'todos'
            ? Object.values(this.biblioteca).flat()
            : this.biblioteca[this.estado.categoria] || [];

        const busqueda = document.getElementById('biblioteca-search')?.value.toLowerCase() || '';
        if (busqueda) items = items.filter(i => i.nombre.toLowerCase().includes(busqueda));

        container.innerHTML = items.map(item => `
            <div class="biblioteca-item" draggable="true" data-id="${item.id}" data-tipo="${item.tipo}" 
                 data-nombre="${item.nombre}" data-icono="${item.icono}" data-rareza="${item.rareza}" data-nivel="${item.nivel}">
                <span class="item-rango ${item.rareza}">${item.nivel}</span>
                <span class="item-icono">${item.icono}</span>
                <div class="item-nombre">${item.nombre}</div>
                <div class="item-tipo">${item.tipo}</div>
            </div>
        `).join('');
        this.setupDragDrop();
    },

    // ========== TOOLBAR ==========
    setupToolbar() {
        document.querySelectorAll('.toolbar-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.toolbar-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                this.estado.herramienta = btn.dataset.tool;

                const mapa = document.getElementById('mapa-container');
                mapa.style.cursor = this.estado.herramienta === 'mover' ? 'grab' :
                    this.estado.herramienta === 'borrar' ? 'crosshair' :
                        this.estado.herramienta === 'niebla' ? 'crosshair' : 'default';

                if (this.estado.herramienta === 'niebla') {
                    this.mostrarNotificacion('🌫️ Arrastra para revelar zonas');
                }

                const mapaCat = { 'npc': 'npcs', 'monstruo': 'criaturas', 'jefe': 'jefes', 'estructura': 'estructuras' };
                if (mapaCat[this.estado.herramienta]) {
                    this.estado.categoria = mapaCat[this.estado.herramienta];
                    document.querySelectorAll('.categoria-tab').forEach(t => t.classList.remove('active'));
                    document.querySelector(`.categoria-tab[data-categoria="${this.estado.categoria}"]`)?.classList.add('active');
                    this.renderBiblioteca();
                }
            });
        });
    },

    setupCategorias() {
        document.querySelectorAll('.categoria-tab').forEach(tab => {
            tab.addEventListener('click', () => {
                document.querySelectorAll('.categoria-tab').forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                this.estado.categoria = tab.dataset.categoria;
                this.renderBiblioteca();
            });
        });
        document.getElementById('biblioteca-search')?.addEventListener('input', () => this.renderBiblioteca());
    },

    // ========== DRAG & DROP ==========
    setupDragDrop() {
        const items = document.querySelectorAll('.biblioteca-item');
        const mapaContainer = document.getElementById('canvas-container');

        items.forEach(item => {
            item.addEventListener('dragstart', () => {
                this.elementoArrastrado = {
                    id: item.dataset.id, tipo: item.dataset.tipo, nombre: item.dataset.nombre,
                    icono: item.dataset.icono, rareza: item.dataset.rareza, nivel: parseInt(item.dataset.nivel)
                };
                item.style.opacity = '0.5';
            });
            item.addEventListener('dragend', () => { item.style.opacity = '1'; this.elementoArrastrado = null; });
        });

        mapaContainer.addEventListener('dragover', (e) => { e.preventDefault(); mapaContainer.classList.add('drop-zone-active'); });
        mapaContainer.addEventListener('dragleave', () => mapaContainer.classList.remove('drop-zone-active'));
        mapaContainer.addEventListener('drop', (e) => {
            e.preventDefault();
            mapaContainer.classList.remove('drop-zone-active');
            if (this.elementoArrastrado) {
                const rect = mapaContainer.getBoundingClientRect();
                this.colocarElemento(this.elementoArrastrado, e.clientX - rect.left, e.clientY - rect.top);
            }
        });
    },

    colocarElemento(datos, x, y) {
        const id = Date.now();
        const elemento = { id, ...datos, x, y };
        this.estado.elementosEnMapa.push(elemento);
        this.renderElementoEnMapa(elemento);
        this.mostrarNotificacion(`✨ ${datos.nombre} colocado`);
    },

    renderElementoEnMapa(elemento) {
        const container = document.getElementById('canvas-container');
        const div = document.createElement('div');
        div.className = 'mapa-elemento';
        div.dataset.id = elemento.id;
        div.style.left = (elemento.x - 22) + 'px';
        div.style.top = (elemento.y - 22) + 'px';

        const tipoClase = elemento.tipo === 'monstruo' ? 'monstruo' : elemento.tipo === 'jefe' ? 'jefe' : elemento.tipo === 'estructura' ? 'estructura' : 'npc';

        div.innerHTML = `<div class="elemento-visual ${tipoClase}">${elemento.icono}<span class="elemento-nivel">${elemento.nivel}</span></div><span class="elemento-nombre">${elemento.nombre}</span>`;

        div.addEventListener('mousedown', (e) => { if (e.button === 0) this.iniciarArrastre(e, div, elemento); });
        div.addEventListener('click', (e) => { e.stopPropagation(); this.seleccionar(elemento, div); });
        div.addEventListener('contextmenu', (e) => { e.preventDefault(); this.seleccionar(elemento, div); this.showContextMenu(e, elemento); });

        container.appendChild(div);
    },

    iniciarArrastre(e, div, elemento) {
        if (this.estado.herramienta === 'borrar') {
            this.eliminarElemento(elemento);
            return;
        }
        if (this.estado.herramienta !== 'seleccionar') return;
        e.preventDefault();
        const offsetX = e.clientX - div.getBoundingClientRect().left;
        const offsetY = e.clientY - div.getBoundingClientRect().top;
        const mover = (e) => {
            const container = document.getElementById('canvas-container');
            const rect = container.getBoundingClientRect();
            div.style.left = (e.clientX - rect.left - offsetX) + 'px';
            div.style.top = (e.clientY - rect.top - offsetY) + 'px';
        };
        const soltar = () => { document.removeEventListener('mousemove', mover); document.removeEventListener('mouseup', soltar); };
        document.addEventListener('mousemove', mover);
        document.addEventListener('mouseup', soltar);
    },

    seleccionar(elemento, div) {
        document.querySelectorAll('.mapa-elemento.selected').forEach(el => el.classList.remove('selected'));
        div.classList.add('selected');
        this.estado.elementoSeleccionado = elemento;
        document.getElementById('elemento-info').textContent = `${elemento.icono} ${elemento.nombre}`;
        document.getElementById('prop-nombre').value = elemento.nombre;
        document.getElementById('prop-nivel').value = elemento.nivel;
    },

    eliminarElemento(elemento) {
        const div = document.querySelector(`.mapa-elemento[data-id="${elemento.id}"]`);
        if (div) div.remove();
        this.estado.elementosEnMapa = this.estado.elementosEnMapa.filter(e => e.id !== elemento.id);
        this.estado.elementoSeleccionado = null;
        document.getElementById('elemento-info').textContent = '-';
        this.mostrarNotificacion(`🗑️ ${elemento.nombre} eliminado`);
    },

    // ========== CONTEXT MENU ==========
    setupContextMenu() {
        const menu = document.getElementById('context-menu');
        document.addEventListener('click', () => menu.classList.remove('active'));

        menu.querySelectorAll('.context-item').forEach(item => {
            item.addEventListener('click', () => {
                const action = item.dataset.action;
                if (action === 'eliminar' && this.contextTarget) {
                    this.eliminarElemento(this.contextTarget);
                } else if (action === 'duplicar' && this.contextTarget) {
                    this.colocarElemento({ ...this.contextTarget }, this.contextTarget.x + 50, this.contextTarget.y + 50);
                }
                menu.classList.remove('active');
            });
        });
    },

    showContextMenu(e, elemento) {
        this.contextTarget = elemento;
        const menu = document.getElementById('context-menu');
        menu.style.left = e.clientX + 'px';
        menu.style.top = e.clientY + 'px';
        menu.classList.add('active');
    },

    // ========== CONSOLE ==========
    setupConsole() {
        const input = document.getElementById('console-input');
        const sendBtn = document.getElementById('console-send');

        document.querySelectorAll('.console-type-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.console-type-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                this.estado.consoleType = btn.dataset.type;
            });
        });

        const enviar = () => {
            const texto = input.value.trim();
            if (!texto) return;
            this.mostrarTextoFlotante(texto);
            input.value = '';
        };

        sendBtn?.addEventListener('click', enviar);
        input?.addEventListener('keypress', (e) => { if (e.key === 'Enter') enviar(); });
    },

    mostrarTextoFlotante(texto) {
        const container = document.getElementById('canvas-container');
        const div = document.createElement('div');
        div.className = 'floating-text';
        div.textContent = texto;
        div.style.left = '50%';
        div.style.top = '40%';
        div.style.transform = 'translateX(-50%)';
        container.appendChild(div);

        setTimeout(() => div.remove(), 6000);
        this.mostrarNotificacion(`📜 Mensaje enviado`);
    },

    // ========== AMBIENCE ==========
    setupAmbience() {
        document.querySelectorAll('.ambience-btn[data-clima]').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.ambience-btn[data-clima]').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                this.mostrarNotificacion(`${btn.textContent} Clima aplicado`);
            });
        });

        document.querySelectorAll('.ambience-btn[data-hora]').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.ambience-btn[data-hora]').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                this.mostrarNotificacion(`${btn.textContent} Hora aplicada`);
            });
        });

        document.querySelectorAll('.ambience-btn[data-musica]').forEach(btn => {
            btn.addEventListener('click', () => {
                this.mostrarNotificacion(`🎵 Música: ${btn.dataset.musica}`);
            });
        });
    },

    // ========== MAPA EVENTS ==========
    setupMapa() {
        const container = document.getElementById('canvas-container');
        container.addEventListener('mousemove', (e) => {
            const rect = container.getBoundingClientRect();
            document.getElementById('mapa-coords').textContent = `X: ${Math.round(e.clientX - rect.left)} | Y: ${Math.round(e.clientY - rect.top)}`;
        });
        container.addEventListener('click', (e) => {
            if (e.target === container || e.target.tagName === 'CANVAS') {
                document.querySelectorAll('.mapa-elemento.selected').forEach(el => el.classList.remove('selected'));
                this.estado.elementoSeleccionado = null;
                document.getElementById('elemento-info').textContent = '-';
            }
        });
    },

    setupZoom() {
        document.getElementById('zoom-in')?.addEventListener('click', () => { this.estado.zoom = Math.min(this.estado.zoom + 0.1, 2); this.aplicarZoom(); });
        document.getElementById('zoom-out')?.addEventListener('click', () => { this.estado.zoom = Math.max(this.estado.zoom - 0.1, 0.5); this.aplicarZoom(); });
        document.getElementById('toggle-grid')?.addEventListener('click', () => {
            const grid = document.querySelector('.mapa-grid-overlay');
            grid.style.display = grid.style.display === 'none' ? 'block' : 'none';
        });
    },

    aplicarZoom() {
        document.getElementById('canvas-container').style.transform = `scale(${this.estado.zoom})`;
        document.getElementById('zoom-level').textContent = Math.round(this.estado.zoom * 100) + '%';
    },

    mostrarNotificacion(texto) {
        const notif = document.getElementById('notification');
        document.getElementById('notif-text').textContent = texto;
        notif.classList.add('show');
        setTimeout(() => notif.classList.remove('show'), 2000);
    }
};

document.addEventListener('DOMContentLoaded', () => DMPanel.init());
