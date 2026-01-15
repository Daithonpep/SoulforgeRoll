import { TokenSystem } from './api.js';

// --- TRANSLATIONS FOR UI ---
const UI_TEXTS = {
    es: {
        title: '💳 Acceso Premium Requerido',
        desc: 'Para forjar este personaje, necesitas un token VIP.',
        price: 'Precio: $5 USD = 5 Tokens',
        btn: 'Pagar con PayPal',
        token: '¿Ya pagaste? Ingresa tu código:',
        tokenBtn: 'Activar Token',
        close: 'Cancelar'
    },
    en: {
        title: '💳 Premium Access Required',
        desc: 'To forge this character, you need a VIP token.',
        price: 'Price: $5 USD = 5 Tokens',
        btn: 'Pay with PayPal',
        token: 'Already paid? Enter your code:',
        tokenBtn: 'Activate Token',
        close: 'Cancel'
    },
    jp: {
        title: '💳 プレミアムアクセスが必要',
        desc: 'このキャラクターを鍛造するにはVIPトークンが必要です。',
        price: '価格: $5 USD = 5トークン',
        btn: 'PayPalで支払う',
        token: '支払い済み？コードを入力:',
        tokenBtn: 'トークンを有効化',
        close: 'キャンセル'
    }
};

export function showPayPalModal(type) {
    // Get current lang or default to spanish
    let lang = 'es';
    if (location.hash.includes('#en')) lang = 'en';
    if (location.hash.includes('#jp')) lang = 'jp';

    const t = UI_TEXTS[lang] || UI_TEXTS.es;

    // Remove existing if any
    const existing = document.getElementById('paypalModal');
    if (existing) existing.remove();

    const modal = document.createElement('div');
    modal.id = 'paypalModal';
    modal.innerHTML = `
        <div style="position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,0.85);z-index:9999;display:flex;align-items:center;justify-content:center;backdrop-filter:blur(5px);">
            <div style="background:#1a1a1f;border:1px solid var(--accent);border-radius:16px;padding:30px;max-width:400px;text-align:center;box-shadow:0 0 50px rgba(0,0,0,0.5);">
                <h2 style="color:var(--accent);margin-top:0;margin-bottom:15px;font-family:'Cinzel',serif;font-size:1.5rem;">${t.title}</h2>
                <p style="color:#aaa;margin-bottom:20px;line-height:1.5;">${t.desc}</p>
                <div style="background:rgba(255,255,255,0.05);padding:10px;border-radius:8px;margin-bottom:20px;">
                    <p style="color:#4ade80;font-weight:bold;margin:0;">${t.price}</p>
                </div>
                
                <!-- PAYPAL BUTTON -->
                <form action="https://www.paypal.com/cgi-bin/webscr" method="post" target="_blank" style="margin-bottom:20px;">
                    <input type="hidden" name="cmd" value="_s-xclick" />
                    <input type="hidden" name="hosted_button_id" value="AUH69VTF7QFH8" />
                    <input type="hidden" name="currency_code" value="USD" />
                    <button type="submit" style="background:#ffc439;color:#000;border:none;padding:12px 30px;border-radius:25px;font-weight:bold;cursor:pointer;font-size:1rem;width:100%;transition:transform 0.2s;">
                        Pay with <span style="font-weight:900;font-style:italic;">PayPal</span>
                    </button>
                </form>
                
                <div style="border-top:1px solid #333;padding-top:15px;margin-top:15px;">
                    <p style="color:#666;font-size:0.8rem;margin-bottom:8px;">${t.token}</p>
                    <div style="display:flex;gap:8px;">
                        <input type="text" id="tokenInput" placeholder="XXXX-XXXX" 
                               style="flex:1;padding:10px;background:#0a0a0f;border:1px solid #333;border-radius:6px;color:white;text-align:center;">
                        <button id="btnActivate" style="padding:10px 15px;background:#333;color:white;border:1px solid #555;border-radius:6px;cursor:pointer;">
                            OK
                        </button>
                    </div>
                </div>
                
                <button id="btnCloseModal" 
                        style="margin-top:20px;background:transparent;border:none;color:#666;cursor:pointer;text-decoration:underline;">
                    ${t.close}
                </button>
            </div>
        </div>
    `;
    document.body.appendChild(modal);

    // Event Listeners
    document.getElementById('btnCloseModal').onclick = () => modal.remove();

    document.getElementById('btnActivate').onclick = () => {
        const code = document.getElementById('tokenInput').value.trim();
        // SIMPLE VALIDATION FOR DEMO
        if (code.length > 4 || code === 'DEMO') {
            TokenSystem.add(5);
            alert("✅ Token Válido. +5 Créditos.");
            modal.remove();
            // Optional: Auto-retry logic could go here
        } else {
            alert("❌ Código inválido.");
        }
    };
}

export function changeLanguage(lang) {
    if (!['es', 'en', 'jp'].includes(lang)) lang = 'es';
    history.replaceState(null, null, '#' + lang);
    // Reload logic or text replacement logic would go here
    // For now we just reload to apply since i18n logic is partially in HTML
    // location.reload(); 
}
