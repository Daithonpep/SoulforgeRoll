# Groq AI Integration for SoulForge

## Overview
This module enhances character generation with AI-powered narrative content using the Groq API.

## Features
- **World-Specific Content**: Generates narrative elements tailored to the selected world (Norse, Greek, Cyberpunk, etc.)
- **Biography Enhancement**: Adds deeper, more evocative biography sections
- **Narrative Hooks**: Creates plot threads specific to the world setting
- **Psychological Depth**: Enriches wounds and shadows with world-specific trauma
- **Signature Quotes**: Generates memorable character quotes

## Setup

1. Copy `.env.example` to `.env`
2. Add your Groq API key to `.env`:
   ```
   GROQ_API_KEY=your_api_key_here
   ```
3. Install the Groq library: `pip install groq`

## API Limits (Free Tier)
- Requests per minute: 30
- Tokens per minute: 14,400
- Requests per day: ~2,000

## Usage

```python
from groq_enhancer import safe_enhance

# Your character data
character = {
    "identidad": {"nombre": "Sigurd", "genero": "Masculino"},
    "rol": "Heroe",
    "mundo": "MitologiaNordica",
    # ... rest of character
}

# Enhance with AI
enhanced = safe_enhance(character)
```

## Fallback Behavior
If the Groq API fails for any reason, `safe_enhance()` returns the original character data unchanged. The system is designed to work with or without AI enhancement.

## Supported Worlds
- MitologiaNordica (Norse Mythology)
- MitologiaGriega (Greek Mythology)
- FantasiaMedieval (Medieval Fantasy)
- FantasiaOscura (Dark Fantasy)
- SciFiCyberpunk (Cyberpunk)
- JaponFeudal (Feudal Japan)
- Noir
- Steampunk
- Western
- Anime
