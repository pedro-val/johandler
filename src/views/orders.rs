// use crate::models::_entities::{orders, payments, postponed_payments};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderPayments {
    pub pid: Uuid,
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
pub struct CreateNewOrder {
    pub client_id: i32,
    pub process_id: i32,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub payments: Vec<OrderPayments>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOrderReturn {
    pub pid: Uuid,
    pub client_id: i32,
    pub process_id: i32,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub payments: Vec<OrderPayments>,
}

impl GetOrderReturn {
    #[must_use]
    pub fn from(order: GetOrderReturn) -> Self {
        Self {
            pid: order.pid,
            client_id: order.client_id,
            payments: order
                .payments
                .into_iter()
                .map(|p| OrderPayments::from(p))
                .collect(),
            process_id: order.process_id,
            open: order.open,
            fee: order.fee,
            partner_fee: order.partner_fee,
        }
    }
}
