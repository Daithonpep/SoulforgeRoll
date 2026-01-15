export class SoulRPGEngine {
    constructor() {
        this.statIcons = {
            STR: '⚔️', DEX: '🏹', CON: '🛡️',
            INT: '🔮', WIS: '👁️', CHA: '🎭'
        };
    }

    process(characterData, formData) {
        // 1. Basic Stat Generation (Standard 10-18 range)
        let stats = {
            STR: 10 + Math.floor(Math.random() * 8),
            DEX: 10 + Math.floor(Math.random() * 8),
            CON: 10 + Math.floor(Math.random() * 8),
            INT: 10 + Math.floor(Math.random() * 8),
            WIS: 10 + Math.floor(Math.random() * 8),
            CHA: 10 + Math.floor(Math.random() * 8)
        };

        const age = parseInt(formData.get('edad_fija')) || 25;
        const moralTone = formData.get('tono_moral') || 'Gris';
        const userClass = formData.get('clase');
        const userRole = formData.get('rol');
        const userRaza = formData.get('raza');

        // Override
        if (userClass) characterData.clase = userClass;
        if (userRole) characterData.rol = userRole;
        if (userRaza) characterData.raza = userRaza;
        characterData.edad = age;
        characterData.tono = moralTone;

        // 2. Age Modifiers
        let ageTraits = [];
        if (age < 25) {
            stats.STR += 2; stats.DEX += 2; stats.CON += 2;
            stats.WIS -= 2;
            ageTraits.push("Vitalidad Juvenil", "Impetuoso");
        } else if (age > 50) {
            stats.STR -= 2; stats.DEX -= 2; stats.CON -= 2;
            stats.WIS += 3; stats.INT += 1;
            ageTraits.push("Sabiduría Ancestral", "Cuerpo Frágil");
            if (Math.random() > 0.5) characterData.inventory = ["Artefacto Heredado: Reloj de Arena de Hueso"];
        }

        // 3. Soul Tension
        let soulTension = 0;
        if (moralTone === 'Oscuro') {
            soulTension = 60 + Math.floor(Math.random() * 30);
            characterData.shadow_bonus = "Furia de Sombra (+1d6 Dmg cuando Tensión > 80)";
        } else if (moralTone === 'Luminoso') {
            soulTension = 10 + Math.floor(Math.random() * 20);
            characterData.purity_bonus = "Sintonía Fina (Ventaja en Perspicacia)";
        } else {
            soulTension = 30 + Math.floor(Math.random() * 40);
        }

        // 4. Moral Matrix
        const moralProgress = Math.floor(Math.random() * 100);

        // SAFE ACCESS to Psicologia
        const psych = characterData.psicologia || {};
        const mentira = psych.mentira || "Una mentira oculta que aún no descubre.";
        const verdad = psych.verdad || "La verdad que debe aceptar para crecer.";

        // 5. Perks
        const perks = this.generateSoulPerks(characterData, stats);

        // 6. Narrative
        const narrative = this.generateNarrativeFlow(characterData, age, moralTone);

        // 7. Timeline
        const timeline = this.generateTimeline(age, moralTone);

        // 8. Economy
        const gear = this.generateEquipment(characterData.clase || characterData.rol, age, moralTone);

        // 9. Tags
        const tags = this.generateTags(characterData);

        return {
            ...characterData,
            isRPG: true,
            rpg_stats: stats,
            mechanics: {
                soul_tension: soulTension,
                age_traits: ageTraits,
                moral_matrix: {
                    current_stage: "Mentira",
                    conflict: mentira,
                    resolution: verdad,
                    progress: moralProgress
                },
                soul_perks: perks
            },
            narrative_arc: narrative,
            timeline: timeline,
            economy: gear.economy,
            equipment: gear.equipment,
            inventory: gear.inventory,
            meta_tags: tags
        };
    }

    generateNarrativeFlow(data, age, tone) {
        const role = data.clase || data.rol || "Aventurero";
        let origins = [
            "Creció escuchando historias de héroes caídos.",
            "Su infancia estuvo marcada por disciplina férrea.",
            "Nacido en la pobreza, aprendió a sobrevivir."
        ];
        if (role.match(/Mago|Hechicero/i)) origins = ["La magia se manifestó temprano como un peligro.", "Estudió en torres prohibidas."];
        else if (role.match(/Picaro|Crimen/i)) origins = ["Las calles fueron su escuela.", "Huyó de su familia noble."];

        let turningPoints = [
            "Una luna roja cambió su destino.",
            "Una traición le enseñó a no confiar.",
            "Encontró un artefacto que susurraba su nombre."
        ];

        let lights = [
            "Conserva una flauta de su padre.",
            "Recuerda una sonrisa salvadora.",
            "Tiene un jardín mental secreto."
        ];

        const pick = (arr) => arr[Math.floor(Math.random() * arr.length)];

        return {
            origin: pick(origins),
            turning_point: pick(turningPoints),
            moment_of_light: pick(lights)
        };
    }

    generateSoulPerks(data, stats) {
        const perks = [];
        let rolePerk = { name: "Golpe Básico", type: "COMBAT" };
        const role = data.clase || data.rol || "Aventurero";

        if (role.match(/Guerrero|Barbaro/i)) rolePerk = { name: "Furia", type: "COMBAT" };
        else if (role.match(/Mago|Hechicero/i)) rolePerk = { name: "Eco Arcano", type: "MAGIC" };

        perks.push(rolePerk);

        let statPerk = { name: "Voluntad", type: "PASIVA" };
        if (stats.STR >= 15) statPerk = { name: "Rompehuesos", type: "COMBAT" };
        else if (stats.INT >= 15) statPerk = { name: "Mente Maestra", type: "PASIVA" };

        perks.push(statPerk);
        return perks;
    }

    generateTimeline(age, tone) {
        return [
            { year: "Año 0", event: "Nacimiento.", scar: null },
            { year: "Actualidad", event: "Busca propósito.", scar: null }
        ];
    }

    generateEquipment(role, age, tone) {
        return {
            economy: { currency: Math.floor(Math.random() * 500), status: "Estable" },
            equipment: ["Ropa de Viajero", "Arma Simple"],
            inventory: ["Raciones", "Odre de Agua"]
        };
    }

    generateTags(data) {
        const tags = ["npc_compatible"];
        if (data.clase) tags.push(data.clase.toLowerCase());
        return tags;
    }
}
