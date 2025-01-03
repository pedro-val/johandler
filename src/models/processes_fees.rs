use super::_entities::processes_fees::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type ProcessesFees = Entity;
use super::_entities::{fees, processes, processes_fees};
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::ActiveValue;
use sea_orm::{IntoActiveModel, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewProcessFee {
    pub process_pid: Uuid,
    pub fee_pid: Uuid,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let mut this = self;
            this.pid = ActiveValue::Set(Uuid::new_v4());
            return Ok(this);
        }
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            return Ok(this);
        }
        Ok(self)
    }
}

impl super::_entities::processes_fees::Model {
    /// finds a process fee by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find process fee by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let process_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        process_fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a process fee by the provided id
    ///
    /// # Errors
    ///
    /// When could not find process fee by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let process_fee = Entity::find_by_id(id).one(db).await?;
        process_fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all process fees
    ///
    /// # Errors
    ///
    /// When could not find process fees or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let process_fees = Entity::find().all(db).await?;
        Ok(process_fees)
    }

    /// creates a new process fee
    ///
    /// # Errors
    ///
    /// When could not create process fee or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        process_fee: CreateNewProcessFee,
    ) -> ModelResult<Vec<Self>> {
        let process = processes::Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        super::_entities::processes::Column::Pid,
                        process_fee.process_pid,
                    )
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let fee = fees::Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Pid, process_fee.fee_pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        let _process_fee = processes_fees::ActiveModel {
            process_id: ActiveValue::Set(process.id),
            fee_id: ActiveValue::Set(fee.id),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }

    /// updates a process fee
    ///
    /// # Errors
    ///
    /// When could not update process fee or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: Uuid,
        process_fee: CreateNewProcessFee,
    ) -> ModelResult<Vec<Self>> {
        let process = processes::Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        super::_entities::processes::Column::Pid,
                        process_fee.process_pid,
                    )
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let fee = fees::Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Pid, process_fee.fee_pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let existing_process_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_process_fee = existing_process_fee.into_active_model();
        edited_process_fee.process_id = ActiveValue::Set(process.id);
        edited_process_fee.fee_id = ActiveValue::Set(fee.id);
        let txn = db.begin().await?;
        let _process_fee = edited_process_fee.update(&txn).await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }

    /// deletes a process fee
    ///
    /// # Errors
    ///
    /// When could not delete process fee or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        let process_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        process_fee.delete(&txn).await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }
}
