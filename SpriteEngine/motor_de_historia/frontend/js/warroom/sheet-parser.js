// ============================================================
// PARSER DE FICHAS HTML
// ============================================================

export class SheetParser {
    /**
     * Extrae el JSON del personaje de un archivo HTML
     * @param {File} file 
     * @returns {Promise<{success: boolean, character?: Object, error?: string}>}
     */
    static async parseFromFile(file) {
        try {
            const content = await file.text();
            return this.parseFromHtml(content);
        } catch (e) {
            return {
                success: false,
                error: 'No se pudo leer el archivo',
            };
        }
    }

    /**
     * Parsea el contenido HTML y extrae los datos del personaje
     * @param {string} html 
     */
    static parseFromHtml(html) {
        try {
            // Buscar el script con los datos del personaje
            const scriptMatch = html.match(
                /<script[^>]*id=["']character-data["'][^>]*>([\s\S]*?)<\/script>/i
            );

            if (!scriptMatch) {
                // Intentar formato alternativo (data attribute en body o div principal)
                const dataMatch = html.match(
                    /data-character=["']([^"']+)["']/i
                );

                if (dataMatch) {
                    const decoded = this.decodeHtmlEntities(dataMatch[1]);
                    const data = JSON.parse(decoded);
                    return this.validateAndTransform(data);
                }

                // Intentar buscar JSON en algún script var characterData = ...
                const varMatch = html.match(/const\s+characterData\s*=\s*({[\s\S]*?});/);
                if (varMatch) {
                    try {
                        // Esto es peligroso con eval, pero asumimos origen confiable o parseamos como json si es puro
                        const data = JSON.parse(varMatch[1]);
                        return this.validateAndTransform(data);
                    } catch (e) { /* ignore */ }
                }

                return {
                    success: false,
                    error: 'No se encontraron datos del personaje en el archivo',
                };
            }

            const jsonContent = scriptMatch[1].trim();
            // A veces el contenido está en Base64 si es seguro
            let data;
            try {
                data = JSON.parse(jsonContent);
            } catch (e) {
                // Si falla, tal vez es base64 raw?
                try {
                    data = JSON.parse(atob(jsonContent));
                } catch (e2) {
                    throw new Error("JSON Inválido");
                }
            }

            return this.validateAndTransform(data);

        } catch (e) {
            return {
                success: false,
                error: `Error al parsear: ${e.message || 'Error desconocido'}`,
            };
        }
    }

    /**
     * Valida y transforma los datos al formato CharacterEssence
     */
    static validateAndTransform(data) {
        // Validar campos requeridos
        if (!data.name || typeof data.name !== 'string') {
            // Intentar buscar nombre en otros campos
            if (data.nombre) data.name = data.nombre;
            else return { success: false, error: 'El personaje no tiene nombre' };
        }

        // Extraer y normalizar datos
        const character = {
            name: data.name,
            portrait: data.portrait || data.image || data.avatar_url || undefined,
            tier: data.tier || data.soul_tier || 'D',
            tension_level: this.extractTension(data),
            current_wound: data.current_wound || (data.wounds && data.wounds[0]) || undefined,
            attributes: this.extractAttributes(data),
            special_traits: this.extractTraits(data),
        };

        return { success: true, character };
    }

    static extractTension(data) {
        if (typeof data.tension === 'number') return data.tension;
        if (typeof data.tension_level === 'number') return data.tension_level;
        if (data.soul?.tension) return data.soul.tension;
        if (data.mechanics?.soul_tension) return data.mechanics.soul_tension;
        return 0;
    }

    static extractAttributes(data) {
        const attrs = {};

        // Formato directo
        if (data.attributes && typeof data.attributes === 'object') {
            for (const [key, value] of Object.entries(data.attributes)) {
                if (typeof value === 'number') {
                    attrs[key] = value;
                }
            }
        }

        // Formato alternativo (stats o rpg_stats)
        const statsSource = data.stats || data.rpg_stats;
        if (statsSource && typeof statsSource === 'object') {
            for (const [key, value] of Object.entries(statsSource)) {
                if (typeof value === 'number') {
                    attrs[key] = value;
                }
            }
        }

        return attrs;
    }

    static extractTraits(data) {
        const traits = [];

        if (Array.isArray(data.traits)) {
            traits.push(...data.traits.filter(t => typeof t === 'string'));
        }

        if (Array.isArray(data.special_traits)) {
            traits.push(...data.special_traits.filter(t => typeof t === 'string'));
        }

        // Extraer de mechanics.soul_perks
        if (data.mechanics && Array.isArray(data.mechanics.soul_perks)) {
            for (const perk of data.mechanics.soul_perks) {
                if (perk.name) traits.push(`Perk: ${perk.name}`);
            }
        }

        // Extraer de máscaras
        if (Array.isArray(data.masks)) {
            for (const mask of data.masks) {
                if (mask.name) traits.push(`Máscara: ${mask.name}`);
            }
        }

        // Extraer psicologia
        if (data.psicologia) {
            if (data.psicologia.arquetipo) traits.push(`Arquetipo: ${data.psicologia.arquetipo}`);
        }

        // Extraer de cicatrices
        if (Array.isArray(data.scars)) {
            for (const scar of data.scars) {
                if (scar.name) traits.push(`Cicatriz: ${scar.name}`);
            }
        }

        return traits;
    }

    static decodeHtmlEntities(text) {
        if (typeof document === 'undefined') return text; // Fallback for non-browser envs
        const textarea = document.createElement('textarea');
        textarea.innerHTML = text;
        return textarea.value;
    }

    /**
     * Valida que un archivo sea una ficha válida antes de subirlo
     */
    static async validateFile(file) {
        // Verificar extensión
        if (!file.name.endsWith('.html') && !file.name.endsWith('.htm')) {
            return { valid: false, error: 'El archivo debe ser HTML' };
        }

        // Verificar tamaño (max 5MB)
        if (file.size > 5 * 1024 * 1024) {
            return { valid: false, error: 'El archivo es demasiado grande (máx. 5MB)' };
        }

        // Intentar parsear
        const result = await this.parseFromFile(file);

        if (!result.success) {
            return { valid: false, error: result.error };
        }

        return { valid: true };
    }
}
