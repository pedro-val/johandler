use super::_entities::order_fees::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type OrderFees = Entity;
use crate::models::_entities::order_fees;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::TransactionTrait;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewOrderFee {
    pub fee_id: i32,
    pub order_id: i32,
    pub open: bool,
    pub value: f32,
    pub info: Option<String>,
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

impl super::_entities::order_fees::Model {
    /// finds an order fee by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find order fee by the given pid or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let order_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::order_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        order_fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// find order fee by id
    ///
    /// # Errors
    ///
    /// When could not find order fee by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let order_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::order_fees::Column::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        order_fee.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all order fees
    ///
    /// # Errors
    ///
    /// When could not find order fees or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let order_fees = Entity::find().all(db).await?;
        Ok(order_fees)
    }

    /// creates a new order fee
    ///
    /// # Errors
    ///
    /// When could not create order fee or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        order_fee: CreateNewOrderFee,
    ) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _order_fee = order_fees::ActiveModel {
            fee_id: ActiveValue::Set(order_fee.fee_id),
            order_id: ActiveValue::Set(order_fee.order_id),
            open: ActiveValue::Set(order_fee.open),
            value: ActiveValue::Set(order_fee.value),
            info: ActiveValue::Set(order_fee.info),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// updates an order fee
    ///
    /// # Errors
    ///
    /// When could not update order fee or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: Uuid,
        order_fee: CreateNewOrderFee,
    ) -> ModelResult<Vec<Self>> {
        let existing_order_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::order_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_order_fee = existing_order_fee.into_active_model();
        edited_order_fee.fee_id = ActiveValue::Set(order_fee.fee_id);
        edited_order_fee.order_id = ActiveValue::Set(order_fee.order_id);
        edited_order_fee.open = ActiveValue::Set(order_fee.open);
        edited_order_fee.value = ActiveValue::Set(order_fee.value);
        edited_order_fee.info = ActiveValue::Set(order_fee.info);
        let txn = db.begin().await?;
        let _order_fee = edited_order_fee.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes an order fee
    ///
    /// # Errors
    ///
    /// When could not delete order fee or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        let existing_order_fee = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::order_fees::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_order_fee.delete(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
