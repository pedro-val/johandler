use crate::models::_entities::{fees, processes, processes_fees};
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessFees {
    pub process_fee_pid: Uuid,
    pub fee_pid: Uuid,
    pub fee_name: String,
    pub r#type_or_info: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessView {
    pub pid: Uuid,
    pub case_type: String,
    pub fees: Vec<ProcessFees>,
}

impl ProcessView {
    #[must_use]
    pub fn from_model(
        model: Vec<processes::Model>,
        processes_fees: &[processes_fees::Model],
        fees: &[fees::Model],
    ) -> Vec<Self> {
        model
            .into_iter()
            .map(|m| Self::from(m, processes_fees, fees))
            .collect()
    }

    #[must_use]
    pub fn from(
        model: processes::Model,
        processes_fees: &[processes_fees::Model],
        fees: &[fees::Model],
    ) -> Self {
        let process_fees: Vec<ProcessFees> = processes_fees
            .iter()
            .filter(|pf| pf.process_id == model.id)
            .filter_map(|pf| {
                fees.iter()
                    .find(|fee| fee.id == pf.fee_id)
                    .map(|fee| ProcessFees {
                        process_fee_pid: pf.pid,
                        fee_pid: fee.pid,
                        fee_name: fee.fee.clone(),
                        r#type_or_info: fee.r#type.clone(),
                    })
            })
            .collect();

        Self {
            pid: model.pid,
            case_type: model.case_type,
            fees: process_fees,
        }
    }

    /// finds all processes
    ///
    /// # Errors
    ///
    /// When could not find processes or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Self>, sea_orm::DbErr> {
        let processes = processes::Entity::find().all(db).await?;
        let processes_fees = processes_fees::Entity::find().all(db).await?;
        let fees = fees::Entity::find().all(db).await?;

        let process_views: Vec<Self> = processes
            .into_iter()
            .map(|process| {
                let process_fees: Vec<ProcessFees> = processes_fees
                    .iter()
                    .filter(|pf| pf.process_id == process.id)
                    .filter_map(|pf| {
                        fees.iter()
                            .find(|fee| fee.id == pf.fee_id)
                            .map(|fee| ProcessFees {
                                process_fee_pid: pf.pid,
                                fee_pid: fee.pid,
                                fee_name: fee.fee.clone(),
                                r#type_or_info: fee.r#type.clone(),
                            })
                    })
                    .collect();

                Self {
                    pid: process.pid,
                    case_type: process.case_type,
                    fees: process_fees,
                }
            })
            .collect();

        Ok(process_views)
    }
}
