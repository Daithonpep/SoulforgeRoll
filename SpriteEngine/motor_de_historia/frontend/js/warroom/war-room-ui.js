
import { getRoomManager } from './room-manager.js';

export class WarRoomUI {
    constructor() {
        this.roomManager = getRoomManager({
            onStateChange: (state) => this.render(state),
            onChatMessage: (msg) => this.appendChatMessage(msg),
            onSystemMessage: (msg, severity) => this.showNotification(msg, severity),
            onRoomUpdate: (room) => this.updateRoomInfo(room),
            onPlayerAction: (action, data) => this.handlePlayerAction(action, data)
        });

        this.currentUserName = null;
        this.initializeUI();
    }

    initializeUI() {
        // Setup Chat Input
        const chatInput = document.getElementById('chatInput');
        if (chatInput) {
            chatInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    this.sendMessage();
                }
            });
        }
    }

    // === ACTIONS ===

    async createRoom() {
        const nameInput = document.getElementById('user-name-input');
        const userName = nameInput.value.trim();

        if (!userName) return alert("Debes elegir un nombre de héroe.");
        this.currentUserName = userName;

        this.showLoader(true);
        const roomId = await this.roomManager.createRoom("Sala de " + userName);

        if (roomId) {
            await this.joinRoomLogic(roomId, userName, 'gm');
        } else {
            alert("Error al crear la sala.");
            this.showLoader(false);
        }
    }

    async joinRoom() {
        const nameInput = document.getElementById('user-name-input');
        const codeInput = document.getElementById('room-code-input');

        const userName = nameInput.value.trim();
        let roomId = codeInput.value.trim();

        // Logic change: If no room code, acts as JOIN button but if code is empty, maybe error?
        // But the button says "UNIRSE".
        if (!userName) return alert("Debes elegir un nombre.");
        if (!roomId) return alert("Debes ingresar un código de sala.");

        this.currentUserName = userName;
        this.showLoader(true);
        await this.joinRoomLogic(roomId, userName, 'player');
    }

    async joinRoomLogic(roomId, userName, role) {
        const success = await this.roomManager.joinRoom(roomId, userName, role);
        if (success) {
            this.showLobby();
        } else {
            alert("No se pudo conectar a la sala.");
        }
        this.showLoader(false);
    }

    sendMessage() {
        const input = document.getElementById('chatInput');
        const message = input.value.trim();
        if (message) {
            this.roomManager.sendChat(message);
            input.value = '';
        }
    }

    rollDice(type) {
        const sides = parseInt(type.substring(1));
        const result = Math.floor(Math.random() * sides) + 1;

        // Show locally instantly (and maybe send to server)
        this.showDiceOverlay(result, type);
        this.roomManager.sendChat(`/roll ${type} ${result}`);
    }

    openDiceBuilder() {
        // Todo: Implement custom dice builder modal
        alert("Constructor de dados: Próximamente");
    }

    toggleFullscreen() {
        if (!document.fullscreenElement) {
            document.documentElement.requestFullscreen();
        } else {
            if (document.exitFullscreen) {
                document.exitFullscreen();
            }
        }
    }

    toggleView(view) {
        // Switch between 'map' and 'narrative' or others
        console.log("Switching view to", view);
    }

    toggleNarrative() {
        const el = document.querySelector('.narrative-content');
        if (el) el.classList.toggle('expanded');
    }

    // === UI UPDATES ===

    render(state) {
        // Called on state change
        if (state.room && state.room.participants) {
            this.updatePlayerList(state.room.participants);
        }
    }

    handlePlayerAction(action, data) {
        if (action === 'joined') {
            this.showNotification(`${data.user_name} se ha unido.`, 'success');
        } else if (action === 'left') {
            this.showNotification(`${data.userName} ha salido.`, 'warning');
        }
    }

    updatePlayerList(participantsMap) {
        const list = document.getElementById('players-list');
        const count = document.getElementById('player-count');
        if (!list) return;

        list.innerHTML = ''; // Clear

        const players = Object.values(participantsMap);
        count.innerText = `${players.length}/10`;

        players.forEach(p => {
            const card = document.createElement('div');
            card.className = 'player-card';
            if (p.connected === false) card.classList.add('offline');

            const avatarUrl = `https://api.dicebear.com/7.x/avataaars/svg?seed=${p.user_name}`;

            card.innerHTML = `
                <div class="player-avatar">
                    <img src="${avatarUrl}" alt="${p.user_name}">
                    <span class="player-status ${p.connected !== false ? 'online' : 'offline'}"></span>
                </div>
                <div class="player-info">
                    <div class="player-name" title="${p.user_name}">${p.user_name}</div>
                    <div class="player-class">
                        <span class="player-class-icon">⚔️</span> ${p.role}
                    </div>
                </div>
            `;
            list.appendChild(card);
        });
    }

    updateRoomInfo(room) {
        const nameDisplay = document.getElementById('room-name-display');
        const codeDisplay = document.getElementById('room-code-display');
        const inviteDisplay = document.getElementById('invite-code-display');

        if (nameDisplay) nameDisplay.innerText = room.name || "Sala de Guerra";
        if (codeDisplay) codeDisplay.innerText = room.id;
        if (inviteDisplay) inviteDisplay.innerText = room.id;

        // Update my own profile display
        const myId = this.roomManager.state.connectionId;
        const me = room.participants[myId];

        if (me) {
            const nameEl = document.getElementById('user-name-display');
            const roleEl = document.getElementById('user-role-display');
            const avatarEl = document.getElementById('user-avatar-display');
            const dmBadge = document.getElementById('dm-badge');

            if (nameEl) nameEl.innerText = me.user_name;
            if (roleEl) roleEl.innerText = me.role;
            if (avatarEl) avatarEl.src = `https://api.dicebear.com/7.x/avataaars/svg?seed=${me.user_name}`;
            if (dmBadge) {
                if (me.role === 'gm') dmBadge.classList.remove('hidden');
                else dmBadge.classList.add('hidden');
            }
        }
    }

    appendChatMessage(msg) {
        const container = document.getElementById('chat-messages');
        if (!container) return;

        const isMe = msg.from === this.currentUserName;
        const el = document.createElement('div');
        el.className = `chat-message ${isMe ? 'own' : ''}`;

        const avatarUrl = `https://api.dicebear.com/7.x/avataaars/svg?seed=${msg.from}`;
        const time = new Date(msg.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

        // Check if system message
        // (Protocol defined messages might differ, assuming standardized format from room-manager)

        el.innerHTML = `
            <div class="message-avatar">
                <img src="${avatarUrl}" alt="${msg.from}">
            </div>
            <div class="message-content">
                <div class="message-header">
                    <span class="message-author">${msg.from}</span>
                    <span class="message-time">${time}</span>
                </div>
                <div class="message-body">${msg.message}</div>
            </div>
        `;

        container.appendChild(el);
        container.scrollTop = container.scrollHeight;
    }

    // === VISUALS ===

    showLobby() {
        document.getElementById('connection-panel').style.display = 'none';
        const lobby = document.getElementById('lobby-ui');
        lobby.classList.remove('hidden');
        lobby.classList.add('animate-fade-in');
    }

    showLoader(show) {
        const loader = document.getElementById('loader');
        if (!loader) return;
        if (show) loader.classList.remove('hidden');
        else loader.classList.add('hidden');
    }

    showDiceOverlay(value, type) {
        const overlay = document.getElementById('dice-overlay');
        const valEl = document.getElementById('dice-result-value');
        const typeEl = document.getElementById('dice-result-type');

        if (valEl) valEl.innerText = value;
        if (typeEl) typeEl.innerText = type;

        if (overlay) {
            overlay.classList.add('active');
            setTimeout(() => overlay.classList.remove('active'), 2500);
        }
    }

    // === MODALS ===

    openModal(id) {
        const modal = document.getElementById(`modal-${id}`);
        if (modal) modal.classList.add('active');
    }

    closeModal(id) {
        const modal = document.getElementById(`modal-${id}`);
        if (modal) modal.classList.remove('active');
    }

    copyRoomCode() {
        const codeDisplay = document.getElementById('room-code-display');
        const code = codeDisplay ? codeDisplay.innerText : '';
        if (code && code !== '----') {
            navigator.clipboard.writeText(code).then(() => {
                this.showNotification("Código copiado al portapapeles", "success");
            });
        }
    }

    showNotification(msg, type = 'info') {
        // Simple console log for now, could be improved with a toast container
        console.log(`[${type.toUpperCase()}] ${msg}`);
        // If we want a visual toast:
        /*
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;
        toast.innerText = msg;
        document.body.appendChild(toast);
        setTimeout(() => toast.remove(), 3000);
        */
    }

    switchChatTab(tab) {
        this.currentChatTab = tab;
        document.querySelectorAll('.chat-tab').forEach(t => {
            t.classList.remove('active');
            if (t.dataset.tab === tab) t.classList.add('active');
        });
    }
}

// Expose instance
const ui = new WarRoomUI();
window.WarRoomUI = ui;
