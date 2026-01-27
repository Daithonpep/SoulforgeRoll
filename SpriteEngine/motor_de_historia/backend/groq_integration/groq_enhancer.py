"""
Groq AI Integration for SoulForge v2.0
--------------------------------------
Genera contenido dinámico y narrativo basado en el mundo seleccionado.
- SIEMPRE genera parte de la biografía con contexto del mundo
- Enriquece secciones psicológicas de forma aleatoria
- Fallback al sistema normal si falla

API: Groq (llama-3.1-8b-instant)
"""

import os
import json
import random
from typing import Optional, Dict, Any, List

# ==================== CONFIGURATION ====================
# API key from environment variable or .env file
GROQ_API_KEY = os.environ.get("GROQ_API_KEY", "")

# Try to load from .env if not in environment
if not GROQ_API_KEY:
    try:
        env_path = os.path.join(os.path.dirname(__file__), ".env")
        if os.path.exists(env_path):
            with open(env_path) as f:
                for line in f:
                    if line.startswith("GROQ_API_KEY="):
                        GROQ_API_KEY = line.strip().split("=", 1)[1].strip('"').strip("'")
                        break
    except:
        pass

GROQ_MODEL = "llama-3.1-8b-instant"
GROQ_MAX_TOKENS = 300
GROQ_TEMPERATURE = 0.85

# ==================== WORLD CONTEXTS ====================
WORLD_CONTEXTS = {
    "MitologiaNordica": {
        "setting": "los nueve mundos de la mitología nórdica",
        "elements": ["runas antiguas", "el Valhalla", "las nornas del destino", "lobos de hielo", "el Ragnarök", "mjolnir", "cuervos de Odín", "hidromiel sagrado", "el Yggdrasil", "berserkers"],
        "tone": "épico y trágico, donde el destino es inevitable pero se enfrenta con honor",
        "names": ["Sigurd", "Freya", "Bjorn", "Astrid", "Ragnar", "Helga", "Ivar", "Thyra"]
    },
    "MitologiaGriega": {
        "setting": "la Grecia de los dioses del Olimpo y los héroes mortales",
        "elements": ["el oráculo de Delfos", "el río Estigia", "néctar divino", "laureles de victoria", "el laberinto", "la égida", "tritones", "sátiros", "el fuego de Prometeo"],
        "tone": "trágico y heroico, donde el hubris lleva a la caída pero la virtud trasciende",
        "names": ["Teseo", "Helena", "Ajax", "Casandra", "Perseus", "Ariadna", "Héctor", "Electra"]
    },
    "FantasiaMedieval": {
        "setting": "un reino de fantasía medieval con magia y profecías",
        "elements": ["torres de hechiceros", "espadas encantadas", "dragones antiguos", "gremios de aventureros", "códices prohibidos", "coronas malditas", "bosques encantados", "portales dimensionales"],
        "tone": "épico y esperanzador, donde el bien puede triunfar pero a un costo",
        "names": ["Aldric", "Elara", "Theron", "Lyra", "Cedric", "Morgana", "Gideon", "Seraphina"]
    },
    "FantasiaOscura": {
        "setting": "un mundo corrompido donde la luz es escasa y el horror acecha",
        "elements": ["iglesias abandonadas", "sangre de demonio", "pactos prohibidos", "maldiciones hereditarias", "elfos corrompidos", "reinos caídos", "plagas sobrenaturales", "susurros del abismo"],
        "tone": "sombrío y desesperanzado, donde sobrevivir ya es una victoria",
        "names": ["Malachar", "Lilith", "Graves", "Moira", "Ashford", "Vesper", "Thane", "Isolde"]
    },
    "SciFiCyberpunk": {
        "setting": "una megaciudad cyberpunk de neón y rain bajo el control de corporaciones",
        "elements": ["implantes neurales", "IAs rebeldes", "hackers de la red", "yakuza callejera", "drogas sintéticas", "androides conscientes", "mercenarios corporativos", "slums verticales"],
        "tone": "cínico y frenético, donde la humanidad se mide en líneas de código",
        "names": ["Zero", "Nova", "Axel", "Jinx", "Razor", "Cipher", "Vex", "Neon"]
    },
    "JaponFeudal": {
        "setting": "el Japón feudal de samuráis, ninjas y espíritus",
        "elements": ["katanas ancestrales", "códigos de honor bushido", "templos en la niebla", "yokai vengadores", "shogunes corruptos", "geishas espías", "ronin sin amo", "flores de cerezo ensangrentadas"],
        "tone": "melancólico y honorable, donde el deber pesa más que la vida",
        "names": ["Takeshi", "Akemi", "Kenji", "Sakura", "Hiroshi", "Yuki", "Masashi", "Hanako"]
    },
    "Noir": {
        "setting": "una ciudad noir de los años 40, llena de sombras y secretos",
        "elements": ["oficinas de detectives", "femme fatales", "whisky barato", "callejones lluviosos", "policías corruptos", "clubes de jazz", "revólveres ocultos", "cartas anónimas"],
        "tone": "cínico y romántico, donde todos tienen algo que ocultar",
        "names": ["Jack", "Vivian", "Vincent", "Gloria", "Max", "Dolores", "Frank", "Rita"]
    },
    "Steampunk": {
        "setting": "una era victoriana alternativa de vapor, engranajes y genios locos",
        "elements": ["autómatas de cuerda", "dirigibles de guerra", "sociedades secretas", "éter luminoso", "armas de rayo", "relojes imposibles", "expediciones al centro de la tierra", "damas inventoras"],
        "tone": "aventurero y maravillado, donde la ciencia es magia y todo es posible",
        "names": ["Archibald", "Victoria", "Sterling", "Beatrice", "Edison", "Cordelia", "Wellington", "Ada"]
    },
    "Western": {
        "setting": "el salvaje oeste americano de forajidos y fronteras",
        "elements": ["duelos al mediodía", "trenes del oro", "sheriffs solitarios", "bandidos legendarios", "pueblos fantasma", "minas malditas", "nativos espirituales", "caballos salvajes"],
        "tone": "polvoriento y justiciero, donde la ley está en la punta del revólver",
        "names": ["Wyatt", "Grace", "Cassidy", "Rosa", "Cole", "Belle", "Jesse", "Annie"]
    },
    "Anime": {
        "setting": "un mundo de anime con poderes extraordinarios y emociones intensas",
        "elements": ["academias de héroes", "torneos de poder", "lazos de amistad", "villanos carismáticos", "transformaciones épicas", "flashbacks traumáticos", "promesas inquebrantables", "sacrificios heroicos"],
        "tone": "dramático y esperanzador, donde el poder del corazón lo puede todo",
        "names": ["Yuki", "Sakura", "Ryu", "Hana", "Kai", "Mei", "Taro", "Luna"]
    }
}

