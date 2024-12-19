use crate::models::_entities::{clients, orders, processes};
use crate::views::orders as OrdersView;
use crate::views::orders::{CreateNewOrder, OrderPayments};
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderPaymentsRequest {
    pub value: f32,
    pub payment_date: Option<chrono::NaiveDate>,
    pub due_date: chrono::NaiveDate,
    pub payment_method: Option<String>,
    pub currency: Option<String>,
    pub postponed_payment: Option<bool>,
    pub open: bool,
    pub postponed_dates: Option<Vec<chrono::NaiveDate>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonOrderToCreate {
    pub process_pid: Uuid,
    pub client_pid: Uuid,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub payments: Vec<OrderPaymentsRequest>,
}

#[debug_handler]
async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<JsonOrderToCreate>,
) -> Result<Response> {
    let create_new_order_params = JsonOrderToCreate {
        client_pid: params.client_pid,
        process_pid: params.process_pid,
        open: params.open,
        fee: params.fee,
        partner_fee: params.partner_fee,
        payments: params.payments,
    };
    let res = orders::Model::create(&ctx.db, &create_new_order_params).await;

    let order = match res {
        Ok(order) => order,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create order",);
            return format::json(());
        }
    };

    format::json(OrdersView::GetOrderReturn::from(order))
}

#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let orders = orders::Model::find_all(&ctx.db).await;

    let orders = match orders {
        Ok(orders) => orders,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not find orders",);
            return format::json(());
        }
    };

    format::json::<Vec<OrdersView::GetOrderReturn>>(
        orders
            .into_iter()
            .map(OrdersView::GetOrderReturn::from)
            .collect(),
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/orders")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
}
