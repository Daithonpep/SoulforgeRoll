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
    
    // 1. Save to localStorage for persistence
    localStorage.setItem('soulforge_lang', lang);
    
    // 2. Update URL hash
    history.replaceState(null, null, '#' + lang);
    
    // 3. Update hidden form fields (critical for backend)
    const langPersonaje = document.getElementById('inputLangPersonaje');
    const langConstelacion = document.getElementById('inputLangConstelacion');
    if (langPersonaje) langPersonaje.value = lang;
    if (langConstelacion) langConstelacion.value = lang;
    
    // 4. Update all text elements with data-i18n attribute
    applyTranslations(lang);
    
    console.log(`🌐 Idioma cambiado a: ${lang}`);
}

// Initialize language on page load
export function initLanguage() {
    // Priority: URL hash > localStorage > default 'es'
    let lang = 'es';
    
    if (location.hash) {
        const hashLang = location.hash.replace('#', '');
        if (['es', 'en', 'jp'].includes(hashLang)) {
            lang = hashLang;
        }
    } else {
        const stored = localStorage.getItem('soulforge_lang');
        if (stored && ['es', 'en', 'jp'].includes(stored)) {
            lang = stored;
        }
    }
    
    changeLanguage(lang);
}

// Apply translations to all data-i18n elements
function applyTranslations(lang) {
    const translations = getTranslations(lang);
    
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (translations[key]) {
            el.textContent = translations[key];
        }
    });
    
    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        if (translations[key]) {
            el.placeholder = translations[key];
        }
    });
}

