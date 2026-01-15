import { WarRoomClient } from './connection.js';
import { SheetParser } from './sheet-parser.js';

// ============================================================
// CONFIGURACIÓN
// ============================================================

const SERVER_URL = window.ENV_WS_SERVER || 'wss://soulforge-server.railway.app'; // Fallback or global ENV
const API_URL = window.ENV_API_SERVER || 'https://soulforge-server.railway.app';

// ============================================================
// ROOM MANAGER
// ============================================================

export class RoomManager {
    constructor(callbacks = {}) {
        this.callbacks = callbacks;
        this.client = null;
        this.messageIdCounter = 0;
        this.state = this.createInitialState();
    }

    createInitialState() {
        return {
            status: 'disconnected',
            connectionId: null,
            latency: 0,
            room: null,
            localCharacter: null,
            characterLoaded: false,
            chatMessages: [],
            systemMessages: [],
        };
    }

    // === CONEXIÓN ===

    async joinRoom(roomId, userName, role) {
        try {
            // Verificar que la sala existe
            const roomInfo = await this.checkRoom(roomId);
            if (!roomInfo.exists) {
                this.addSystemMessage('La sala no existe', 'error');
                return false;
            }

            // Crear cliente
            this.client = new WarRoomClient(SERVER_URL, roomId, this.createClientCallbacks());

            // Conectar
            await this.client.connect();

            // Unirse a la sala
            this.client.join(userName, role);

            return true;

        } catch (e) {
            console.error(e);
            this.addSystemMessage('Error al conectar con el servidor', 'error');
            return false;
        }
    }

    async createRoom(name, config) {
        try {
            const response = await fetch(`${API_URL}/api/rooms`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, config }),
            });

            if (!response.ok) {
                throw new Error('Failed to create room');
            }

            const data = await response.json();
            return data.room_id;

        } catch (e) {
            console.error(e);
            this.addSystemMessage('Error al crear la sala', 'error');
            return null;
        }
    }

    async checkRoom(roomId) {
        try {
            const response = await fetch(`${API_URL}/api/rooms/${roomId}`);
            const data = await response.json();
            return { exists: data.exists, info: data };
        } catch {
            return { exists: false };
        }
    }

    disconnect() {
        this.client?.disconnect();
        this.client = null;
        this.state = this.createInitialState();
        this.notifyStateChange();
    }

    // === FICHA DE PERSONAJE ===

    async loadCharacterFromFile(file) {
        const result = await SheetParser.parseFromFile(file);

        if (result.success && result.character) {
            this.state.localCharacter = result.character;

            // Enviar al servidor si estamos conectados
            if (this.client && this.client.isConnected()) {
                this.client.loadCharacter(result.character);
            }

            this.state.characterLoaded = true;
            this.addSystemMessage(`Personaje "${result.character.name}" cargado`, 'success');
            this.notifyStateChange();
        } else {
            this.addSystemMessage(result.error || 'Error al cargar ficha', 'error');
        }

        return result;
    }

    // === ACCIONES DE JUEGO ===

    moveToken(x, y, layer = 0) {
        if (!this.client?.isConnected()) return;
        this.client.moveToken({ x, y, layer });
    }

    sendChat(message) {
        if (!this.client?.isConnected()) return;
        if (!message.trim()) return;
        this.client.sendChat(message);
    }

    // === ACCIONES DE DM ===

    startGame() {
        if (!this.client?.isDM()) return;
        this.client.startGame();
    }

    pauseGame() {
        if (!this.client?.isDM()) return;
        this.client.pauseGame();
    }

    revealArea(x, y, radius) {
        if (!this.client?.isDM()) return;
        this.client.revealArea(x, y, radius);
    }

    // === GETTERS ===

    getState() {
        return { ...this.state };
    }

    getParticipants() {
        return this.state.room?.participants || [];
    }

    getReadyPlayers() {
        return this.getParticipants().filter(
            p => p.state === 'ready' || p.state === 'in_game'
        );
    }

    isConnected() {
        return this.state.status === 'connected';
    }

    isDM() {
        return this.client?.isDM() ?? false;
    }

    getRoomCode() {
        return this.state.room?.id ?? null;
    }

    // === HELPERS PRIVADOS ===

    createClientCallbacks() {
        return {
            onStatusChange: (status) => {
                this.state.status = status;
                this.safeCallback('onConnectionChange', status);
                this.notifyStateChange();
            },

            onRoomUpdate: (room) => {
                this.state.room = room;
                this.state.connectionId = this.client?.getConnectionId() ?? null;
                this.safeCallback('onRoomUpdate', room);
                this.notifyStateChange();
            },

            onPlayerJoined: (participant) => {
                this.addSystemMessage(`${participant.user_name} se ha unido`, 'info');
                this.safeCallback('onPlayerAction', 'joined', participant);
            },

            onPlayerLeft: (connectionId, userName) => {
                this.addSystemMessage(`${userName} ha abandonado la sala`, 'info');
                this.safeCallback('onPlayerAction', 'left', { connectionId, userName });
            },

            onChatMessage: (from, message, isDm) => {
                const chatMsg = {
                    id: `msg_${++this.messageIdCounter}`,
                    from,
                    message,
                    isDm,
                    timestamp: Date.now(),
                };

                this.state.chatMessages.push(chatMsg);

                // Mantener solo los últimos 100 mensajes
                if (this.state.chatMessages.length > 100) {
                    this.state.chatMessages = this.state.chatMessages.slice(-100);
                }

                this.safeCallback('onChatMessage', chatMsg);
                this.notifyStateChange();
            },

            onSystemMessage: (message, severity) => {
                this.addSystemMessage(message, severity);
            },

            onGamePhaseChanged: (phase) => {
                const phaseNames = {
                    'lobby': 'Sala de espera',
                    'preparation': 'Preparación',
                    'active': 'Partida en curso',
                    'paused': 'Pausada',
                    'ended': 'Finalizada',
                };

                this.addSystemMessage(
                    `Estado de la partida: ${phaseNames[phase] || phase}`,
                    phase === 'active' ? 'success' : 'info'
                );
            },

            onError: (code, message) => {
                this.addSystemMessage(message, 'error');
            },
        };
    }

    addSystemMessage(message, severity) {
        this.state.systemMessages.push({
            id: `sys_${++this.messageIdCounter}`,
            message,
            severity,
            timestamp: Date.now(),
        });

        // Mantener solo los últimos 50 mensajes de sistema
        if (this.state.systemMessages.length > 50) {
            this.state.systemMessages = this.state.systemMessages.slice(-50);
        }

        this.notifyStateChange();
    }

    notifyStateChange() {
        if (this.callbacks.onStateChange) {
            this.callbacks.onStateChange({ ...this.state });
        }
    }

    safeCallback(name, ...args) {
        if (this.callbacks && typeof this.callbacks[name] === 'function') {
            this.callbacks[name](...args);
        }
    }
}

// === SINGLETON EXPORT ===

let instance = null;

export function getRoomManager(callbacks) {
    if (!instance) {
        instance = new RoomManager(callbacks);
    } else if (callbacks) {
        // Actualizar callbacks
        instance.callbacks = { ...instance.callbacks, ...callbacks };
    }
    return instance;
}

export function resetRoomManager() {
    instance?.disconnect();
    instance = null;
}
