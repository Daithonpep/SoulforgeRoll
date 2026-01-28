@echo off
title DAITHON TTS SERVER
echo [SOULFORGE] Levantando la voz de Daithon...
cd /d "%~dp0"
python tts_server.py
pause
