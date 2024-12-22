# Use a imagem oficial do Rust como base
FROM rust:latest

# Defina o diretório de trabalho
WORKDIR /usr/src/app

# Copie os arquivos do projeto para o diretório de trabalho
COPY . .

# Compile a aplicação
RUN cargo build --release

# Defina o comando de inicialização
CMD ["cargo", "run", "--release", "--bin", "johandler-cli", "start"]