// ============ TURN MANAGER - SHARED MODULE ============
// Sistema de turnos compartido entre DD y Jugadores

const TurnManager = {
    // Estado de turnos
    estado: {
        fase: 'narrativa', // 'narrativa', 'combate', 'exploracion'
        turnoActual: null, // ID del jugador con turno activo
        jugadores: [
            { id: 1, nombre: 'Aldric', icono: '🧙', turnoCompletado: false, tension: 62, hp: 75, activo: true },
            { id: 2, nombre: 'Kira', icono: '⚔️', turnoCompletado: false, tension: 35, hp: 90, activo: true },
            { id: 3, nombre: 'Vesper', icono: '✨', turnoCompletado: false, tension: 88, hp: 60, activo: true },
        ],
        combateTerminado: false,
        rondaActual: 1
    },

    // Dar turno a un jugador (llamado por DD)
    darTurno(jugadorId) {
        this.estado.turnoActual = jugadorId;
        const jugador = this.estado.jugadores.find(j => j.id === jugadorId);
        if (jugador) {
            console.log(`🎯 Turno de ${jugador.nombre}`);
            // Notificar al jugador (WebSocket en producción)
            this.onTurnoAsignado?.(jugador);
        }
        this.actualizarUI();
    },

    // Terminar turno (llamado por jugador)
    terminarTurno(jugadorId) {
        const jugador = this.estado.jugadores.find(j => j.id === jugadorId);
        if (jugador) {
            jugador.turnoCompletado = true;
            this.estado.turnoActual = null;
            console.log(`✅ ${jugador.nombre} terminó su turno`);
            this.onTurnoTerminado?.(jugador);
        }
        this.actualizarUI();
    },

    // Verificar si todos han tenido turno
    todosCompletaron() {
        return this.estado.jugadores.filter(j => j.activo).every(j => j.turnoCompletado);
    },

    // Terminar combate (llamado por DD)
    terminarCombate(motivo = 'victoria') {
        this.estado.combateTerminado = true;
        this.estado.fase = 'narrativa';
        console.log(`⚔️ Combate terminado: ${motivo}`);
        this.onCombateTerminado?.(motivo);
        this.actualizarUI();
    },

    // Nueva ronda
    nuevaRonda() {
        this.estado.jugadores.forEach(j => j.turnoCompletado = false);
        this.estado.turnoActual = null;
        this.estado.rondaActual++;
        console.log(`🔄 Ronda ${this.estado.rondaActual}`);
        this.actualizarUI();
    },

    // Iniciar combate
    iniciarCombate() {
        this.estado.fase = 'combate';
        this.estado.combateTerminado = false;
        this.estado.rondaActual = 1;
        this.estado.jugadores.forEach(j => j.turnoCompletado = false);
        console.log('⚔️ ¡Combate iniciado!');
        this.actualizarUI();
    },

    // Actualizar tensión de un jugador
    actualizarTension(jugadorId, nuevaTension) {
        const jugador = this.estado.jugadores.find(j => j.id === jugadorId);
        if (jugador) {
            jugador.tension = nuevaTension;
            this.actualizarUI();
        }
    },

    // Obtener clase CSS según tensión
    getTensionClass(tension) {
        if (tension >= 80) return 'critical';
        if (tension >= 60) return 'high';
        if (tension >= 30) return 'medium';
        return 'low';
    },

    // Placeholder para actualización de UI (sobrescrito por cada panel)
    actualizarUI() {
        // Sobrescrito en dm_panel y player_console
    },

    // Callbacks (sobrescritos por cada panel)
    onTurnoAsignado: null,
    onTurnoTerminado: null,
    onCombateTerminado: null
};

// Exportar para uso global
window.TurnManager = TurnManager;
