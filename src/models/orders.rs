use super::_entities::orders;
use super::_entities::orders::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
pub type Orders = Entity;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::IntoActiveModel;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewOrder {
    pub client_id: i32,
    pub process_id: i32,
    pub open: bool,
    pub fee: i32,
    pub partner_fee: Option<i32>,
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

impl super::_entities::orders::Model {
    /// finds an order by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find order by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        order.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all orders
    ///
    /// # Errors
    ///
    /// When could not find orders or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let orders = Entity::find().all(db).await?;
        Ok(orders)
    }

    /// creates a new order
    ///
    /// # Errors
    ///
    /// When could not create order or DB query error
    pub async fn create(db: &DatabaseConnection, order: CreateNewOrder) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let order = orders::ActiveModel {
            client_id: ActiveValue::Set(order.client_id),
            process_id: ActiveValue::Set(order.process_id),
            open: ActiveValue::Set(order.open),
            fee: ActiveValue::Set(order.fee),
            partner_fee: ActiveValue::Set(order.partner_fee),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(order)
    }

    /// updates an order
    ///
    /// # Errors
    ///
    /// When could not update order or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        order: CreateNewOrder,
    ) -> ModelResult<Self> {
        let existing_order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_order = existing_order.into_active_model();
        edited_order.client_id = ActiveValue::Set(order.client_id);
        edited_order.process_id = ActiveValue::Set(order.process_id);
        edited_order.open = ActiveValue::Set(order.open);
        edited_order.fee = ActiveValue::Set(order.fee);
        edited_order.partner_fee = ActiveValue::Set(order.partner_fee);
        let txn = db.begin().await?;
        let order = edited_order.update(&txn).await?;
        txn.commit().await?;
        Ok(order)
    }

    /// deletes an order
    ///
    /// # Errors
    ///
    /// When could not delete order or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<()> {
        let existing_order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_order.delete(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}
