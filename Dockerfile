FROM rust:latest as builder

WORKDIR /usr/src/

# Copie o Cargo.toml, Cargo.lock e o diretório migration para fixar as dependências
COPY Cargo.toml Cargo.lock migration/ ./

# Baixe as dependências sem compilar o código
RUN cargo fetch

# Copie o restante do código
COPY . .

# Limpar o cache de compilação do Cargo
RUN cargo clean

# Compilar o projeto em modo release
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/johandler-cli /usr/app/johandler-cli

# Tornar o binário executável
RUN chmod +x /usr/app/johandler-cli

ENTRYPOINT ["/usr/app/johandler-cli", "start", "-e", "production", "-b", "0.0.0.0"]