# Default for unknown worlds
DEFAULT_CONTEXT = {
    "setting": "un mundo de fantasía épica",
    "elements": ["magia antigua", "profecías olvidadas", "reinos en guerra", "héroes improbables", "artefactos poderosos"],
    "tone": "épico y aventurero",
    "names": ["Aric", "Elena", "Marcus", "Lydia", "Vale", "Sera"]
}

# ==================== GROQ API ====================

def call_groq(prompt: str, system_prompt: str = None) -> Optional[str]:
    """Call Groq API with retry logic"""
    try:
        from groq import Groq
        
        client = Groq(api_key=GROQ_API_KEY)
        
        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})
        
        completion = client.chat.completions.create(
            model=GROQ_MODEL,
            messages=messages,
            max_tokens=GROQ_MAX_TOKENS,
            temperature=GROQ_TEMPERATURE
        )
        
        return completion.choices[0].message.content.strip()
        
    except ImportError:
        print("[GROQ] Library not installed")
        return None
    except Exception as e:
        print(f"[GROQ] Error: {e}")
        return None

# ==================== GENERIC CHAT ====================

def chat_with_aria(messages: List[Dict[str, str]], system_prompt: Optional[str] = None, max_tokens: int = 500) -> Optional[str]:
    """Generic chat interface for Aria/Oracle"""
    try:
        from groq import Groq
        client = Groq(api_key=GROQ_API_KEY)
        
        full_messages = []
        if system_prompt:
            full_messages.append({"role": "system", "content": system_prompt})
        full_messages.extend(messages)
        
        completion = client.chat.completions.create(
            model=GROQ_MODEL,
            messages=full_messages,
            max_tokens=max_tokens,
            temperature=GROQ_TEMPERATURE
        )
        return completion.choices[0].message.content.strip()
    except Exception as e:
        print(f"[GROQ CHAT] Error: {e}")
        return None

