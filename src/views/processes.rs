use crate::models::_entities::processes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessView {
    pub pid: uuid::Uuid,
    pub case_type: String,
}

impl ProcessView {
    #[must_use]
    pub fn from_model(model: Vec<processes::Model>) -> Vec<Self> {
        model.into_iter().map(Self::from).collect()
    }

    #[must_use]
    pub fn from(model: processes::Model) -> Self {
        Self {
            pid: model.pid,
            case_type: model.case_type,
        }
    }
}
