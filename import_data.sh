#!/bin/bash

# Definir a URI de conexão do banco de dados
DATABASE_URL="postgres://johandler:gpYDmXB0gLsDSZS@johandler-db.flycast:5432/johandler?sslmode=disable"

# Função para importar dados de um arquivo JSON para uma tabela
import_data() {
  local table=$1
  local file=$2
  psql $DATABASE_URL -c "\copy $table FROM '$file' WITH (FORMAT json)"
}

# Importar dados de cada arquivo JSON na ordem das migrations
import_data "processes" "database_backup/backup_processes.json"
import_data "partners" "database_backup/backup_partners.json"
import_data "sellers" "database_backup/backup_sellers.json"
import_data "clients" "database_backup/backup_clients.json"
import_data "fees" "database_backup/backup_fees.json"
import_data "order_fees" "database_backup/backup_order_fees.json"
import_data "orders" "database_backup/backup_orders.json"
import_data "payments" "database_backup/backup_payments.json"
import_data "postponed_payments" "database_backup/backup_postponed_payments.json"

echo "Dados importados com sucesso!"