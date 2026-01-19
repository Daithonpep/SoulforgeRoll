# SoulForge - Motor de Historia
# Dockerfile para Railway (Rust + Actix-web)

# Usar una versión específica de Rust basada en Debian Bookworm para asegurar compatibilidad de GLIBC
FROM rust:1-bookworm as builder

# Dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar todo el código fuente
COPY . .

# Compilar en release
RUN cargo build --release

# === Imagen final ligera ===
FROM debian:bookworm-slim

# Instalar dependencias del sistema y Python para la integración IA
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado
COPY --from=builder /app/target/release/soulforge-server /app/soulforge-server

# Copiar integración de Groq
COPY --from=builder /app/groq_integration /app/groq_integration

# Crear entorno virtual e instalar dependencias de Python (para evitar conflictos con paquetes del sistema)
RUN python3 -m venv /app/venv
ENV PATH="/app/venv/bin:$PATH"
RUN pip install groq

# Puerto (Railway usa $PORT)
ENV PORT=8080

EXPOSE 8080

# Ejecutar servidor
CMD ["./soulforge-server"]
