import os
import requests
from flask import Flask, request, send_from_directory, jsonify
from flask_cors import CORS
import dashscope
from dashscope.audio.tts_v2 import SpeechSynthesizer

app = Flask(__name__)
CORS(app)

# CONFIG
DASHSCOPE_API_KEY = "sk-0a442750e50f49cfa39cd667cd601614" # Clave del usuario
OUTPUT_DIR = "tts_output"
if not os.path.exists(OUTPUT_DIR):
    os.makedirs(OUTPUT_DIR)

dashscope.api_key = DASHSCOPE_API_KEY

@app.route('/tts', methods=['POST'])
def tts():
    try:
        data = request.json
        text = data.get('text', '')
        if not text:
            return jsonify({"error": "No text provided"}), 400

        # Create filename based on text hash
        import hashlib
        filename = hashlib.md5(text.encode()).hexdigest() + ".mp3"
        filepath = os.path.join(OUTPUT_DIR, filename)

        if not os.path.exists(filepath):
            # Generate speech
            # For Spanish male: "mateo" or "enrique" or similar
            # Daithon: We want something calm and wise.
            synthesizer = SpeechSynthesizer(model="sambert-v1", voice="zh_female_xiaoyun") # Fallback
            
            # Note: DashScope voices vary by language. 
            # For Spanish (es), common ones are: 'es_female_isabel', 'es_male_mateo'
            voice_id = "es_male_mateo" 

            result = SpeechSynthesizer.call(model='sambert-v1',
                                            text=text,
                                            voice=voice_id,
                                            format='mp3')
            
            if result.get_audio_data():
                with open(filepath, 'wb') as f:
                    f.write(result.get_audio_data())
            else:
                return jsonify({"error": "Failed to generate audio", "details": str(result)}), 500

        return jsonify({
            "url": f"http://localhost:3456/audio/{filename}"
        })

    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route('/audio/<filename>')
def serve_audio(filename):
    return send_from_directory(OUTPUT_DIR, filename)

if __name__ == '__main__':
    print("--- DAITHON TTS SERVER RUNNING ---")
    print("Voice: Mateo (Spanish Male)")
    app.run(port=3456)