# ==================== CONTENT GENERATORS ====================

def get_world_context(world: str) -> Dict[str, Any]:
    """Get the context for a world, with fallback"""
    return WORLD_CONTEXTS.get(world, DEFAULT_CONTEXT)

def generate_biography_fragment(name: str, rol: str, world: str, genero: str = "unknown") -> Optional[str]:
    """Generate a narrative biography fragment based on world"""
    ctx = get_world_context(world)
    elements = random.sample(ctx["elements"], min(3, len(ctx["elements"])))
    
    system = f"""Eres un narrador de historias de {ctx['setting']}.
Tu tono es {ctx['tone']}.
Escribe en español, de forma evocadora y literaria.
NO uses clichés genéricos. Sé específico al mundo."""

    prompt = f"""Escribe un párrafo narrativo (4-5 oraciones) sobre un momento crucial en la vida de {name}, un {rol} {'masculino' if genero == 'Masculino' else 'femenino' if genero == 'Femenino' else ''} en {ctx['setting']}.

Incluye naturalmente algunos de estos elementos: {', '.join(elements)}.

El párrafo debe:
- Sentirse como parte de una novela
- Tener tensión dramática
- Revelar algo del carácter del personaje
- Ser específico a este mundo, NO genérico

Solo escribe el párrafo, sin introducción."""

    return call_groq(prompt, system)

def generate_narrative_hook(name: str, rol: str, world: str) -> Optional[str]:
    """Generate a narrative hook specific to the world"""
    ctx = get_world_context(world)
    element = random.choice(ctx["elements"])
    
    system = f"Eres un escritor de {ctx['setting']}. Tu tono es {ctx['tone']}."
    
    prompt = f"""En 2 oraciones, describe un misterio o conflicto pendiente para {name} ({rol}) relacionado con {element}.

Debe ser algo que deje al lector queriendo saber más. Sé específico al mundo de {ctx['setting']}.

Solo las 2 oraciones, sin introducción."""

    return call_groq(prompt, system)

def generate_wound_or_shadow(name: str, rol: str, world: str, section: str) -> Optional[str]:
    """Generate psychological depth based on world"""
    ctx = get_world_context(world)
    
    system = f"Eres un psicólogo narrativo especializado en personajes de {ctx['setting']}."
    
    if section == "herida":
        prompt = f"""En 2-3 oraciones, describe el trauma profundo de {name} ({rol}) en {ctx['setting']}.

¿Qué evento del pasado lo marcó? Hazlo específico al mundo y su tono {ctx['tone']}.

Solo el trauma, sin introducción."""
    else:  # sombra
        prompt = f"""En 2-3 oraciones, describe el lado oscuro oculto de {name} ({rol}) en {ctx['setting']}.

¿Qué rasgo niega de sí mismo? ¿Qué impulso reprime? Hazlo específico al mundo.

Solo la descripción, sin introducción."""

    return call_groq(prompt, system)

def generate_signature_quote(name: str, rol: str, world: str) -> Optional[str]:
    """Generate a character's signature quote"""
    ctx = get_world_context(world)
    
    prompt = f"""Inventa UNA frase corta y memorable que {name} ({rol}) diría en {ctx['setting']}.

La frase debe:
- Reflejar el tono {ctx['tone']}
- Sentirse natural para el personaje
- Ser única, no un cliché

Solo la frase entre comillas."""

    return call_groq(prompt)

# ==================== MAIN ENHANCER ====================

