# SoulForge - Motor de Historia
# Dockerfile para Railway (Rust + Actix-web)

# Usar la última versión estable de Rust
FROM rust:latest as builder

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

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado
COPY --from=builder /app/target/release/soulforge /app/soulforge

# Templates y static removidos para Backend Only
# COPY --from=builder /app/templates ./templates
# COPY --from=builder /app/static ./static

# Puerto (Railway usa $PORT)
ENV PORT=8080
EXPOSE 8080

# Ejecutar servidor
CMD ["./soulforge"]
