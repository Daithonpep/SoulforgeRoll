import { TokenSystem } from './api.js';

export function showPayPalModal(type) {
    alert("MODO PRUEBA: Tokens recargados automáticamente.");
    TokenSystem.set(5);
}

export function changeLanguage(lang) {
    // Simplified i18n
    if (!['es', 'en', 'jp'].includes(lang)) lang = 'es';
    history.replaceState(null, null, '#' + lang);
    // ... Implement full translation logic if needed
    console.log('Language changed to:', lang);
}

// Attach specific UI functions to window
window.changeLanguage = changeLanguage;
