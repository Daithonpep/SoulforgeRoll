import { TokenSystem } from './api.js';

// --- IMMERSIVE TRANSLATIONS FOR UNLOCK MODAL ---
const UI_TEXTS = {
    es: {
        title: '🔓 Desbloquear Creación',
        desc: 'Este personaje espera cobrar vida.',
        desc2: 'Completa el proceso para darle forma en el mundo.',
        price: 'Costo: $4.99 USD',
        btn: '🔓 Desbloquear Ahora',
        processing: '✨ Verificando pago...',
        token: '¿Ya tienes un código? Ingrésalo aquí:',
        tokenBtn: 'Activar',
        close: 'Volver',
        success: '✨ ¡Desbloqueado! Tu personaje está listo.',
        error: 'El código no es válido. Intenta de nuevo.'
    },
    en: {
        title: '🔓 Unlock Creation',
        desc: 'This character is waiting to come alive.',
        desc2: 'Complete the process to bring it into the world.',
        price: 'Cost: $4.99 USD',
        btn: '🔓 Unlock Now',
        processing: '✨ Verifying payment...',
        token: 'Already have a code? Enter it here:',
        tokenBtn: 'Activate',
        close: 'Go Back',
        success: '✨ Unlocked! Your character is ready.',
        error: 'The code is not valid. Try again.'
    },
    jp: {
        title: '🔓 クリエーションをアンロック',
        desc: 'このキャラクターは命を吹き込まれるのを待っています。',
        desc2: 'プロセスを完了して、世界に形を与えましょう。',
        price: '費用: $4.99 USD',
        btn: '🔓 今すぐアンロック',
        processing: '✨ 支払いを確認中...',
        token: 'コードをお持ちですか？ここに入力:',
        tokenBtn: '有効化',
        close: '戻る',
        success: '✨ アンロック完了！キャラクターの準備ができました。',
        error: 'コードが無効です。もう一度お試しください。'
    }
};

// Get current page URL for PayPal return
function getReturnUrl() {
    const base = window.location.origin + window.location.pathname;
    return base + '?payment=success';
}

// Check if user is returning from PayPal
export function checkPaymentReturn() {
    const params = new URLSearchParams(window.location.search);
    if (params.get('payment') === 'success') {
        // User just returned from PayPal - show success and grant tokens
        const pendingAction = sessionStorage.getItem('soulforge_pending_action');

        // Grant tokens
        TokenSystem.add(1);

        // Clean URL
        window.history.replaceState({}, document.title, window.location.pathname);

        // Show success notification
        showSuccessNotification();

        // Auto-execute pending action after short delay
        if (pendingAction) {
            sessionStorage.removeItem('soulforge_pending_action');
            setTimeout(() => {
                if (pendingAction === 'forjar_alma') {
                    const btn = document.getElementById('btnPersonaje');
                    if (btn) btn.click();
                } else if (pendingAction === 'forjar_constelacion') {
                    const btn = document.getElementById('btnConstelacion');
                    if (btn) btn.click();
                }
            }, 1500);
        }

        return true;
    }
    return false;
}

function showSuccessNotification() {
    const lang = localStorage.getItem('soulforge_lang') || 'es';
    const t = UI_TEXTS[lang] || UI_TEXTS.es;

    const notif = document.createElement('div');
    notif.innerHTML = `
        <div style="position:fixed;top:20px;left:50%;transform:translateX(-50%);background:linear-gradient(135deg,#1a1a2e,#16213e);border:2px solid #4ade80;border-radius:12px;padding:20px 40px;z-index:10000;box-shadow:0 10px 40px rgba(74,222,128,0.3);animation:slideDown 0.5s ease;">
            <p style="color:#4ade80;font-family:'Cinzel',serif;font-size:1.2rem;margin:0;text-shadow:0 0 10px rgba(74,222,128,0.5);">
                ${t.success}
            </p>
        </div>
        <style>
            @keyframes slideDown {
                from { opacity:0; transform:translateX(-50%) translateY(-20px); }
                to { opacity:1; transform:translateX(-50%) translateY(0); }
            }
        </style>
    `;
    document.body.appendChild(notif);
    setTimeout(() => notif.remove(), 4000);
}

