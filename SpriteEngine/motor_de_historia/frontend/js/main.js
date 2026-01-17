import { TokenSystem, fetchCharacter, fetchConstellation } from './api.js';
import { SoulRPGEngine } from './engine.js';
import { showPayPalModal } from './ui.js';

const rpgEngine = new SoulRPGEngine();

// --- MAIN HANDLERS ---

export async function handleForjarAlma(event) {
    if (event) event.preventDefault();
    console.log('⚡ Forjando Alma...');

    // 0. Check Tokens
    if (TokenSystem.get() <= 0) {
        // TEMPORARY PROMO
        console.log('No tokens, giving promo tokens.');
        TokenSystem.set(5);
    }

    if (TokenSystem.get() <= 0) {
        showPayPalModal('alma');
        return false;
    }

    if (!TokenSystem.use()) {
        alert("Sin tokens.");
        return false;
    }

    // 1. Get Data
    const form = document.getElementById('formPersonaje');
    const formData = new FormData(form);
    const params = new URLSearchParams(formData).toString();

    // Determine mode based on rol
    const rol = formData.get('rol') || '';
    const isRPGMode = (rol === 'Jugador'); // Only "Jugador" gets RPG treatment

    // 2. Loading
    document.body.style.cursor = 'wait';
    const btn = document.getElementById('btnPersonaje');
    if (btn) btn.innerText = 'Forjando...';

    try {
        // 3. API Call
        const personaje = await fetchCharacter(params);

        if (isRPGMode) {
            // === RPG MODE: Animation + Stats ===
            console.log('🎮 Modo RPG: Animación + Stats');

            // Process with RPG engine (adds stats)
            const rpgChar = rpgEngine.process(personaje, formData);

            // Store data for the crystal animation page
            sessionStorage.setItem('lastForgedCharacter', JSON.stringify(rpgChar));

            // Redirect to Forge Animation
            window.location.href = 'forja_de_almas.html';

        } else {
            // === STORY MODE: Direct Download (no stats) ===
            console.log('📖 Modo Historia: Descarga directa');

            // Generate and download story-only HTML
            const htmlContent = generateStoryHTML(personaje);
            downloadHTML(htmlContent, personaje.nombre || 'Personaje');

            // Show success message
            if (btn) btn.innerHTML = '✓ ¡Descargado!';
            setTimeout(() => {
                if (btn) btn.innerHTML = '<span style="font-size: 1.2rem;">⚡</span> FORJAR ALMA';
            }, 2000);
        }

    } catch (e) {
        console.error(e);
        alert('Error: ' + e.message);
        TokenSystem.add(1); // Refund
    } finally {
        document.body.style.cursor = 'default';
    }
}

