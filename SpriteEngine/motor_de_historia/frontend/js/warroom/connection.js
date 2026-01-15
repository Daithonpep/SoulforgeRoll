// ============================================================
// CLIENTE DE WAR ROOM (WebSocket Wrapper)
// ============================================================

export class WarRoomClient {
    constructor(serverUrl, roomId, callbacks = {}) {
        this.serverUrl = serverUrl;
        this.roomId = roomId;
        this.callbacks = callbacks;

        this.ws = null;
        this.state = {
            status: 'disconnected',
            connectionId: null,
            room: null,
            latency: 0,
            lastError: null,
        };

        // Reconexión automática
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000;
        this.reconnectTimer = null;

        // Ping/Pong
        this.pingInterval = null;
        this.lastPingTime = 0;
    }

    // === CONEXIÓN ===

    connect() {
        return new Promise((resolve, reject) => {
            if (this.ws?.readyState === WebSocket.OPEN) {
                resolve();
                return;
            }

            this.updateStatus('connecting');

            // Asegurar protocolo ws/wss correcto
            const protocol = this.serverUrl.startsWith('https') ? 'wss' : 'ws';
            const host = this.serverUrl.replace(/^https?:\/\//, '');
            const wsUrl = `${protocol}://${host}/ws/${this.roomId}`;

            console.log(`🔌 Conectando a War Room: ${wsUrl}`);

            try {
                this.ws = new WebSocket(wsUrl);

                this.ws.onopen = () => {
                    console.log("✅ WebSocket Conectado!");
                    this.reconnectAttempts = 0;
                    this.startPingInterval();
                    resolve();
                };

                this.ws.onmessage = (event) => {
                    this.handleMessage(event.data);
                };

                this.ws.onclose = (event) => {
                    console.log(`🔌 WebSocket Cerrado: ${event.code} ${event.reason}`);
                    this.stopPingInterval();
                    this.handleDisconnect();
                };

                this.ws.onerror = (error) => {
                    console.error("❌ WebSocket Error:", error);
                    this.updateStatus('error');
                    this.state.lastError = 'Connection error';
                    reject(error);
                };
            } catch (e) {
                reject(e);
            }
        });
    }

    disconnect() {
        this.stopReconnect();
        this.stopPingInterval();

        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }

        this.updateStatus('disconnected');
    }

    handleDisconnect() {
        if (this.state.status === 'connected' || this.state.status === 'reconnecting') {
            this.attemptReconnect();
        }
    }

    attemptReconnect() {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            this.updateStatus('error');
            this.state.lastError = 'Max reconnection attempts reached';
            return;
        }

        this.updateStatus('reconnecting');
        this.reconnectAttempts++;

        const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

        console.log(`🔄 Reintentando conexión en ${delay}ms... (Intento ${this.reconnectAttempts})`);

        this.reconnectTimer = setTimeout(() => {
            this.connect().catch(() => {
                this.attemptReconnect();
            });
        }, delay);
    }

    stopReconnect() {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
    }

    // === PING/PONG ===

    startPingInterval() {
        this.pingInterval = setInterval(() => {
            this.ping();
        }, 30000); // Cada 30 segundos
    }

    stopPingInterval() {
        if (this.pingInterval) {
            clearInterval(this.pingInterval);
            this.pingInterval = null;
        }
    }

    ping() {
        this.lastPingTime = Date.now();
        this.send({ type: 'ping' });
    }

    // === ENVÍO DE MENSAJES ===

    send(message) {
        if (this.ws?.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        } else {
            console.warn("⚠️ Intentando enviar mensaje sin conexión:", message);
        }
    }

    // === RECEPCIÓN DE MENSAJES ===

    handleMessage(data) {
        try {
            const message = JSON.parse(data);

            switch (message.type) {
                case 'welcome':
                    this.state.connectionId = message.payload.connection_id;
                    this.state.room = message.payload.room;
                    this.updateStatus('connected');
                    this.safeCallback('onRoomUpdate', message.payload.room);
                    break;

                case 'player_joined':
                    if (this.state.room) {
                        this.state.room.participants.push(message.payload.participant);
                        this.safeCallback('onPlayerJoined', message.payload.participant);
                        this.safeCallback('onRoomUpdate', this.state.room);
                    }
                    break;

                case 'player_left':
                    if (this.state.room) {
                        this.state.room.participants = this.state.room.participants.filter(
                            p => p.connection_id !== message.payload.connection_id
                        );
                        this.safeCallback('onPlayerLeft', message.payload.connection_id, message.payload.user_name);
                        this.safeCallback('onRoomUpdate', this.state.room);
                    }
                    break;

                case 'character_loaded':
                    if (this.state.room) {
                        const participant = this.state.room.participants.find(
                            p => p.connection_id === message.payload.connection_id
                        );
                        if (participant) {
                            participant.character_name = message.payload.character_name;
                            participant.character_portrait = message.payload.portrait;
                            participant.state = 'ready';
                        }
                        this.safeCallback('onRoomUpdate', this.state.room);
                    }
                    break;

                case 'game_phase_changed':
                    if (this.state.room) {
                        this.state.room.phase = message.payload.phase;
                        this.safeCallback('onGamePhaseChanged', message.payload.phase);
                        this.safeCallback('onRoomUpdate', this.state.room);
                    }
                    break;

                case 'token_moved':
                    this.safeCallback('onTokenMoved', message.payload.connection_id, message.payload.position);
                    break;

                case 'chat_message':
                    this.safeCallback('onChatMessage', message.payload.from, message.payload.message, message.payload.is_dm);
                    break;

                case 'system_message':
                    this.safeCallback('onSystemMessage', message.payload.message, message.payload.severity);
                    break;

                case 'pong':
                    this.state.latency = Date.now() - this.lastPingTime;
                    break;

                case 'full_sync':
                    this.state.room = message.payload.room;
                    this.safeCallback('onRoomUpdate', message.payload.room);
                    break;

                case 'error':
                    this.state.lastError = message.payload.message;
                    this.safeCallback('onError', message.payload.code, message.payload.message);
                    break;
            }
        } catch (e) {
            console.error('Failed to parse message:', e);
        }
    }

    updateStatus(status) {
        this.state.status = status;
        this.safeCallback('onStatusChange', status);
    }

    safeCallback(name, ...args) {
        if (this.callbacks && typeof this.callbacks[name] === 'function') {
            this.callbacks[name](...args);
        }
    }

    // === API PÚBLICA ===

    join(userName, role) {
        this.send({
            type: 'join',
            payload: { user_name: userName, role },
        });
    }

    loadCharacter(character) {
        this.send({
            type: 'load_character',
            payload: { character },
        });
    }

    moveToken(position) {
        this.send({
            type: 'move_token',
            payload: { position },
        });
    }

    sendChat(message) {
        this.send({
            type: 'chat',
            payload: { message },
        });
    }

    startGame() {
        this.send({ type: 'start_game' });
    }

    pauseGame() {
        this.send({ type: 'pause_game' });
    }

    revealArea(x, y, radius) {
        this.send({
            type: 'reveal_area',
            payload: { x, y, radius },
        });
    }

    requestSync() {
        this.send({ type: 'request_sync' });
    }

    // === GETTERS ===

    getState() {
        return { ...this.state };
    }

    getRoom() {
        return this.state.room;
    }

    getConnectionId() {
        return this.state.connectionId;
    }

    isConnected() {
        return this.state.status === 'connected';
    }

    isDM() {
        return this.state.room?.is_dm ?? false;
    }
}
