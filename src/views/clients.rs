use crate::models::_entities::{clients, orders, partners, processes, sellers};
use crate::views::partners::PartnerView;
use crate::views::sellers::SellerView;
use loco_rs::model::ModelResult;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProcessView {
    pub pid: Uuid,
    pub case_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientOrdersView {
    pub pid: Uuid,
    pub process: OrderProcessView,
    pub open: bool,
    pub fee: f32,
    pub seller: SellerView,
    pub partner_fee: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientViewResponse {
    pub pid: Uuid,
    pub name: String,
    pub contact: String,
    pub phone: Option<String>,
    pub phone2: Option<String>,
    pub email: Option<String>,
    pub partner: Option<PartnerView>,
    pub orders: Vec<ClientOrdersView>,
}

impl ClientViewResponse {
    /// Creates a `ClientViewResponse` from a `clients::Model`.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the database queries fail.
    pub async fn from_model(db: &DatabaseConnection, client: clients::Model) -> ModelResult<Self> {
        let orders = orders::Model::find_by_client_id(db, client.id).await?;

        let mut client_orders = Vec::new();

        for order in orders {
            let process = processes::Model::find_by_id(db, order.process_id).await?;

            client_orders.push(ClientOrdersView {
                pid: order.pid,
                process: OrderProcessView {
                    pid: process.pid,
                    case_type: process.case_type,
                },
                seller: SellerView::from(sellers::Model::find_by_id(db, order.seller_id).await?),
                open: order.open,
                fee: order.fee,
                partner_fee: order.partner_fee,
            });
        }

        let partner = match client.partner_id {
            Some(id) => {
                let partner = partners::Model::find_by_id(db, id).await?;
                Some(PartnerView::from(partner))
            }
            None => None,
        };

        Ok(Self {
            pid: client.pid,
            name: client.name,
            contact: client.contact,
            phone: Some(client.phone),
            phone2: client.phone2,
            email: Some(client.email),
            partner: partner,
            orders: client_orders,
        })
    }
}