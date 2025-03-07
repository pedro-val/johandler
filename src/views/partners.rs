use crate::models::_entities::partners;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PartnerView {
    pub pid: uuid::Uuid,
    pub name: String,
    pub information: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl PartnerView {
    #[must_use]
    pub fn from_model(model: Vec<partners::Model>) -> Vec<Self> {
        model.into_iter().map(Self::from).collect()
    }

    #[must_use]
    pub fn from(model: partners::Model) -> Self {
        Self {
            pid: model.pid,
            name: model.name,
            information: model.information,
            phone: model.phone,
            email: model.email,
        }
    }
}
