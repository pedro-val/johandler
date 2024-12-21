use crate::models::_entities::fees;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeView {
    pub pid: uuid::Uuid,
    pub fee: String,
    pub r#type: Option<String>,
}

impl FeeView {
    #[must_use]
    pub fn from_model(model: Vec<fees::Model>) -> Vec<Self> {
        model.into_iter().map(Self::from).collect()
    }

    #[must_use]
    pub fn from(model: fees::Model) -> Self {
        Self {
            pid: model.pid,
            fee: model.fee,
            r#type: model.r#type,
        }
    }
}