// Generate HTML for story-only characters (no stats, pure narrative)
function generateStoryHTML(data) {
    const name = data.nombre || 'Personaje Sin Nombre';
    const rol = data.rol || 'Protagonista';
    const mundo = data.mundo || 'Desconocido';

    // Extract narrative elements
    const herida = data.psicologia?.herida || data.herida || 'Una herida que el tiempo no ha sanado.';
    const mascara = data.psicologia?.mascara || data.mascara || 'La cara que muestra al mundo.';
    const deseo = data.psicologia?.deseo || data.deseo || 'Lo que realmente anhela en su corazón.';
    const mentira = data.psicologia?.mentira || data.mentira || 'Lo que se dice a sí mismo para sobrevivir.';
    const sombra = data.psicologia?.sombra || data.sombra || 'Lo que oculta incluso de sí mismo.';

    // Biography
    const bioFases = data.biografia?.fases || [];
    const bioHTML = bioFases.length > 0
        ? bioFases.map(f => `<div class="bio-phase"><h3>✦ ${f.titulo}</h3><p>${f.contenido}</p></div>`).join('<div class="separator">❖</div>')
        : '<p>Una historia por escribir...</p>';

    // Narrative hooks
    const ganchos = data.ganchos_narrativos || [];
    const ganchosHTML = ganchos.length > 0
        ? ganchos.map(g => `<div class="hook">[Hook] ${g}</div>`).join('')
        : '<div class="hook">[Hook] El destino tiene planes para este personaje...</div>';

    // --- SKILLS & TALENTS (SoulForge Gacha System) ---
    const skills = data.skills || [];
    const soulTier = data.soul_tier || 'Desconocido';

    let skillsHTML = '';
    if (skills.length > 0) {
        skillsHTML = `
        <div class="skills-section">
            <h2 class="section-title">⚔️ Talentos & Habilidades</h2>
            <div class="tier-banner">ALMA TIER: ${soulTier.toUpperCase()}</div>
            <div class="skills-grid">
                ${skills.map(s => {
            const dots = Array(10).fill(0).map((_, i) =>
                `<span class="dot ${i < s.power_level ? 'fill' : ''}"></span>`
            ).join('');

            return `
                    <div class="skill-card cat-${s.category}">
                        <div class="skill-header">
                            <span class="skill-name">${s.name}</span>
                            <span class="skill-badge">${s.category}</span>
                        </div>
                        <div class="skill-desc">${s.description}</div>
                        <div class="skill-unlock">"${s.unlock_reason}"</div>
                        <div class="skill-meta">
                            <span>${s.cost ? '💠 ' + s.cost : ''}</span>
                            <div class="power-dots">${dots}</div>
                        </div>
                    </div>`;
        }).join('')}
            </div>
            <div style="text-align:center; font-size:0.8rem; color:#666; margin-top:10px;">
                * Poder y profundidad forjados por la complejidad del alma.
            </div>
        </div>`;
    }

    return `<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${name} - Ficha de Historia</title>
    <link href="https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Crimson+Text:ital,wght@0,400;0,600;0,700;1,400&family=Metamorphous&display=swap" rel="stylesheet">
    <style>
        :root { --paper: #e3dac9; --ink: #1a1a1a; --accent: #8b4513; --blood: #722f37; --gold: #c5a059; }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { background-color: #2a2a2a; padding: 40px 20px; font-family: 'Crimson Text', serif; color: var(--ink); min-height: 100vh; display: flex; justify-content: center; }
        .story-sheet { width: 100%; max-width: 800px; background: var(--paper); background-image: url('https://www.transparenttextures.com/patterns/aged-paper.png'); box-shadow: 0 0 60px rgba(0,0,0,0.5); padding: 50px; border-radius: 4px; }
        .header { text-align: center; border-bottom: 3px double var(--accent); padding-bottom: 25px; margin-bottom: 30px; }
        .role-badge { display: inline-block; background: var(--accent); color: var(--paper); padding: 4px 15px; border-radius: 4px; font-family: 'Cinzel'; font-size: 0.8rem; margin-bottom: 10px; }
        h1 { font-family: 'Cinzel'; font-size: 2.8rem; color: var(--ink); margin: 10px 0; }
        .subtitle { font-family: 'Metamorphous'; color: var(--accent); font-style: italic; }
        .section-title { font-family: 'Cinzel'; font-size: 1.4rem; color: var(--blood); border-bottom: 2px solid var(--accent); padding-bottom: 8px; margin: 30px 0 15px 0; }
        .psych-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin: 20px 0; }
        .psych-item { background: rgba(0,0,0,0.04); border: 1px solid #ccc; border-left: 4px solid var(--accent); padding: 15px; border-radius: 6px; }
        .psych-label { font-weight: bold; color: var(--blood); font-size: 0.9rem; margin-bottom: 5px; }
        .psych-text { font-style: italic; color: #444; line-height: 1.5; }
        .bio-phase h3 { font-family: 'Cinzel'; color: var(--blood); font-size: 1.2rem; margin-bottom: 10px; }
        .bio-phase p { text-align: justify; line-height: 1.8; margin-bottom: 15px; }
        .separator { text-align: center; color: var(--accent); opacity: 0.5; margin: 20px 0; }
        .hooks-section { background: rgba(0,0,0,0.03); border: 1px solid #ccc; padding: 20px; border-radius: 8px; margin-top: 25px; }
        .hook { border-bottom: 1px dashed #ccc; padding: 10px 0; color: #333; }
        .hook:last-child { border-bottom: none; }
        
        /* SKILLS STYLES */
        .skills-section { margin-top: 30px; border-top: 2px solid var(--accent); padding-top: 20px; }
        .tier-banner { text-align: center; background: #222; color: var(--gold); padding: 8px; font-family: 'Cinzel'; margin-bottom: 20px; border-radius: 4px; }
        .skills-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
        .skill-card { background: rgba(0,0,0,0.03); padding: 10px; border-left: 4px solid #666; border-radius: 0 6px 6px 0; }
        .skill-header { display: flex; justify-content: space-between; margin-bottom: 5px; font-family: 'Cinzel'; font-weight: bold; font-size: 0.9rem; }
        .skill-badge { font-size: 0.6rem; padding: 1px 5px; background: rgba(0,0,0,0.1); border-radius: 4px; }
        .skill-desc { font-size: 0.85rem; font-style: italic; color: #444; margin-bottom: 5px; }
        .skill-unlock { font-size: 0.7rem; color: #888; font-style: italic; margin-bottom: 5px; border-top: 1px dashed #ccc; padding-top: 3px; }
        .skill-meta { display: flex; justify-content: space-between; align-items: center; font-size: 0.8rem; }
        .power-dots { display: flex; gap: 1px; }
        .dot { width: 5px; height: 5px; background: #ddd; border-radius: 50%; }
        .dot.fill { background: var(--gold); }
        
        .cat-Active { border-left-color: #4169E1; }
        .cat-Passive { border-left-color: #32CD32; }
        .cat-Ultimate { border-left-color: #FFD700; background: rgba(255, 215, 0, 0.1); }
        .cat-Signature { border-left-color: #9932CC; background: rgba(153, 50, 204, 0.05); }

        .footer { text-align: center; margin-top: 30px; color: #888; font-size: 0.8rem; border-top: 1px solid #ccc; padding-top: 20px; }
        .download-btn { display: block; width: fit-content; margin: 25px auto 0; background: var(--ink); color: var(--paper); border: 2px solid var(--accent); padding: 12px 30px; font-family: 'Cinzel'; cursor: pointer; border-radius: 8px; }
        .download-btn:hover { background: var(--accent); color: var(--paper); }
        @media (max-width: 600px) { .psych-grid, .skills-grid { grid-template-columns: 1fr; } .story-sheet { padding: 25px; } h1 { font-size: 2rem; } }
    </style>
</head>
<body>
    <div class="story-sheet">
        <div class="header">
            <div class="role-badge">${rol.toUpperCase()}</div>
            <h1>${name}</h1>
            <div class="subtitle">${mundo} · Personaje de Historia</div>
        </div>
        <h2 class="section-title">🧠 Psicología del Personaje</h2>
        <div class="psych-grid">
            <div class="psych-item"><div class="psych-label">La Herida</div><div class="psych-text">${herida}</div></div>
            <div class="psych-item"><div class="psych-label">La Máscara</div><div class="psych-text">${mascara}</div></div>
            <div class="psych-item"><div class="psych-label">El Deseo</div><div class="psych-text">${deseo}</div></div>
            <div class="psych-item"><div class="psych-label">La Mentira</div><div class="psych-text">${mentira}</div></div>
            <div class="psych-item" style="grid-column: 1 / -1;"><div class="psych-label">La Sombra</div><div class="psych-text">${sombra}</div></div>
        </div>
        <h2 class="section-title">📜 Biografía</h2>
        ${bioHTML}
        
        ${skillsHTML}

        <div class="hooks-section">
            <h2 class="section-title" style="margin-top: 0;">📖 Ganchos Narrativos</h2>
            ${ganchosHTML}
        </div>
        <button class="download-btn" onclick="window.print()">🖨️ Imprimir / PDF</button>
        <div class="footer">Generado por SoulForge Engine | ${new Date().toLocaleDateString()}</div>
    </div>
</body>
</html>`;
}

function downloadHTML(content, name) {
    const blob = new Blob([content], { type: 'text/html;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${name.replace(/[^a-z0-9]/gi, '_')}_Historia.html`;
    document.body.appendChild(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
}

// Attach to window for HTML access
window.handleForjarAlma = handleForjarAlma;
