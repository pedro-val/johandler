use crate::models::_entities::sellers;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SellerView {
    pub pid: uuid::Uuid,
    pub name: String,
}

impl SellerView {
    pub fn from_model(model: Vec<sellers::Model>) -> Vec<Self> {
        model.into_iter().map(|m| SellerView::from(m)).collect()
    }

    pub fn from(model: sellers::Model) -> Self {
        Self {
            pid: model.pid,
            name: model.name,
        }

    }
}
