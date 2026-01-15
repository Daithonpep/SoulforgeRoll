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
        const world = data.mundo || "Fantasía";

        // SISTEMA DE COMPOSICIÓN NARRATIVA PROFUNDA

        const origins = [
            `Nacido bajo el presagio de una tormenta antinatural, ${role.toLowerCase()} mostró desde joven una afinidad inquietante con fuerzas que no comprendía. Su infancia no fue fácil; marcado por la diferencia, aprendió a observar desde las sombras antes que a actuar bajo la luz del sol.`,
            `Proveniente de una estirpe olvidada, creció entre las ruinas de lo que alguna vez fue un gran imperio. Las historias de sus antepasados no fueron cuentos de cuna, sino advertencias grabadas en piedra y sangre. La disciplina férrea fue su única compañera.`,
            `La calle fue su verdadera madre y la necesidad su maestro cruel. Sin un apellido que lo protegiera, tuvo que forjar su propia identidad a golpe de ingenio y supervivencia en los callejones más oscuros de la ciudad, donde la moral es un lujo que pocos pueden permitirse.`
        ];

        const traumas = [
            `Pero el destino es caprichoso. Una traición inesperada de quien consideraba su mentor rompió su visión del mundo, dejándole una cicatriz invisible pero palpitante: la certeza de que la confianza es la más peligrosa de las debilidades.`,
            `Todo cambió la noche que el cielo se tiñó de rojo. Lo que presenció en aquel ritual prohibido fracturó su mente, dejando grietas por donde a veces se cuela una oscuridad que lucha por contener. Desde entonces, el silencio es su refugio y su condena.`,
            `La pérdida fue absoluta y devastadora. No fue solo la muerte de sus seres queridos, sino la forma en que el sistema corrupto encubrió la tragedia lo que encendió una llama fría de venganza en su pecho. Una llama que no calienta, solo consume.`
        ];

        const goals = [
            `Ahora, vaga buscando no solo redención, sino una verdad que pueda reescribir su historia. No busca ser un héroe, solo alguien capaz de inclinar la balanza cuando el mundo inevitablemente comience a arder de nuevo.`,
            `Su propósito actual es un misterio para muchos, pero en su interior arde una ambición clara: obtener el poder necesario para que nadie vuelva a tener autoridad sobre su destino. El fin justifica los medios, o eso se repite cada noche.`,
            `Camina por el sendero gris, ofreciendo sus talentos al mejor postor mientras reúne en secreto las piezas de un rompecabezas antiguo que podría salvar o condenar a todos. La soledad es el precio que ha aceptado pagar por este conocimiento.`
        ];

        // Mezclar para crear bio única
        const p1 = origins[Math.floor(Math.random() * origins.length)];
        const p2 = traumas[Math.floor(Math.random() * traumas.length)];
        const p3 = goals[Math.floor(Math.random() * goals.length)];

        return {
            // Bio completa compuesta
            origin: `${p1}\n\n${p2}\n\n${p3}`,
            turning_point: "La traición del mentor bajo la luna roja.", // Resumen para UI
            moment_of_light: "El recuerdo de una promesa hecha ante una tumba vacía."
        };
    }

    generateSoulPerks(data, stats) {
        // ... (Keep existing simple logic or expand similarly if needed)
        // For brevity keeping this part, but ensuring narrative is the focus
        const perks = [];
        let rolePerk = { name: "Golpe Básico", type: "COMBAT", flavor: "Un ataque directo y sin florituras." };
        const role = data.clase || data.rol || "Aventurero";

        if (role.match(/Guerrero|Barbaro/i)) rolePerk = { name: "Furia de Sangre", type: "COMBAT", flavor: "Canaliza el dolor para golpear con fuerza devastadora." };
        else if (role.match(/Mago|Hechicero/i)) rolePerk = { name: "Resonancia Arcana", type: "MAGIC", flavor: "El aire vibra a su alrededor, cargado de energía latente." };

        perks.push(rolePerk);
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
