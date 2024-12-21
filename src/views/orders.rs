use crate::controllers::orders::JsonOrderFeesToCreate;
use crate::views::partners::PartnerView;
use crate::views::sellers::SellerView;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeInOrdersReturn {
    pub fee_pid: Uuid,
    pub order_fee_pid: Option<Uuid>,
    pub fee: String,
    pub r#type: Option<String>,
    pub value: f32,
    pub info: Option<String>,
    pub open: bool,
}

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
    pub seller_pid: Uuid,
    pub process_pid: Uuid,
    pub open: bool,
    pub fee: f32,
    pub fees: Vec<JsonOrderFeesToCreate>,
    pub payout: Option<f32>,
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
    pub fees: Vec<FeeInOrdersReturn>,
    pub payout: Option<f32>,
    pub partner_fee: Option<f32>,
    pub seller: SellerView,
    pub client: ClientOrderReturn,
    pub process: ClientProcessReturn,
    pub payments: Vec<OrderPayments>,
}

impl GetOrderReturn {
    #[must_use]
    pub fn from(order: Self) -> Self {
        Self {
            pid: order.pid,
            client: order.client,
            payments: order
                .payments
                .into_iter()
                .map(OrderPayments::from)
                .collect(),
            process: order.process,
            seller: order.seller,
            open: order.open,
            fee: order.fee,
            fees: order.fees.into_iter().collect(),
            payout: order.payout,
            partner_fee: order.partner_fee,
        }
    }
}
