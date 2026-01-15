export const API_BASE = 'https://soulforge.up.railway.app';
const PAYPAL_ME_LINK = 'https://paypal.me/YOURPAYPAL/5';

// --- TOKEN SYSTEM ---
export const TokenSystem = {
    get() {
        return parseInt(localStorage.getItem('soulforge_tokens') || '0');
    },
    
    set(n) {
        localStorage.setItem('soulforge_tokens', n.toString());
        this.updateDisplay();
    },
    
    use() {
        const current = this.get();
        if (current > 0) {
            this.set(current - 1);
            return true;
        }
        return false;
    },
    
    add(n) {
        this.set(this.get() + n);
    },

    updateDisplay() {
        const display = document.getElementById('creditsDisplay');
        const count = document.getElementById('creditsCount');
        if (!display || !count) return;

        const tokens = this.get();
        if (tokens > 0) {
            display.style.display = 'block';
            count.textContent = tokens;
        } else {
            // Optional: hide or show 0
            display.style.display = 'none'; 
        }
    }
};

// --- API DOWNLOAD ---
export async function fetchCharacter(params) {
    const response = await fetch(`${API_BASE}/api/v1/personaje?${params}`);
    if (!response.ok) throw new Error('Error del servidor');
    return await response.json();
}

export async function fetchConstellation(params) {
    const response = await fetch(`${API_BASE}/api/v1/constelacion?${params}`);
    if (!response.ok) throw new Error('Error del servidor');
    return await response.json();
}
