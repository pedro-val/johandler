use crate::models::_entities::{clients, partners, processes, sellers};
use crate::views::partners::PartnerView;
use crate::views::sellers::SellerView;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderPayments {
    pub pid: Option<Uuid>,
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
    pub client_pid: Uuid,
    pub process_pid: Uuid,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub payments: Vec<OrderPayments>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientOrderReturn {
    pub pid: Uuid,
    pub name: String,
    pub contact: String,
    pub phone: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
    pub seller: SellerView,
    pub partner: Option<PartnerView>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientProcessReturn {
    pub pid: Uuid,
    pub case_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOrderReturn {
    pub pid: Uuid,
    pub open: bool,
    pub fee: f32,
    pub partner_fee: Option<f32>,
    pub client: ClientOrderReturn,
    pub process: ClientProcessReturn,
    pub payments: Vec<OrderPayments>,
}

impl GetOrderReturn {
    #[must_use]
    pub fn from(order: GetOrderReturn) -> Self {
        Self {
            pid: order.pid,
            client: order.client,
            payments: order
                .payments
                .into_iter()
                .map(|p| OrderPayments::from(p))
                .collect(),
            process: order.process,
            open: order.open,
            fee: order.fee,
            partner_fee: order.partner_fee,
        }
    }
}
