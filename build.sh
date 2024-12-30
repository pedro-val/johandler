#!/bin/bash

# Instalar as dependências
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Instalar dependências do projeto
cargo build --release

# Mover o binário compilado para o diretório de saída
mkdir -p public
cp target/release/johandler-cli public/

# Copiar o certificado SSL para o diretório de saída
cp config/prod-ca-2021.crt public/