// Translation dictionary (mirrors i18n.rs)
function getTranslations(lang) {
    const TRANSLATIONS = {
        es: {
            hero_tagline: "Sistema de Forja de Almas",
            index_title: "Motor SoulForge",
            index_subtitle: "Generación Narrativa Procedural & Sistema de Psicología Profunda",
            card_alma_title: "⚔️ Forjar Alma",
            card_alma_desc: "Crea un personaje único con profundidad psicológica.",
            label_nombre: "NOMBRE DEL PERSONAJE",
            label_nombre_placeholder: "Ej: Kael (vacío = aleatorio)",
            genero: "GÉNERO",
            rol: "ROL",
            label_mundo: "MUNDO",
            label_tono: "TONO",
            btn_forjar_alma: "FORJAR ALMA",
            card_constelacion_title: "🌌 Forjar Constelación",
            card_constelacion_desc: "Genera un grupo de personajes interconectados.",
            label_tamano_grupo: "TAMAÑO DEL GRUPO",
            label_personalizar: "Personalizar",
            label_motivacion_villano: "MOTIVACIÓN DEL VILLANO",
            label_romance: "ROMANCE",
            label_densidad: "DENSIDAD",
            btn_forjar_constelacion: "FORJAR CONSTELACIÓN",
            how_it_works_title: "¿Cómo Funciona?",
            how_it_works_subtitle: "Tu viaje de la idea al personaje.",
            step1_title: "1. Configura",
            step1_desc: "Elige género, mundo... o déjalo al azar.",
            step2_title: "2. Forja",
            step2_desc: "La IA genera psicología profunda e historia.",
            step3_title: "3. Descarga",
            step3_desc: "Obtén una ficha HTML premium lista para usar.",
            demos_title: "Ejemplos de Personajes",
            demos_subtitle: "Mira la profundidad y complejidad.",
            demo_click_cta: "VER FICHA COMPLETA",
            aviso_legal_titulo: "Aviso Legal",
            aviso_legal_1: "SoulForge es una herramienta de creación asistida.",
            aviso_legal_2: "Los personajes son generados dinámicamente.",
            aviso_legal_3: "Es responsabilidad del usuario descargar el archivo.",
            aviso_legal_4: "El uso implica que los resultados varían.",
            footer_dev: "Developed with ❤️ by DAITHON BENEDICTUS",
            edad: "EDAD",
            label_clase: "CLASE / OFICIO"
        },
        en: {
            hero_tagline: "Soul Forging System",
            index_title: "SoulForge Engine",
            index_subtitle: "Procedural Narrative Generation & Deep Psychology System",
            card_alma_title: "⚔️ Forge Soul",
            card_alma_desc: "Create a unique character with psychological depth.",
            label_nombre: "CHARACTER NAME",
            label_nombre_placeholder: "Ex: Kael (empty = random)",
            genero: "GENDER",
            rol: "ROLE",
            label_mundo: "WORLD",
            label_tono: "TONE",
            btn_forjar_alma: "FORGE SOUL",
            card_constelacion_title: "🌌 Forge Constellation",
            card_constelacion_desc: "Generate a group of interconnected characters.",
            label_tamano_grupo: "GROUP SIZE",
            label_personalizar: "Customize",
            label_motivacion_villano: "VILLAIN MOTIVATION",
            label_romance: "ROMANCE",
            label_densidad: "DENSITY",
            btn_forjar_constelacion: "FORGE CONSTELLATION",
            how_it_works_title: "How Does It Work?",
            how_it_works_subtitle: "Your journey from idea to character.",
            step1_title: "1. Configure",
            step1_desc: "Choose gender, world... or leave it random.",
            step2_title: "2. Forge",
            step2_desc: "AI generates deep psychology and history.",
            step3_title: "3. Download",
            step3_desc: "Get a premium HTML sheet ready to use.",
            demos_title: "Character Examples",
            demos_subtitle: "See the depth and complexity.",
            demo_click_cta: "VIEW FULL SHEET",
            aviso_legal_titulo: "Legal Notice",
            aviso_legal_1: "SoulForge is an assisted creation tool.",
            aviso_legal_2: "Characters are dynamically generated.",
            aviso_legal_3: "User is responsible for downloading the file.",
            aviso_legal_4: "Usage implies results may vary.",
            footer_dev: "Developed with ❤️ by DAITHON BENEDICTUS",
            edad: "AGE",
            label_clase: "CLASS / PROFESSION"
        },
        jp: {
            hero_tagline: "魂の鍛造システム",
            index_title: "SoulForge エンジン",
            index_subtitle: "手続き型物語生成と深層心理システム",
            card_alma_title: "⚔️ 魂を鍛造する",
            card_alma_desc: "心理的な深みを持つユニークなキャラクターを作成。",
            label_nombre: "キャラクター名",
            label_nombre_placeholder: "例: カエル (空欄＝ランダム)",
            genero: "性別",
            rol: "役割",
            label_mundo: "世界設定",
            label_tono: "トーン",
            btn_forjar_alma: "魂を鍛造",
            card_constelacion_title: "🌌 星座を鍛造する",
            card_constelacion_desc: "相互接続されたキャラクターのグループを生成。",
            label_tamano_grupo: "グループサイズ",
            label_personalizar: "カスタマイズ",
            label_motivacion_villano: "敵対者の動機",
            label_romance: "ロマンス",
            label_densidad: "密度",
            btn_forjar_constelacion: "星座を鍛造",
            how_it_works_title: "どのように機能しますか？",
            how_it_works_subtitle: "アイデアからキャラクターへの旅。",
            step1_title: "1. 設定",
            step1_desc: "性別、世界を選択...またはランダムに。",
            step2_title: "2. 鍛造",
            step2_desc: "AIが深層心理と歴史を生成。",
            step3_title: "3. ダウンロード",
            step3_desc: "すぐに使えるプレミアムHTML。",
            demos_title: "キャラクター例",
            demos_subtitle: "深さと複雑さをご覧ください。",
            demo_click_cta: "完全なシートを見る",
            aviso_legal_titulo: "法的通知",
            aviso_legal_1: "SoulForgeは創作支援ツールです。",
            aviso_legal_2: "キャラクターは動的に生成されます。",
            aviso_legal_3: "ファイルのダウンロードはユーザーの責任です。",
            aviso_legal_4: "結果は異なる場合があります。",
            footer_dev: "Developed with ❤️ by DAITHON BENEDICTUS",
            edad: "年齢",
            label_clase: "クラス / 職業"
        }
    };
    
    return TRANSLATIONS[lang] || TRANSLATIONS.es;
}

// Exports for HTML onclick access
window.changeLanguage = changeLanguage;

// Auto-init on DOMContentLoaded
document.addEventListener('DOMContentLoaded', initLanguage);
