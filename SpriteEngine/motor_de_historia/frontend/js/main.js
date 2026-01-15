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

    // 2. Loading
    document.body.style.cursor = 'wait';
    const btn = document.getElementById('btnPersonaje');
    if (btn) btn.innerText = 'Forjando...';

    try {
        // 3. API Call
        const personaje = await fetchCharacter(params);

        // 4. RPG Process
        const rpgChar = rpgEngine.process(personaje, formData);

        // 5. Redirect to Forge View (Visual)
        // Store data to session storage to pass to the visualizer
        sessionStorage.setItem('lastForgedCharacter', JSON.stringify(rpgChar));

        window.location.href = 'forja_de_almas.html'; // Redirect to Crystal Animation

    } catch (e) {
        console.error(e);
        alert('Error: ' + e.message);
        TokenSystem.add(1); // Refund
    } finally {
        document.body.style.cursor = 'default';
        if (btn) btn.innerHTML = '<span style="font-size: 1.2rem;">⚡</span> FORJAR ALMA';
    }
}

// Attach to window for HTML access
window.handleForjarAlma = handleForjarAlma;
