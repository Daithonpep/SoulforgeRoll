// ============ PLAYER CONSOLE CORE ============
// Consola del Jugador - Soulforge VTT

const PlayerConsole = {
    // Estado del jugador
    estado: {
        roomCode: 'SOUL-7X9K',
        jugadorId: 1, // ID de este jugador
        character: null,
        tension: 48,
        hp: 75,
        maxHp: 100,
        mana: 30,
        maxMana: 50,
        lastDiceResult: null,
        esMiTurno: false, // Controlado por el DD
        fase: 'esperando' // 'esperando', 'mi_turno', 'turno_completado'
    },

    // Compañeros de grupo (sincronizado con DD)
    party: [
        { id: 2, nombre: 'Kira', icono: '⚔️', tension: 35, hp: 90 },
        { id: 3, nombre: 'Vesper', icono: '✨', tension: 88, hp: 60 },
    ],

    // WebSocket connection (para comunicarse con DD)
    ws: null,

    init() {
        this.initCanvas();
        this.initChat();
        this.setupDice();
        this.setupChat();
        this.setupSkills();
        this.setupActions();
        this.setupTurnIndicator();
        this.setupPartyTensionWarnings();
        this.loadCharacterFromStorage();
        this.connectToServer();
        console.log('🎮 Player Console Ready');
    },

    initChat() {
        // Mensaje de bienvenida
        this.chatChannels.grupo.messages.push({
            type: 'system',
            text: '— La partida ha comenzado —',
            sender: null,
            time: new Date().toLocaleTimeString('es-ES', { hour: '2-digit', minute: '2-digit' })
        });
    },

    // ========== TURN INDICATOR ==========
    setupTurnIndicator() {
        // Crear indicador de turno si no existe
        const gameView = document.getElementById('game-view');
        if (gameView && !document.getElementById('turn-indicator')) {
            const indicator = document.createElement('div');
            indicator.id = 'turn-indicator';
            indicator.className = 'turn-indicator';
            indicator.innerHTML = '⏳ Esperando tu turno...';
            indicator.style.cssText = `
                position: absolute; top: 20px; left: 50%; transform: translateX(-50%);
                background: rgba(0,0,0,0.85); border: 2px solid #888; border-radius: 25px;
                padding: 10px 25px; font-family: Cinzel, serif; font-size: 1.2vw;
                z-index: 50; transition: all 0.5s;
            `;
            gameView.appendChild(indicator);
        }
        this.updateTurnIndicator();
    },

    updateTurnIndicator() {
        const indicator = document.getElementById('turn-indicator');
        const endTurnBtn = document.getElementById('end-turn');

        if (this.estado.esMiTurno) {
            if (indicator) {
                indicator.innerHTML = '🎯 ¡ES TU TURNO!';
                indicator.style.borderColor = '#daa520';
                indicator.style.color = '#daa520';
                indicator.style.boxShadow = '0 0 20px rgba(218, 165, 32, 0.5)';
            }
            if (endTurnBtn) endTurnBtn.disabled = false;
        } else if (this.estado.fase === 'turno_completado') {
            if (indicator) {
                indicator.innerHTML = '✅ Turno completado';
                indicator.style.borderColor = '#228b22';
                indicator.style.color = '#228b22';
                indicator.style.boxShadow = 'none';
            }
            if (endTurnBtn) endTurnBtn.disabled = true;
        } else {
            if (indicator) {
                indicator.innerHTML = '⏳ Esperando tu turno...';
                indicator.style.borderColor = '#888';
                indicator.style.color = '#e0d5c7';
                indicator.style.boxShadow = 'none';
            }
            if (endTurnBtn) endTurnBtn.disabled = true;
        }
    },

    // Llamado cuando DD da turno a este jugador
    recibirTurno() {
        this.estado.esMiTurno = true;
        this.estado.fase = 'mi_turno';
        this.updateTurnIndicator();
        this.mostrarNotificacion('🎯 ¡Es tu turno!');
        this.addChatMessage('system', '— Es tu turno —');

        // Efecto visual dramático
        document.body.style.animation = 'pulse-turn 0.5s';
        setTimeout(() => document.body.style.animation = '', 500);
    },

    // ========== PARTY TENSION WARNINGS ==========
    setupPartyTensionWarnings() {
        // Actualizar UI de compañeros cada 5 segundos (simulado)
        this.updatePartyDisplay();
        setInterval(() => this.updatePartyDisplay(), 5000);
    },

    updatePartyDisplay() {
        const partyMembers = document.querySelectorAll('.party-member');

        this.party.forEach((member, index) => {
            const memberEl = partyMembers[index + 1]; // +1 porque el primero es "Tú"
            if (memberEl) {
                // Actualizar barra de HP
                const hpFill = memberEl.querySelector('.member-hp-fill');
                if (hpFill) hpFill.style.width = member.hp + '%';

                // Warning de tensión alta
                if (member.tension >= 80) {
                    memberEl.classList.add('tension-warning');
                    memberEl.style.animation = 'pulse-danger 1s infinite';
                    memberEl.style.borderColor = '#dc143c';

                    // Cambiar texto de tensión
                    const statusEl = memberEl.querySelector('.member-status');
                    if (statusEl) statusEl.innerHTML = `<span style="color:#dc143c">⚠️ Tensión crítica!</span>`;
                } else {
                    memberEl.classList.remove('tension-warning');
                    memberEl.style.animation = '';
                    memberEl.style.borderColor = '';
                }
            }
        });
    },

    // ========== CANVAS ==========
    initCanvas() {
        const canvas = document.getElementById('game-canvas');
        const container = document.getElementById('game-view');

        const resize = () => {
            canvas.width = container.offsetWidth;
            canvas.height = container.offsetHeight;
            this.drawGameView();
        };

        resize();
        window.addEventListener('resize', resize);
    },

    drawGameView() {
        const canvas = document.getElementById('game-canvas');
        const ctx = canvas.getContext('2d');

        // Fondo oscuro con textura
        const gradient = ctx.createRadialGradient(
            canvas.width / 2, canvas.height / 2, 0,
            canvas.width / 2, canvas.height / 2, canvas.width / 1.5
        );
        gradient.addColorStop(0, '#1a1a25');
        gradient.addColorStop(1, '#0a0a0f');
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Partículas decorativas
        for (let i = 0; i < 50; i++) {
            ctx.fillStyle = `rgba(184, 134, 11, ${Math.random() * 0.1})`;
            ctx.beginPath();
            ctx.arc(
                Math.random() * canvas.width,
                Math.random() * canvas.height,
                Math.random() * 2 + 1,
                0, Math.PI * 2
            );
            ctx.fill();
        }

        // Texto de espera si no hay mapa
        ctx.fillStyle = 'rgba(184, 134, 11, 0.3)';
        ctx.font = '20px Cinzel';
        ctx.textAlign = 'center';
        ctx.fillText('Esperando al Director del Destino...', canvas.width / 2, canvas.height / 2);
    },

    // ========== DADOS ==========
    setupDice() {
        document.querySelectorAll('.dice-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const diceType = btn.dataset.dice;
                this.rollDice(diceType, btn);
            });
        });
    },

    rollDice(type, btn) {
        // Animación de shake
        btn.classList.add('rolling');
        setTimeout(() => btn.classList.remove('rolling'), 500);

        // Calcular resultado
        const sides = parseInt(type.replace('d', ''));
        const result = Math.floor(Math.random() * sides) + 1;

        // Mostrar resultado
        const resultEl = document.getElementById('dice-result');
        const typeEl = document.getElementById('dice-type');

        resultEl.textContent = result;
        typeEl.textContent = type.toUpperCase();

        // Efecto visual según resultado
        resultEl.style.color = result === sides ? '#00ff00' : result === 1 ? '#ff0000' : '#daa520';

        // Añadir al chat
        this.addChatMessage('system', `🎲 Tiraste ${type}: ${result}${result === sides ? ' (Crítico!)' : result === 1 ? ' (Pifia!)' : ''}`);

        // Guardar resultado
        this.estado.lastDiceResult = { type, result };

        // Enviar al servidor
        this.sendToServer({ type: 'dice_roll', dice: type, result });
    },

    // ========== CHAT ==========
    // Canales de chat - 'grupo' siempre existe, secretos se añaden dinámicamente
    chatChannels: {
        grupo: { name: '👥 Grupo', messages: [], unread: false }
        // Secretos se añaden cuando DD conecta jugadores, ej:
        // 'secreto_aldric_kira': { name: '🔮 Secreto', messages: [], unread: false, participants: [1, 2] }
    },
    currentChannel: 'grupo',

    setupChat() {
        const input = document.getElementById('chat-input');
        const sendBtn = document.getElementById('chat-send');

        const enviar = () => {
            const texto = input.value.trim();
            if (!texto) return;

            this.addChatMessage('player', texto, 'Tú', this.currentChannel);
            this.sendToServer({ type: 'chat', channel: this.currentChannel, message: texto });
            input.value = '';
        };

        sendBtn.addEventListener('click', enviar);
        input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') enviar();
        });

        this.renderChatTabs();
    },

    renderChatTabs() {
        const tabsContainer = document.getElementById('chat-tabs');
        if (!tabsContainer) return;

        tabsContainer.innerHTML = '';

        Object.entries(this.chatChannels).forEach(([channelId, channel]) => {
            const btn = document.createElement('button');
            btn.className = 'chat-tab' + (channelId === this.currentChannel ? ' active' : '');
            if (channelId !== 'grupo') btn.classList.add('secret');
            if (channel.unread && channelId !== this.currentChannel) btn.classList.add('has-new');
            btn.dataset.chat = channelId;
            btn.textContent = channel.name;

            btn.addEventListener('click', () => this.switchChannel(channelId));
            tabsContainer.appendChild(btn);
        });
    },

    switchChannel(channelId) {
        this.currentChannel = channelId;
        this.chatChannels[channelId].unread = false;
        this.renderChatTabs();
        this.renderChatMessages();
    },

    renderChatMessages() {
        const container = document.getElementById('chat-messages');
        if (!container) return;

        container.innerHTML = '';
        const channel = this.chatChannels[this.currentChannel];

        if (!channel || channel.messages.length === 0) {
            container.innerHTML = '<div class="chat-message system">— Sin mensajes aún —</div>';
            return;
        }

        channel.messages.forEach(msg => {
            const div = document.createElement('div');
            div.className = `chat-message ${msg.type}`;

            if (msg.type === 'system') {
                div.textContent = msg.text;
            } else {
                div.innerHTML = `
                    <div class="chat-sender">${msg.sender}</div>
                    <div class="chat-text">${msg.text}</div>
                    <div class="chat-time">${msg.time}</div>
                `;
            }
            container.appendChild(div);
        });

        container.scrollTop = container.scrollHeight;
    },

    addChatMessage(type, text, sender = null, channel = 'grupo') {
        const now = new Date();
        const time = `${now.getHours()}:${now.getMinutes().toString().padStart(2, '0')}`;

        if (!this.chatChannels[channel]) {
            channel = 'grupo'; // fallback
        }

        const msg = { type, text, sender, time };
        this.chatChannels[channel].messages.push(msg);

        // Si no estamos en ese canal, marcar como no leído
        if (channel !== this.currentChannel) {
            this.chatChannels[channel].unread = true;
            this.renderChatTabs();
        } else {
            // Actualizar UI directamente si estamos en ese canal
            this.renderChatMessages();
        }
    },

    // Llamado cuando DD crea un enlace secreto entre dos jugadores
    addSecretChannel(channelId, channelName, participants) {
        if (participants.includes(this.estado.jugadorId)) {
            this.chatChannels[channelId] = {
                name: channelName || '🔮 Secreto',
                messages: [],
                unread: false,
                participants: participants
            };
            this.renderChatTabs();
            this.mostrarNotificacion(`🔮 Nuevo canal secreto disponible`);
        }
    },

    // Recibir mensaje del servidor (DD o otro jugador)
    receiveMessage(data) {
        const { type, text, sender, channel } = data;

        // Si es un canal secreto que no tenemos, ignorar (no somos participantes)
        if (channel && !this.chatChannels[channel] && channel !== 'grupo') {
            return;
        }

        this.addChatMessage(type || 'player', text, sender, channel || 'grupo');
    },

    // ========== HABILIDADES ==========
    setupSkills() {
        document.querySelectorAll('.skill-slot:not(.on-cooldown)').forEach(slot => {
            slot.addEventListener('click', () => {
                const skill = slot.dataset.skill;
                this.useSkill(skill, slot);
            });
        });

        // Teclas rápidas
        document.addEventListener('keydown', (e) => {
            const keyMap = { '1': 'fireball', '2': 'shadow', '3': 'drain', '4': 'ultimate' };
            if (keyMap[e.key]) {
                const slot = document.querySelector(`.skill-slot[data-skill="${keyMap[e.key]}"]`);
                if (slot && !slot.classList.contains('on-cooldown')) {
                    this.useSkill(keyMap[e.key], slot);
                }
            }
        });
    },

    useSkill(skillName, slot) {
        const skillCostText = slot.querySelector('.skill-cost').textContent;
        // Extraer el primer número encontrado en el texto (ej: "15 Mana" -> 15)
        const costMatch = skillCostText.match(/(\d+)/);
        const cost = costMatch ? parseInt(costMatch[0]) : 0;

        if (this.estado.mana < cost) {
            this.mostrarNotificacion('❌ No tienes suficiente mana');
            // Feedback visual de error
            slot.style.borderColor = '#dc143c';
            setTimeout(() => slot.style.borderColor = '', 500);
            return;
        }

        // Gastar mana
        this.estado.mana -= cost;
        this.updateStats();

        // Efecto visual
        slot.style.transform = 'scale(0.95)';
        setTimeout(() => slot.style.transform = '', 150);

        // Crear efecto de partícula o brillo (opcional)
        const ripple = document.createElement('div');
        ripple.className = 'skill-ripple';
        slot.appendChild(ripple);
        setTimeout(() => ripple.remove(), 500);

        // Añadir al chat
        const skillNameText = slot.querySelector('.skill-name').textContent;
        this.addChatMessage('system', `⚡ Usaste ${skillNameText}`);

        // Enviar al servidor
        this.sendToServer({ type: 'skill_use', skill: skillName });

        this.mostrarNotificacion(`✨ ${skillNameText} activada`);
    },

    // ========== ACCIONES ==========
    setupActions() {
        document.querySelectorAll('.action-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const action = btn.textContent.trim();
                this.performAction(action);
            });
        });

        document.getElementById('end-turn')?.addEventListener('click', () => {
            this.endTurn();
        });
    },

    performAction(action) {
        this.addChatMessage('system', `🎯 Realizas: ${action}`);
        this.sendToServer({ type: 'action', action });
        this.mostrarNotificacion(`🎯 ${action}`);
    },

    endTurn() {
        if (!this.estado.esMiTurno) {
            this.mostrarNotificacion('❌ No es tu turno');
            return;
        }

        this.estado.esMiTurno = false;
        this.estado.fase = 'turno_completado';
        this.updateTurnIndicator();

        this.addChatMessage('system', '— Fin de tu turno —');
        this.sendToServer({ type: 'end_turn', jugadorId: this.estado.jugadorId });
        this.mostrarNotificacion('✅ Turno terminado');
    },

    updateStats() {
        // Actualizar barras de stats
        const vidaBar = document.querySelector('.stat-bar.vida');
        const manaBar = document.querySelector('.stat-bar.mana');
        const tensionBar = document.querySelector('.stat-bar.tension');

        if (vidaBar) vidaBar.style.width = (this.estado.hp / this.estado.maxHp * 100) + '%';
        if (manaBar) manaBar.style.width = (this.estado.mana / this.estado.maxMana * 100) + '%';
        if (tensionBar) tensionBar.style.width = this.estado.tension + '%';

        // Actualizar valores de texto
        document.querySelector('.stat-row:nth-child(1) .stat-value').textContent = `${this.estado.hp}/${this.estado.maxHp}`;
        document.querySelector('.stat-row:nth-child(2) .stat-value').textContent = `${this.estado.mana}/${this.estado.maxMana}`;
        document.querySelector('.stat-row:nth-child(3) .stat-value').textContent = `${this.estado.tension}%`;

        // Actualizar orbe de tensión
        const orb = document.getElementById('tension-orb');
        orb.textContent = this.estado.tension;
        orb.className = 'tension-orb';
        if (this.estado.tension < 30) orb.classList.add('low');
        else if (this.estado.tension < 70) orb.classList.add('medium');
        else orb.classList.add('high');
    },

    // ========== PERSONAJE ==========
    loadCharacterFromStorage() {
        const charData = localStorage.getItem('soulforge_character');
        if (charData) {
            try {
                this.estado.character = JSON.parse(charData);
                this.applyCharacterData(this.estado.character);
            } catch (e) {
                console.warn('No character data found');
            }
        }
    },

    applyCharacterData(char) {
        if (!char) return;

        // Nombre y título
        document.getElementById('char-name').textContent = char.identidad?.nombre || 'Aventurero';
        document.getElementById('char-title').textContent = char.identidad?.titulo || 'Sin título';

        // Avatar
        const avatarUrl = char.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${char.identidad?.nombre || 'default'}`;
        document.getElementById('portrait-img').src = avatarUrl;

        // Tier
        const tier = (char.tier || 'alma').toLowerCase().replace(' ', '_');
        const frame = document.getElementById('portrait-frame');
        frame.className = 'portrait-frame';
        frame.classList.add(`tier-${tier}`);

        // Stats
        if (char.attributes) {
            this.estado.hp = char.attributes.hp || 100;
            this.estado.maxHp = char.attributes.max_hp || 100;
            this.estado.mana = char.attributes.mana || 50;
            this.estado.maxMana = char.attributes.max_mana || 50;
        }

        this.updateStats();
    },

    // ========== MENSAJES DEL DD ==========
    showDDMessage(text, type = 'narrar') {
        const msgDiv = document.getElementById('dd-message');
        const textDiv = document.getElementById('dd-message-text');
        const typeDiv = msgDiv.querySelector('.dd-message-type');

        const typeLabels = {
            'narrar': 'El Director del Destino narra...',
            'susurro': 'Una voz susurra en tu mente...',
            'evento': '⚡ ¡EVENTO!'
        };

        typeDiv.textContent = typeLabels[type] || typeLabels['narrar'];
        textDiv.textContent = text;
        msgDiv.style.display = 'block';

        // También añadir al chat
        this.addChatMessage('dd', text, '🎭 Director del Destino');

        // Auto-ocultar después de un tiempo
        setTimeout(() => {
            msgDiv.style.display = 'none';
        }, 8000);
    },

    // ========== CONEXIÓN ==========
    connectToServer() {
        // Por ahora simulamos la conexión
        // En producción esto sería WebSocket real
        console.log('📡 Conectando al servidor...');

        // Simular conexión exitosa
        setTimeout(() => {
            this.mostrarNotificacion('✅ Conectado a la sala');
        }, 1000);

        // Simular mensaje del DD después de 3 segundos
        setTimeout(() => {
            this.showDDMessage('El bosque se oscurece. Preparad vuestras armas, algo se acerca...', 'narrar');
        }, 3000);
    },

    sendToServer(data) {
        // Por ahora solo log, después WebSocket real
        console.log('📤 Enviando:', data);

        // Simular feedback del servidor
        if (data.type === 'dice_roll') {
            // El DD vería este resultado
        }
    },

    // ========== UTILS ==========
    mostrarNotificacion(texto) {
        const notif = document.getElementById('notification');
        document.getElementById('notif-text').textContent = texto;
        notif.classList.add('show');
        setTimeout(() => notif.classList.remove('show'), 2500);
    }
};

// Auto-init
document.addEventListener('DOMContentLoaded', () => PlayerConsole.init());
