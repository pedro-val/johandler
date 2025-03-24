FROM rust:latest as builder

WORKDIR /usr/src/

COPY . .

# Limpar o cache de compilação do Cargo
RUN cargo clean

# Atualizar as dependências do Cargo
RUN cargo update

# Compilar o projeto em modo release
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/johandler-cli /usr/app/johandler-cli

# Tornar o binário executável
RUN chmod +x /usr/app/johandler-cli

ENTRYPOINT ["/usr/app/johandler-cli", "start", "-e", "production", "-b", "0.0.0.0"]