export function showPayPalModal(type) {
    // Get current lang
    let lang = localStorage.getItem('soulforge_lang') || 'es';
    if (location.hash.includes('#en')) lang = 'en';
    if (location.hash.includes('#jp')) lang = 'jp';

    const t = UI_TEXTS[lang] || UI_TEXTS.es;

    // Save what user was trying to do
    sessionStorage.setItem('soulforge_pending_action', type === 'alma' ? 'forjar_alma' : 'forjar_constelacion');

    // Remove existing modal
    const existing = document.getElementById('paypalModal');
    if (existing) existing.remove();

    const returnUrl = getReturnUrl();

    const modal = document.createElement('div');
    modal.id = 'paypalModal';
    modal.innerHTML = `
        <div style="position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,0.92);z-index:9999;display:flex;align-items:center;justify-content:center;backdrop-filter:blur(8px);">
            <div style="background:linear-gradient(180deg,#1a1a2e 0%,#0f0f1a 100%);border:2px solid var(--accent);border-radius:20px;padding:40px;max-width:420px;text-align:center;box-shadow:0 0 80px rgba(212,175,55,0.2),inset 0 0 30px rgba(0,0,0,0.5);">
                
                <!-- Unlock Circle Decoration -->
                <div style="width:80px;height:80px;margin:0 auto 20px;border:3px solid var(--accent);border-radius:50%;display:flex;align-items:center;justify-content:center;animation:pulse 2s infinite;box-shadow:0 0 20px rgba(212,175,55,0.3);">
                    <span style="font-size:2.5rem;">🔓</span>
                </div>
                
                <h2 style="color:var(--accent);margin:0 0 15px;font-family:'Cinzel',serif;font-size:1.6rem;text-shadow:0 0 20px rgba(212,175,55,0.5);">${t.title}</h2>
                
                <p style="color:#ccc;margin-bottom:8px;font-size:1.1rem;font-style:italic;">${t.desc}</p>
                <p style="color:#888;margin-bottom:25px;line-height:1.6;">${t.desc2}</p>
                
                <div style="background:rgba(74,222,128,0.1);border:1px solid rgba(74,222,128,0.3);padding:15px;border-radius:10px;margin-bottom:25px;">
                    <p style="color:#4ade80;font-weight:bold;margin:0;font-size:1.1rem;">${t.price}</p>
                </div>
                
                <!-- PAYPAL BUTTON with Return URL -->
                <form action="https://www.paypal.com/cgi-bin/webscr" method="post" target="_top" style="margin-bottom:25px;">
                    <input type="hidden" name="cmd" value="_s-xclick" />
                    <input type="hidden" name="hosted_button_id" value="AUH69VTF7QFH8" />
                    <input type="hidden" name="currency_code" value="USD" />
                    <input type="hidden" name="return" value="${returnUrl}" />
                    <input type="hidden" name="cancel_return" value="${window.location.href}" />
                    <button type="submit" style="background:linear-gradient(135deg,#d4af37,#f4d03f);color:#000;border:none;padding:15px 40px;border-radius:30px;font-weight:bold;cursor:pointer;font-size:1.1rem;width:100%;transition:all 0.3s;box-shadow:0 5px 20px rgba(212,175,55,0.4);font-family:'Cinzel',serif;">
                        ${t.btn}
                    </button>
                </form>
                
                <div style="border-top:1px solid #333;padding-top:20px;margin-top:10px;">
                    <p style="color:#555;font-size:0.85rem;margin-bottom:10px;">${t.token}</p>
                    <div style="display:flex;gap:10px;">
                        <input type="text" id="tokenInput" placeholder="XXXX-XXXX-XXXX" 
                               style="flex:1;padding:12px;background:#0a0a0f;border:1px solid #333;border-radius:8px;color:white;text-align:center;font-family:monospace;font-size:1rem;letter-spacing:2px;">
                        <button id="btnActivate" style="padding:12px 20px;background:linear-gradient(135deg,#333,#444);color:white;border:1px solid #555;border-radius:8px;cursor:pointer;font-weight:bold;">
                            ✓
                        </button>
                    </div>
                </div>
                
                <button id="btnCloseModal" 
                        style="margin-top:25px;background:transparent;border:none;color:#555;cursor:pointer;font-size:0.9rem;">
                    ${t.close}
                </button>
            </div>
        </div>
        <style>
            @keyframes pulse {
                0%, 100% { box-shadow: 0 0 20px rgba(212,175,55,0.3); }
                50% { box-shadow: 0 0 40px rgba(212,175,55,0.6); }
            }
        </style>
    `;
    document.body.appendChild(modal);

    // Event Listeners
    document.getElementById('btnCloseModal').onclick = () => {
        sessionStorage.removeItem('soulforge_pending_action');
        modal.remove();
    };

    document.getElementById('btnActivate').onclick = () => {
        const code = document.getElementById('tokenInput').value.trim().toUpperCase();

        // Validate code format (at least 8 characters, alphanumeric with dashes)
        if (code.length >= 8 && /^[A-Z0-9-]+$/.test(code)) {
            TokenSystem.add(1);
            modal.remove();
            showSuccessNotification();

            // Auto-trigger the pending action
            const pendingAction = sessionStorage.getItem('soulforge_pending_action');
            sessionStorage.removeItem('soulforge_pending_action');

            setTimeout(() => {
                if (pendingAction === 'forjar_alma') {
                    const btn = document.getElementById('btnPersonaje');
                    if (btn) btn.click();
                } else if (pendingAction === 'forjar_constelacion') {
                    const btn = document.getElementById('btnConstelacion');
                    if (btn) btn.click();
                }
            }, 1000);
        } else {
            alert(t.error);
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
            el.innerHTML = translations[key];
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
            hierarchy_title: "🏆 Jerarquía de Poder",
            hierarchy_desc: "El sistema de cartas no es solo estético; define el potencial narrativo y mecánico de tu personaje. Las cartas de alto nivel (SSS) no solo otorgan mejores estadísticas, sino que alteran las reglas del juego.",
            class_eco: '<strong style="color: #2E8B57;">Clase Eco (C-B):</strong> Sobrevivientes. Stats base. Tecnología estándar.',
            class_shadow: '<strong style="color: var(--accent);">Clase Sombra (A):</strong> Agentes de élite. Acceso a perks ocultos.',
            class_primordial: '<strong style="color: #c026d3;">Clase Primordial (SSS):</strong> Entidades cósmicas. Rompen las reglas.',
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
            hierarchy_title: "🏆 Power Hierarchy",
            hierarchy_desc: "The card system is not just aesthetic; it defines the narrative and mechanical potential of your character. High-tier cards (SSS) not only grant better stats but alter the rules of the game.",
            class_eco: '<strong style="color: #2E8B57;">Echo Class (C-B):</strong> Survivors. Base stats. Standard tech.',
            class_shadow: '<strong style="color: var(--accent);">Shadow Class (A):</strong> Elite agents. Access to hidden perks.',
            class_primordial: '<strong style="color: #c026d3;">Primordial Class (SSS):</strong> Cosmic entities. They break the rules.',
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
            hierarchy_title: "🏆 力の階層",
            hierarchy_desc: "カードシステムは単なる見た目ではありません。キャラクターの物語的および機械的な可能性を定義します。高ランクのカード（SSS）は、ステータスが高いだけでなく、ゲームのルールそのものを変えます。",
            class_eco: '<strong style="color: #2E8B57;">エコクラス (C-B):</strong> 生存者。基本ステータス。標準技術。',
            class_shadow: '<strong style="color: var(--accent);">シャドウクラス (A):</strong> エリートエージェント。隠しパークへのアクセス。',
            class_primordial: '<strong style="color: #c026d3;">プリモーディアルクラス (SSS):</strong> 宇宙的存在。ルールを破壊する。',
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
