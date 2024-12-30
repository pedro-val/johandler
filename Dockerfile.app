# Use a imagem oficial do Rust como base
FROM rust:latest

# Defina o diretório de trabalho
WORKDIR /usr/src/app

# Copie os arquivos do projeto para o diretório de trabalho
COPY . .

# Defina a variável de ambiente para produção
ENV RUST_ENV=production

# Copie o certificado SSL para o contêiner
COPY config/prod-ca-2021.crt /usr/src/app/config/prod-ca-2021.crt

# Compile a aplicação
RUN cargo build --release

# Defina o comando de inicialização
CMD ["cargo", "run", "--release", "--bin", "johandler-cli", "start"]