use crate::models::_entities::fees;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeView {
    pub pid: uuid::Uuid,
    pub fee: String,
    pub r#type: Option<String>,
}

impl FeeView {
    pub fn from_model(model: Vec<fees::Model>) -> Vec<Self> {
        model.into_iter().map(|m| FeeView::from(m)).collect()
    }

    pub fn from(model: fees::Model) -> Self {
        Self {
            pid: model.pid,
            fee: model.fee,
            r#type: model.r#type,
        }
    }
}
