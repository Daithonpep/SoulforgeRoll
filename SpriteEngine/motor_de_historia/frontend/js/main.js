import { TokenSystem, fetchCharacter, fetchConstellation } from './api.js';
import { SoulRPGEngine } from './engine.js';
import { showPayPalModal } from './ui.js';
import { getLabels } from './i18n-sheet.js';

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
    // DETECT LANGUAGE AND GET LABELS
    const lang = data.idioma || data.lang || 'es';
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
        :root { --paper: #e3dac9; --ink: #1a1a1a; --accent: #8b4513; --blood: #722f37; --gold: #c5a059; --purple: #663399; }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { background-color: #2a2a2a; padding: 40px 20px; font-family: 'Crimson Text', serif; color: var(--ink); min-height: 100vh; display: flex; justify-content: center; }
        .story-sheet { width: 100%; max-width: 900px; background: var(--paper); background-image: url('https://www.transparenttextures.com/patterns/aged-paper.png'); box-shadow: 0 0 60px rgba(0,0,0,0.5); padding: 50px; border-radius: 4px; }
        
        .header { text-align: center; border-bottom: 3px double var(--accent); padding-bottom: 25px; margin-bottom: 30px; }
        .role-badge { display: inline-block; background: var(--accent); color: var(--paper); padding: 4px 15px; border-radius: 4px; font-family: 'Cinzel'; font-size: 0.8rem; margin-bottom: 10px; }
        h1 { font-family: 'Cinzel'; font-size: 2.8rem; color: var(--ink); margin: 10px 0; letter-spacing: 1px; }
        .subtitle { font-family: 'Metamorphous'; color: var(--accent); font-style: italic; font-size: 1.1rem; }
        .meta-info { font-size: 0.9rem; color: #666; margin-top: 10px; }
        
        .quote-block { border-left: 4px solid var(--purple); background: rgba(102,51,153,0.05); padding: 20px; margin: 25px 0; font-size: 1.2rem; font-style: italic; color: #333; }
        
        .section-title { font-family: 'Cinzel'; font-size: 1.4rem; color: var(--blood); border-bottom: 2px solid var(--accent); padding-bottom: 8px; margin: 35px 0 20px 0; display: flex; align-items: center; gap: 10px; }
        .tier-tag { background: var(--gold); color: #000; padding: 2px 8px; font-size: 0.7rem; border-radius: 4px; }
        
        .appearance-box { background: rgba(0,0,0,0.03); border: 1px solid #ccc; padding: 20px; border-radius: 8px; margin-bottom: 25px; }
        .appear-item { margin-bottom: 12px; }
        .appear-label { font-weight: bold; color: var(--accent); }
        
        .psych-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin: 20px 0; }
        .psych-item { background: rgba(0,0,0,0.04); border: 1px solid #ccc; border-left: 4px solid var(--accent); padding: 15px; border-radius: 0 6px 6px 0; }
        .psych-label { font-weight: bold; color: var(--blood); font-size: 0.9rem; margin-bottom: 5px; text-transform: uppercase; letter-spacing: 1px; }
        .psych-text { font-style: italic; color: #444; line-height: 1.5; }
        .psych-full { grid-column: 1 / -1; }
        
        .arc-box { background: linear-gradient(to right, rgba(102,51,153,0.05), transparent); border: 1px solid var(--purple); padding: 20px; margin: 20px 0; border-radius: 8px; }
        .arc-item { margin-bottom: 15px; padding-bottom: 15px; border-bottom: 1px dashed #ccc; }
        .arc-item:last-child { border-bottom: none; margin-bottom: 0; padding-bottom: 0; }
        .arc-label { font-weight: bold; color: var(--purple); font-size: 0.85rem; }
        .arc-text { margin-top: 5px; font-style: italic; }
        
        .bio-phase h3 { font-family: 'Cinzel'; color: var(--blood); font-size: 1.2rem; margin-bottom: 10px; }
        .bio-phase p { text-align: justify; line-height: 1.8; margin-bottom: 15px; }
        .separator { text-align: center; color: var(--accent); opacity: 0.5; margin: 25px 0; font-size: 1.2rem; }
        
        .hooks-section { background: rgba(0,0,0,0.03); border: 1px solid #ccc; padding: 25px; border-radius: 8px; margin-top: 30px; }
        .hook-item { border-bottom: 1px dashed #ccc; padding: 12px 0; color: #333; font-size: 1.05rem; }
        .hook-item:last-child { border-bottom: none; }
        
        .skills-section { margin-top: 35px; border-top: 2px solid var(--gold); padding-top: 25px; }
        .skills-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
        .skill-card { background: rgba(0,0,0,0.03); padding: 15px; border-left: 4px solid #666; border-radius: 0 6px 6px 0; }
        .skill-header { display: flex; justify-content: space-between; margin-bottom: 8px; }
        .skill-name { font-family: 'Cinzel'; font-weight: bold; font-size: 1rem; }
        .skill-badge { font-size: 0.65rem; padding: 2px 6px; background: rgba(0,0,0,0.1); border-radius: 4px; text-transform: uppercase; }
        .skill-desc { font-size: 0.9rem; font-style: italic; color: #444; margin-bottom: 8px; line-height: 1.4; }
        .skill-meta { display: flex; gap: 15px; font-size: 0.8rem; color: #666; }
        
        .footer { text-align: center; margin-top: 40px; color: #888; font-size: 0.8rem; border-top: 1px solid #ccc; padding-top: 20px; }
        .download-btn { display: block; width: fit-content; margin: 25px auto 0; background: var(--ink); color: var(--paper); border: 2px solid var(--accent); padding: 12px 30px; font-family: 'Cinzel'; cursor: pointer; border-radius: 8px; }
        .download-btn:hover { background: var(--accent); color: var(--paper); }
        
        @media (max-width: 700px) { .psych-grid, .skills-grid { grid-template-columns: 1fr; } .story-sheet { padding: 25px; } h1 { font-size: 2rem; } }
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
