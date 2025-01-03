use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::sqlx;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::app::sqlx::PgPool;

use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};

pub struct App;

#[derive(Deserialize, Serialize)]
struct Process {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    case_type: String,
}

#[derive(Deserialize, Serialize)]
struct Partner {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    name: String,
    information: String,
    phone: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct Seller {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Client {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    name: String,
    contact: String,
    phone: String,
    phone2: Option<String>,
    email: String,
    partner_id: Option<i32>,
}

#[derive(Deserialize, Serialize)]
struct Fee {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    fee: String,
    r#type: String,
}

#[derive(Deserialize, Serialize)]
struct OrderFee {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    fee_id: i32,
    order_id: i32,
    open: bool,
    value: i32,
    info: String,
}

#[derive(Deserialize, Serialize)]
struct Order {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    client_id: i32,
    process_id: i32,
    open: bool,
    payout: i32,
    fee: i32,
    partner_fee: Option<i32>,
    seller_id: i32,
}

#[derive(Deserialize, Serialize)]
struct Payment {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    value: i32,
    payment_date: Option<String>,
    due_date: String,
    payment_method: String,
    currency: String,
    postponed_payment: bool,
    order_id: i32,
    open: bool,
}

#[derive(Deserialize, Serialize)]
struct PostponedPayment {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    value: i32,
    payment_date: Option<String>,
    due_date: String,
    payment_method: String,
    currency: String,
    postponed_payment: bool,
    order_id: i32,
    open: bool,
}

#[derive(Deserialize, Serialize)]
struct ProcessFee {
    created_at: String,
    updated_at: String,
    id: i32,
    pid: String,
    fee_id: i32,
    process_id: i32,
    open: bool,
    value: i32,
    info: String,
}

impl App {
    async fn import_data<T: for<'de> Deserialize<'de> + Serialize + Send + Sync>(
        pool: &PgPool,
        table: &str,
        file: &str,
    ) -> Result<(), sqlx::Error> {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let records: Vec<T> = serde_json::from_str(&data).expect("Unable to parse JSON");

        for record in records {
            let query_str = match table {
                "processes" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, case_type) VALUES ($1, $2, $3, $4, $5)",
                    table
                ),
                "partners" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, name, information, phone, email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    table
                ),
                "sellers" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, name) VALUES ($1, $2, $3, $4, $5)",
                    table
                ),
                "clients" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, name, contact, phone, phone2, email, partner_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    table
                ),
                "fees" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, fee, type) VALUES ($1, $2, $3, $4, $5, $6)",
                    table
                ),
                "order_fees" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, fee_id, order_id, open, value, info) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    table
                ),
                "orders" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, client_id, process_id, open, payout, fee, partner_fee, seller_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                    table
                ),
                "payments" => format!(
                    "INSERT INTO {} (created_at, updated_at, id, pid, value, payment_date, due_date, payment_method, currency, postponed_payment, order_id, open) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                    table
                ),
                _ => panic!("Unknown table: {}", table),
            };

            let query = match table {
                "processes" => {
                    let record: Process = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.case_type.clone())
                },
                "partners" => {
                    let record: Partner = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.name.clone())
                        .bind(record.information.clone())
                        .bind(record.phone.clone())
                        .bind(record.email.clone())
                },
                "sellers" => {
                    let record: Seller = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.name.clone())
                },
                "clients" => {
                    let record: Client = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.name.clone())
                        .bind(record.contact.clone())
                        .bind(record.phone.clone())
                        .bind(record.phone2.clone())
                        .bind(record.email.clone())
                        .bind(record.partner_id)
                },
                "fees" => {
                    let record: Fee = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.fee.clone())
                        .bind(record.r#type.clone())
                },
                "order_fees" => {
                    let record: OrderFee = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.fee_id)
                        .bind(record.order_id)
                        .bind(record.open)
                        .bind(record.value)
                        .bind(record.info.clone())
                },
                "orders" => {
                    let record: Order = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.client_id)
                        .bind(record.process_id)
                        .bind(record.open)
                        .bind(record.payout)
                        .bind(record.fee)
                        .bind(record.partner_fee)
                        .bind(record.seller_id)
                },
                "payments" => {
                    let record: Payment = serde_json::from_value(serde_json::to_value(record).unwrap()).unwrap();
                    sqlx::query(&query_str)
                        .bind(record.created_at.clone())
                        .bind(record.updated_at.clone())
                        .bind(record.id)
                        .bind(record.pid.clone())
                        .bind(record.value)
                        .bind(record.payment_date.clone())
                        .bind(record.due_date.clone())
                        .bind(record.payment_method.clone())
                        .bind(record.currency.clone())
                        .bind(record.postponed_payment)
                        .bind(record.order_id)
                        .bind(record.open)
                },
                _ => panic!("Unknown table: {}", table),
            };

            query.execute(pool).await?;
        }

        Ok(())
    }

    async fn import_all_data(pool: &PgPool) -> Result<(), sqlx::Error> {
        App::import_data::<Process>(pool, "processes", "database_backup/backup_processes.json").await?;
        App::import_data::<Partner>(pool, "partners", "database_backup/backup_partners.json").await?;
        App::import_data::<Seller>(pool, "sellers", "database_backup/backup_sellers.json").await?;
        App::import_data::<Client>(pool, "clients", "database_backup/backup_clients.json").await?;
        App::import_data::<Order>(pool, "orders", "database_backup/backup_orders.json").await?;
        App::import_data::<Payment>(pool, "payments", "database_backup/backup_payments.json").await?;
        App::import_data::<Fee>(pool, "fees", "database_backup/backup_fees.json").await?;
        App::import_data::<OrderFee>(pool, "order_fees", "database_backup/backup_order_fees.json").await?;
        println!("Dados importados com sucesso!");

        Ok(())
    }
}

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        let app = create_app::<Self, Migrator>(mode, environment).await?;
        
        // Conectar ao banco de dados e importar os dados
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await?;
        App::import_all_data(&pool).await?;

        Ok(app)
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::auth::routes())
            .add_route(controllers::orders::routes())
            .add_route(controllers::sellers::routes())
            .add_route(controllers::clients::routes())
            .add_route(controllers::partners::routes())
            .add_route(controllers::processes::routes())
            .add_route(controllers::fees::routes())
            .add_route(controllers::processes_fees::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
        // tasks-inject (do not remove)
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}