def enhance_character_with_world(character_data: Dict[str, Any]) -> Dict[str, Any]:
    """
    Enhance a character with AI-generated content based on their world.
    
    ALWAYS generates:
    - At least one biography fragment
    - A narrative hook
    
    RANDOMLY enhances (50% each):
    - The wound (herida)
    - The shadow (sombra)  
    - Signature quote
    """
    
    # Extract character info
    identidad = character_data.get("identidad", {})
    name = identidad.get("nombre", "el personaje")
    genero = identidad.get("genero", "")
    rol = character_data.get("rol", "Héroe")
    world = character_data.get("mundo", "FantasiaMedieval")
    
    enhanced = character_data.copy()
    enhanced["ai_enhancements"] = []
    
    print(f"[GROQ] Enhancing {name} ({rol}) in {world}...")
    
    # === ALWAYS: Biography Fragment ===
    bio_fragment = generate_biography_fragment(name, rol, world, genero)
    if bio_fragment:
        if "biografia" not in enhanced:
            enhanced["biografia"] = {"fases": []}
        if "fases" not in enhanced["biografia"]:
            enhanced["biografia"]["fases"] = []
        
        enhanced["biografia"]["fases"].append({
            "titulo": "Un Momento que lo Cambió Todo",
            "contenido": bio_fragment
        })
        enhanced["ai_enhancements"].append("biografia")
        print(f"[GROQ] ✓ Biography fragment added")
    
    # === ALWAYS: Narrative Hook ===
    hook = generate_narrative_hook(name, rol, world)
    if hook:
        if "ganchos_narrativos" not in enhanced:
            enhanced["ganchos_narrativos"] = []
        enhanced["ganchos_narrativos"].insert(0, hook)
        enhanced["ai_enhancements"].append("gancho")
        print(f"[GROQ] ✓ Narrative hook added")
    
    # === RANDOM (50%): Wound ===
    if random.random() < 0.5:
        wound = generate_wound_or_shadow(name, rol, world, "herida")
        if wound:
            if "capas" not in enhanced:
                enhanced["capas"] = {}
            if "herida" not in enhanced["capas"]:
                enhanced["capas"]["herida"] = {}
            enhanced["capas"]["herida"]["causante"] = wound
            enhanced["ai_enhancements"].append("herida")
            print(f"[GROQ] ✓ Wound enhanced")
    
    # === RANDOM (50%): Shadow ===
    if random.random() < 0.5:
        shadow = generate_wound_or_shadow(name, rol, world, "sombra")
        if shadow:
            if "capas" not in enhanced:
                enhanced["capas"] = {}
            if "sombra" not in enhanced["capas"]:
                enhanced["capas"]["sombra"] = {}
            enhanced["capas"]["sombra"]["rasgo_negado"] = shadow
            enhanced["ai_enhancements"].append("sombra")
            print(f"[GROQ] ✓ Shadow enhanced")
    
    # === RANDOM (40%): Quote ===
    if random.random() < 0.4:
        quote = generate_signature_quote(name, rol, world)
        if quote:
            if "capas" not in enhanced:
                enhanced["capas"] = {}
            if "mascara" not in enhanced["capas"]:
                enhanced["capas"]["mascara"] = {}
            enhanced["capas"]["mascara"]["frase_tipica"] = quote
            enhanced["ai_enhancements"].append("frase")
            print(f"[GROQ] ✓ Quote generated")
    
    print(f"[GROQ] Done! Enhancements: {enhanced['ai_enhancements']}")
    return enhanced

# ==================== FALLBACK WRAPPER ====================

def safe_enhance(character_data: Dict[str, Any]) -> Dict[str, Any]:
    """
    Wrapper that safely enhances or returns original on failure.
    """
    try:
        return enhance_character_with_world(character_data)
    except Exception as e:
        print(f"[GROQ] Enhancement failed, using fallback: {e}")
        return character_data

# ==================== TEST ====================

if __name__ == "__main__":
    # Test with Nordic mythology
    test_char = {
        "identidad": {
            "nombre": "Eirik Piedra de Sangre",
            "genero": "Masculino",
            "edad": 28
        },
        "rol": "Heroe",
        "mundo": "MitologiaNordica",
        "biografia": {"fases": []},
        "capas": {}
    }
    
    print("=" * 50)
    print("Testing Groq Enhancement - Nordic Mythology")
    print("=" * 50)
    
    result = safe_enhance(test_char)
    
    print("\n--- RESULT ---")
    if "ai_enhancements" in result:
        print(f"Enhancements made: {result['ai_enhancements']}")
        
        if result.get("biografia", {}).get("fases"):
            print(f"\nBiography:\n{result['biografia']['fases'][-1]['contenido']}")
        
        if result.get("ganchos_narrativos"):
            print(f"\nNarrative Hook:\n{result['ganchos_narrativos'][0]}")
        
        if result.get("capas", {}).get("herida", {}).get("causante"):
            print(f"\nWound:\n{result['capas']['herida']['causante']}")
    else:
        print("No enhancements (fallback used)")
