use super::_entities::fees::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Fees = Entity;
use crate::models::_entities::fees;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::TransactionTrait;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewFee {
    pub fee: String,
    pub r#type: Option<String>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
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

impl super::_entities::fees::Model {
    /// finds a fee by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find fee by the given pid or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// find fee by id
    ///
    /// # Errors
    ///
    /// When could not find fee by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all fees
    ///
    /// # Errors
    ///
    /// When could not find fees or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let fees = Entity::find().all(db).await?;
        Ok(fees)
    }

    /// creates a new fee
    ///
    /// # Errors
    ///
    /// When could not create fee or DB query error
    pub async fn create(db: &DatabaseConnection, fee: CreateNewFee) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _fee = fees::ActiveModel {
            fee: ActiveValue::Set(fee.fee),
            r#type: ActiveValue::Set(fee.r#type),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// updates a fee
    ///
    /// # Errors
    ///
    /// When could not update fee or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: Uuid,
        fee: CreateNewFee,
    ) -> ModelResult<Vec<Self>> {
        let existing_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_fee = existing_fee.into_active_model();
        edited_fee.fee = ActiveValue::Set(fee.fee);
        edited_fee.r#type = ActiveValue::Set(fee.r#type);
        let txn = db.begin().await?;
        let _fee = edited_fee.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes a fee
    ///
    /// # Errors
    ///
    /// When could not delete fee or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        let existing_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_fee.delete(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
