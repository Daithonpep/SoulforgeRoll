import { TokenSystem, fetchCharacter, fetchConstellation } from './api.js';
import { SoulRPGEngine } from './engine.js';
import { showPayPalModal } from './ui.js';
import { getLabels } from './i18n-sheet.js';

const rpgEngine = new SoulRPGEngine();

// --- MAIN HANDLERS ---

export async function handleForjarAlma(event) {
    if (event) event.preventDefault();
    console.log('⚡ Forjando Alma...');

    // 0. Check Tokens - No free tokens, must pay or have VIP code
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
    // DETECT LANGUAGE: Check data first, then localStorage, then default to 'es'
    const storedLang = typeof localStorage !== 'undefined' ? localStorage.getItem('soulforge_lang') : null;
    const lang = data.idioma || data.lang || storedLang || 'es';
    const L = getLabels(lang);

    // CORRECT DATA EXTRACTION
    const name = data.identidad?.nombre || data.nombre || L.nameless;
    const titulo = data.identidad?.titulo || '';
    const rol = data.rol || 'Protagonista';
    const mundo = data.mundo || 'Desconocido';
    const edad = data.identidad?.edad || '?';
    const genero = data.identidad?.genero || '';

    // Vestimenta y apariencia
    const vestimenta = data.identidad?.vestimenta || L.defaultClothes;
    const voz = data.identidad?.voz || L.defaultVoice;
    const rasgo = data.identidad?.rasgo_distintivo || L.defaultTrait;
    const manierismo = data.identidad?.manierismo || '';

    // PSYCHOLOGY (from capas)
    const mascara = data.capas?.mascara || {};
    const herida = data.capas?.herida || {};
    const mentira = data.capas?.mentira || {};
    const sombra = data.capas?.sombra || {};
    const deseo = data.capas?.deseo_necesidad || {};

    // Narrative elements with localized fallbacks
    const heridaTexto = herida.causante ?
        `${herida.causante}. ${herida.circunstancia || ''} ${herida.como_lo_cambio || ''}` :
        L.defaultWound;

    const mascaraTexto = mascara.comportamiento_publico || L.defaultMask;
    const fraseTexto = mascara.frase_tipica || L.defaultQuote;
    const miedoTexto = mascara.miedo_central || L.defaultFear;
    const secretoTexto = mascara.deseo_secreto || L.defaultSecret;

    const deseoTexto = deseo.deseo_consciente || L.defaultWant;
    const necesidadTexto = deseo.necesidad_real || L.defaultNeed;

    const mentiraTexto = mentira.la_mentira || L.defaultLie;
    const verdadTexto = mentira.verdad_necesaria || L.defaultTruth;

    const sombraTexto = sombra.rasgo_negado || L.defaultShadow;
    const sombraTrigger = sombra.trigger_emergencia || L.defaultTrigger;

    // ARCO NARRATIVO
    const arco = data.arco || {};
    const arcoInicial = arco.estado_inicial || L.defaultArc1;
    const arcoQuiebre = arco.punto_de_quiebre || L.defaultArc2;
    const arcoPositivo = arco.resolucion_positiva || L.defaultArc3;
    const arcoTragico = arco.resolucion_tragica || L.defaultArc4;

    // BIOGRAPHY
    const bioFases = data.biografia?.fases || [];
    const bioHTML = bioFases.length > 0
        ? bioFases.map(f => `
            <div class="bio-phase">
                <h3>✦ ${f.titulo}</h3>
                <p>${f.contenido}</p>
            </div>`).join('<div class="separator">❖</div>')
        : `<p><em>${L.defaultBio}</em></p>`;

    // HOOKS
    const ganchos = data.ganchos_narrativos || [];
    const momentos = data.momentos_definitorios || [];

    const ganchosHTML = ganchos.length > 0
        ? ganchos.map(g => `<div class="hook-item">⚔️ ${g}</div>`).join('')
        : `<div class="hook-item">⚔️ ${L.defaultHook}</div>`;

    const momentosHTML = momentos.length > 0
        ? momentos.map(m => `<div class="hook-item">💫 ${m}</div>`).join('')
        : '';

    // --- SKILLS (if available) ---
    const skills = data.skills || [];
    const soulTier = data.soul_tier || '';

    let skillsHTML = '';
    if (skills.length > 0) {
        skillsHTML = `
        <div class="skills-section">
            <h2 class="section-title">⚔️ Talentos & Habilidades ${soulTier ? `<span class="tier-tag">${soulTier}</span>` : ''}</h2>
            <div class="skills-grid">
                ${skills.map(s => {
            const catColor = s.category === 'Active' ? '#4169E1' :
                s.category === 'Passive' ? '#32CD32' :
                    s.category === 'Ultimate' ? '#FFD700' : '#9932CC';
            return `
                    <div class="skill-card" style="border-left-color: ${catColor};">
                        <div class="skill-header">
                            <span class="skill-name">${s.name}</span>
                            <span class="skill-badge">${s.category}</span>
                        </div>
                        <div class="skill-desc">${s.description}</div>
                        <div class="skill-meta">
                            ${s.cost > 0 ? `<span>💠 ${s.cost} MP</span>` : ''}
                            <span>⚡ Poder: ${s.power_level}/10</span>
                        </div>
                    </div>`;
        }).join('')}
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
        :root { --paper: #f5f0e6; --ink: #0a0a0a; --accent: #8b4513; --blood: #6b1818; --gold: #c5a059; --purple: #5a2d82; }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { background-color: #1a1a1a; padding: 40px 20px; font-family: 'Crimson Text', serif; color: var(--ink); min-height: 100vh; display: flex; justify-content: center; font-size: 18px; line-height: 1.6; }
        .story-sheet { width: 100%; max-width: 950px; background: var(--paper); box-shadow: 0 0 80px rgba(0,0,0,0.6); padding: 60px; border-radius: 6px; }
        
        .header { text-align: center; border-bottom: 4px double var(--accent); padding-bottom: 30px; margin-bottom: 35px; }
        .role-badge { display: inline-block; background: var(--accent); color: white; padding: 6px 20px; border-radius: 4px; font-family: 'Cinzel', serif; font-size: 0.85rem; font-weight: 700; margin-bottom: 12px; letter-spacing: 2px; }
        h1 { font-family: 'Cinzel', serif; font-size: 3.2rem; color: var(--ink); margin: 12px 0; letter-spacing: 2px; font-weight: 900; text-shadow: 1px 1px 2px rgba(0,0,0,0.1); }
        .subtitle { font-family: 'Metamorphous', cursive; color: var(--accent); font-style: italic; font-size: 1.3rem; font-weight: 600; }
        .meta-info { font-size: 1rem; color: #444; margin-top: 12px; font-weight: 500; }
        
        .quote-block { border-left: 5px solid var(--purple); background: rgba(90,45,130,0.08); padding: 25px 30px; margin: 30px 0; font-size: 1.35rem; font-style: italic; color: #222; font-weight: 500; border-radius: 0 8px 8px 0; }
        
        .section-title { font-family: 'Cinzel', serif; font-size: 1.6rem; color: var(--blood); border-bottom: 3px solid var(--accent); padding-bottom: 10px; margin: 40px 0 25px 0; display: flex; align-items: center; gap: 12px; font-weight: 700; }
        .tier-tag { background: var(--gold); color: #000; padding: 4px 12px; font-size: 0.75rem; border-radius: 4px; font-weight: 700; }
        
        .appearance-box { background: rgba(0,0,0,0.04); border: 2px solid #bbb; padding: 25px; border-radius: 10px; margin-bottom: 30px; }
        .appear-item { margin-bottom: 15px; font-size: 1.1rem; }
        .appear-label { font-weight: 700; color: var(--accent); margin-right: 8px; }
        
        .psych-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 18px; margin: 25px 0; }
        .psych-item { background: rgba(0,0,0,0.05); border: 2px solid #aaa; border-left: 5px solid var(--accent); padding: 18px; border-radius: 0 8px 8px 0; }
        .psych-label { font-weight: 800; color: var(--blood); font-size: 1rem; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 1.5px; }
        .psych-text { font-style: italic; color: #222; line-height: 1.6; font-size: 1.05rem; font-weight: 500; }
        .psych-full { grid-column: 1 / -1; }
        
        .arc-box { background: linear-gradient(to right, rgba(90,45,130,0.08), transparent); border: 2px solid var(--purple); padding: 25px; margin: 25px 0; border-radius: 10px; }
        .arc-item { margin-bottom: 18px; padding-bottom: 18px; border-bottom: 2px dashed #ccc; }
        .arc-item:last-child { border-bottom: none; margin-bottom: 0; padding-bottom: 0; }
        .arc-label { font-weight: 800; color: var(--purple); font-size: 0.95rem; }
        .arc-text { margin-top: 8px; font-style: italic; font-size: 1.05rem; font-weight: 500; color: #222; }
        
        .bio-phase h3 { font-family: 'Cinzel', serif; color: var(--blood); font-size: 1.4rem; margin-bottom: 12px; font-weight: 700; }
        .bio-phase p { text-align: justify; line-height: 1.9; margin-bottom: 18px; font-size: 1.1rem; font-weight: 500; color: #111; }
        .separator { text-align: center; color: var(--accent); opacity: 0.6; margin: 30px 0; font-size: 1.4rem; }
        
        .hooks-section { background: rgba(0,0,0,0.04); border: 2px solid #bbb; padding: 30px; border-radius: 10px; margin-top: 35px; }
        .hook-item { border-bottom: 2px dashed #ccc; padding: 15px 0; color: #111; font-size: 1.15rem; font-weight: 500; }
        .hook-item:last-child { border-bottom: none; }
        
        .skills-section { margin-top: 40px; border-top: 3px solid var(--gold); padding-top: 30px; }
        .skills-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 18px; }
        .skill-card { background: rgba(0,0,0,0.04); padding: 18px; border-left: 5px solid #666; border-radius: 0 8px 8px 0; border: 1px solid #bbb; }
        .skill-header { display: flex; justify-content: space-between; margin-bottom: 10px; }
        .skill-name { font-family: 'Cinzel', serif; font-weight: 800; font-size: 1.1rem; }
        .skill-badge { font-size: 0.7rem; padding: 3px 8px; background: rgba(0,0,0,0.12); border-radius: 4px; text-transform: uppercase; font-weight: 700; }
        .skill-desc { font-size: 1rem; font-style: italic; color: #333; margin-bottom: 10px; line-height: 1.5; font-weight: 500; }
        .skill-meta { display: flex; gap: 18px; font-size: 0.9rem; color: #555; font-weight: 600; }
        
        .footer { text-align: center; margin-top: 45px; color: #666; font-size: 0.9rem; border-top: 2px solid #ccc; padding-top: 25px; font-weight: 500; }
        .download-btn { display: block; width: fit-content; margin: 30px auto 0; background: var(--ink); color: var(--paper); border: 3px solid var(--accent); padding: 15px 40px; font-family: 'Cinzel', serif; cursor: pointer; border-radius: 8px; font-size: 1rem; font-weight: 700; transition: all 0.3s; }
        .download-btn:hover { background: var(--accent); color: white; transform: scale(1.02); }
        
        @media (max-width: 700px) { .psych-grid, .skills-grid { grid-template-columns: 1fr; } .story-sheet { padding: 30px; } h1 { font-size: 2.2rem; } body { font-size: 16px; } }
    </style>
</head>
<body>
    <div class="story-sheet">
        <div class="header">
            <div class="role-badge">${rol.toUpperCase()}</div>
            <h1>${name}</h1>
            ${titulo ? `<div class="subtitle">"${titulo}"</div>` : ''}
            <div class="meta-info">${mundo} · ${genero ? genero + ' · ' : ''}${edad} ${L.years}</div>
        </div>
        
        <div class="quote-block">
            "${fraseTexto}"
        </div>
        
        <h2 class="section-title">${L.firstImpression}</h2>
        <div class="appearance-box">
            <div class="appear-item"><span class="appear-label">${L.clothing}:</span> ${vestimenta}</div>
            <div class="appear-item"><span class="appear-label">${L.voice}:</span> ${voz}</div>
            <div class="appear-item"><span class="appear-label">${L.trait}:</span> ${rasgo}</div>
            ${manierismo ? `<div class="appear-item"><span class="appear-label">${L.mannerism}:</span> ${manierismo}</div>` : ''}
            <div class="appear-item"><span class="appear-label">${L.behavior}:</span> ${mascaraTexto}</div>
        </div>
        
        <h2 class="section-title">${L.psychology}</h2>
        <div class="psych-grid">
            <div class="psych-item">
                <div class="psych-label">${L.wound}</div>
                <div class="psych-text">${heridaTexto}</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.mask}</div>
                <div class="psych-text">${mascaraTexto}</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.fear}</div>
                <div class="psych-text">${miedoTexto}</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.secret}</div>
                <div class="psych-text">${secretoTexto}</div>
            </div>
            <div class="psych-item psych-full">
                <div class="psych-label">${L.shadow}</div>
                <div class="psych-text">${sombraTexto} <br><em style="color:#888;">${L.emerges}: ${sombraTrigger}</em></div>
            </div>
        </div>
        
        <h2 class="section-title">${L.conflict}</h2>
        <div class="psych-grid">
            <div class="psych-item">
                <div class="psych-label">${L.wants}</div>
                <div class="psych-text">${deseoTexto}</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.needs}</div>
                <div class="psych-text">${necesidadTexto}</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.lie}</div>
                <div class="psych-text">"${mentiraTexto}"</div>
            </div>
            <div class="psych-item">
                <div class="psych-label">${L.truth}</div>
                <div class="psych-text">${verdadTexto}</div>
            </div>
        </div>
        
        <h2 class="section-title">${L.arc}</h2>
        <div class="arc-box">
            <div class="arc-item">
                <div class="arc-label">${L.initial}</div>
                <div class="arc-text">${arcoInicial}</div>
            </div>
            <div class="arc-item">
                <div class="arc-label">${L.turning}</div>
                <div class="arc-text">${arcoQuiebre}</div>
            </div>
            <div class="arc-item">
                <div class="arc-label">${L.ifWins}</div>
                <div class="arc-text">${arcoPositivo}</div>
            </div>
            <div class="arc-item">
                <div class="arc-label">${L.ifFails}</div>
                <div class="arc-text">${arcoTragico}</div>
            </div>
        </div>
        
        <h2 class="section-title">${L.bio}</h2>
        ${bioHTML}
        
        ${skillsHTML}

        <div class="hooks-section">
            <h2 class="section-title" style="margin-top:0; border-bottom:none;">${L.hooks}</h2>
            ${ganchosHTML}
            ${momentosHTML ? `
            <h3 style="font-family:'Cinzel'; color:var(--purple); margin-top:20px; font-size:1.1rem;">${L.moments}</h3>
            ${momentosHTML}
            ` : ''}
        </div>
        
        <button class="download-btn" onclick="window.print()">🖨️ ${L.print}</button>
        <div class="footer">SoulForge Engine | ${new Date().toLocaleDateString()}</div>
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
