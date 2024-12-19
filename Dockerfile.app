FROM rust:latest

# Instale as dependências necessárias
RUN apt-get update && apt-get install -y libpq-dev

# Crie um diretório de trabalho
WORKDIR /usr/src/app

# Copie os arquivos do projeto para o diretório de trabalho
COPY . .

# Instale o cargo-watch
RUN cargo install cargo-watch

# Compile as dependências do projeto
RUN cargo build --release

# Comando para iniciar a aplicação
CMD ["cargo", "watch", "-x", "run --bin johandler-cli start"]