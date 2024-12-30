#!/bin/bash

# Definir o diretório HOME explicitamente
export HOME=/root

echo "Iniciando o script de build..."

# Instalar o Docker
echo "Instalando o Docker..."
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Construir a imagem Docker
echo "Construindo a imagem Docker..."
docker build -t johandler-app .

# Rodar o container Docker
echo "Rodando o container Docker..."
docker run -d -p 8080:8080 johandler-app

echo "Script de build concluído com sucesso."