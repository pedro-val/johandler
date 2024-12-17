use crate::models::_entities::{orders, processes, users};
use crate::views::orders as OrdersView;
use crate::views::orders::{CreateNewOrder, OrderPayments};
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonOrderToCreate {
    pub process_pid: Uuid,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub payments: Vec<OrderPayments>,
}

#[debug_handler]
async fn create_order(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<JsonOrderToCreate>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let process = processes::Model::find_by_pid(&ctx.db, params.process_pid).await?;
    let create_new_order_params = CreateNewOrder {
        client_id: user.id,
        process_id: process.id,
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/orders")
        .add("/create", post(create_order))
}
