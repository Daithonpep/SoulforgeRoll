
        // --- SOUL RPG ENGINE (V4.5 Logic) ---
        class SoulRPGEngine {
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

                const age = parseInt(formData.get('edad')) || 25;
                const moralTone = formData.get('tono') || 'gritty';
                
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
                    // Chance for Heirloom
                    if (Math.random() > 0.5) characterData.inventory = ["Artefacto Heredado: Reloj de Arena de Hueso"];
                }

                // 3. Soul Tension & Moral Matrix
                let soulTension = 0; // 0-100
                if (moralTone === 'dark' || moralTone === 'lovecraft') {
                    soulTension = 60 + Math.floor(Math.random() * 30); // High tension start
                    characterData.shadow_bonus = "Furia de Sombra (+1d6 Dmg cuando Tensión > 80)";
                } else {
                    soulTension = 10 + Math.floor(Math.random() * 20); // Enlightened start
                    characterData.purity_bonus = "Sintonía Fina (Ventaja en Perspicacia)";
                }

                // 4. Procedural Life History (Timeline)
                const timeline = this.generateTimeline(age, moralTone);

                // 5. Meta-Tagging based on Text Analysis (Simulated)
                const tags = this.generateTags(characterData);

                // Construct Enhanced RPG JSON
                return {
                    ...characterData,
                    isRPG: true,
                    rpg_stats: stats,
                    mechanics: {
                        soul_tension: soulTension,
                        age_traits: ageTraits,
                        moral_matrix: {
                            current_stage: "Mentira",
                            conflict: characterData.psicologia.mentira,
                            resolution: characterData.psicologia.verdad,
                            progress: 10 // 0 to 100
                        }
                    },
                    timeline: timeline,
                    meta_tags: tags
                };
            }

            generateTimeline(age, tone) {
                const events = [];
                // Generate a simpler timeline for now
                const origin = { year: "Origen", event: "Nacimiento bajo una estrella auspiciosa.", scar: null };
                events.push(origin);
                
                if (age > 15) events.push({ year: "Juventud", event: "Primera pérdida significativa.", scar: "Cicatriz emocional: Desconfianza" });
                if (age > 30) events.push({ year: "Adultez", event: "Gran conflicto armado o social.", scar: "Cicatriz física: Cojera leve" });
                
                return events;
            }

            generateTags(data) {
                const tags = ["npc_compatible"];
                if (data.rol && (data.rol.includes("Paladin") || data.rol.includes("Clerigo"))) tags.push("divine_caster");
                return tags;
            }
        }

        const rpgEngine = new SoulRPGEngine();

        function showRPGCharacterHTML(p) {
            // RPG SHEET DESIGN GENERATOR
            const jsonString = encodeURIComponent(JSON.stringify(p, null, 2));
            const tensionColor = p.mechanics.soul_tension > 50 ? '#ff4444' : '#ffd700';
            
            // Build Stats HTML
            let statsHTML = '';
            for (let [key, val] of Object.entries(p.rpg_stats)) {
                statsHTML += `
                    <div class="stat-box">
                        <div class="stat-icon">${rpgEngine.statIcons[key]}</div>
                        <div class="stat-val">${val}</div>
                        <div class="stat-label">${key}</div>
                    </div>
                `;
            }

            // Build Timeline HTML
            let timelineHTML = '';
            p.timeline.forEach(t => {
                timelineHTML += `
                    <div class="timeline-event">
                        <div class="event-year">${t.year}</div>
                        <div class="event-desc">${t.event}</div>
                        ${t.scar ? `<div class="event-scar">🤕 ${t.scar}</div>` : ''}
                    </div>
                `;
            });

            const htmlContent = `
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <title>${p.nombre} - RPG Sheet</title>
    <link href="https://fonts.googleapis.com/css2?family=Cinzel:wght@700&family=Crimson+Text:ital,wght@0,400;0,700;1,400&display=swap" rel="stylesheet">
    <style>
        :root { --bg: #1a1a20; --paper: #e3dac9; --ink: #2b2b2b; --gold: #c5a059; --blood: #8a0b0b; }
        body { background: var(--bg); font-family: 'Crimson Text', serif; margin:0; padding: 20px; display:flex; justify-content:center; }
        
        .sheet-container {
            width: 1000px;
            background: var(--paper);
            color: var(--ink);
            box-shadow: 0 0 50px rgba(0,0,0,0.5);
            display: grid;
            grid-template-columns: 300px 1fr;
            min-height: 100vh;
            position: relative;
        }

        /* SIDE PANEL (Interactive) */
        .side-panel {
            background: #111;
            color: #ccc;
            padding: 20px;
            border-right: 5px solid var(--gold);
            display: flex;
            flex-direction: column;
            gap: 20px;
        }

        .avatar-frame {
            width: 200px; height: 200px;
            border: 3px solid var(--gold);
            border-radius: 50%;
            margin: 0 auto;
            background: #222;
            display: flex; align-items: center; justify-content: center;
            font-size: 4rem;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
        }
        .stat-box {
            background: #222; border: 1px solid #444;
            padding: 10px; text-align: center; border-radius: 5px;
        }
        .stat-val { font-family: 'Cinzel', serif; font-size: 1.5rem; color: var(--gold); }
        .stat-label { font-size: 0.8rem; text-transform: uppercase; color: #888; }
        .stat-icon { font-size: 1.2rem; margin-bottom: 5px; }

        /* MAIN CONTENT */
        .main-content { padding: 40px; position: relative; }
        
        /* SOUL TENSION METER */
        .tension-container {
            position: absolute; top: 0; left: 0; width: 100%; height: 10px; background: #333;
        }
        .tension-bar {
            height: 100%; background: ${tensionColor}; width: ${p.mechanics.soul_tension}%;
            transition: width 1s;
        }
        .tension-label {
            position: absolute; top: 15px; right: 20px; font-family: 'Cinzel'; font-size: 0.8rem; color: var(--blood);
        }

        h1 { font-family: 'Cinzel'; font-size: 3rem; margin: 0; color: var(--ink); border-bottom: 2px solid var(--gold); padding-bottom: 10px; }
        .meta-line { font-style: italic; color: #555; margin-bottom: 30px; font-size: 1.1rem; }

        .section-title {
            font-family: 'Cinzel'; font-size: 1.5rem; color: var(--blood); margin-top: 30px; margin-bottom: 15px; border-bottom: 1px solid #ccc;
        }

        .tag { background: #d4c5a9; padding: 2px 8px; border-radius: 4px; font-size: 0.9rem; border: 1px solid #bba; display: inline-block; margin: 2px; }

        .timeline-event {
            border-left: 2px solid var(--gold); padding-left: 15px; margin-bottom: 15px;
        }
        .event-year { font-weight: bold; font-family: 'Cinzel'; color: #444; }
        .event-scar { color: var(--blood); font-weight: bold; font-size: 0.9rem; margin-top: 5px; }

        .moral-matrix {
            background: rgba(0,0,0,0.05); padding: 20px; border-radius: 8px; border: 1px dashed #aaa;
        }

        .btn-download {
            position: absolute; bottom: 20px; right: 20px;
            background: var(--blood); color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px; font-family: 'Cinzel';
        }
    </style>
</head>
<body>
    <div class="sheet-container">
        <!-- SIDE PANEL -->
        <div class="side-panel">
            <div class="avatar-frame">👤</div>
            <div class="stats-grid">
                ${statsHTML}
            </div>
            <div style="margin-top:20px;">
                <div style="font-size:0.8rem; color:#888; margin-bottom:5px;">BEHAVIORAL TAGS</div>
                ${p.meta_tags.map(t => `<span style="color:var(--gold); border:1px solid #444; padding:2px 6px; font-size:0.7rem; margin:2px; display:inline-block; border-radius:3px;">${t}</span>`).join('')}
            </div>
        </div>

        <!-- MAIN CONTENT -->
        <div class="main-content">
            <div class="tension-container">
                <div class="tension-bar"></div>
            </div>
            <div class="tension-label">SOUL TENSION: ${p.mechanics.soul_tension}%</div>

            <h1>${p.nombre}</h1>
            <div class="meta-line">${p.raza} ${p.rol} · ${p.edad} Años · ${p.tono}</div>

            <p>${p.resumen}</p>

            <div class="section-title">🧠 Psico-Matriz Evolutiva</div>
            <div class="moral-matrix">
                <p><strong>La Mentira (Estado Inicial):</strong> ${p.mechanics.moral_matrix.conflict}</p>
                <div style="text-align:center; margin:10px; font-size:1.5rem;">⬇️</div>
                <p><strong>La Verdad (Objetivo):</strong> ${p.mechanics.moral_matrix.resolution}</p>
                <div style="margin-top:10px; background:#ccc; height:5px; border-radius:2px; overflow:hidden;">
                    <div style="width:${p.mechanics.moral_matrix.progress}%; background:var(--gold); height:100%;"></div>
                </div>
                <p style="text-align:right; font-size:0.8rem;">Progreso de Redención: ${p.mechanics.moral_matrix.progress}%</p>
            </div>

            <div class="section-title">📜 Línea de Vida & Cicatrices</div>
            ${timelineHTML}

            <a href="#" onclick='downloadJSON()' class="btn-download">💾 Download RPG JSON</a>
        </div>
    </div>

    <script>
        function downloadJSON() {
             const dataStr = "data:text/json;charset=utf-8," + decodeURIComponent("${jsonString}");
             const downloadAnchorNode = document.createElement('a');
             downloadAnchorNode.setAttribute("href", dataStr);
             downloadAnchorNode.setAttribute("download", "${p.nombre}_RPG_Sheet.json");
             document.body.appendChild(downloadAnchorNode);
             downloadAnchorNode.click();
             downloadAnchorNode.remove();
        }
    </script>
</body>
</html>
            `;

            const newWindow = window.open();
            newWindow.document.write(htmlContent);
            newWindow.document.close();
        }
