use super::_entities::postponed_payments::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type PostponedPayments = Entity;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::TransactionTrait;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewPostponedPayment {
    pub payment_id: i32,
    pub postponed_date: chrono::NaiveDate,
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

impl super::_entities::postponed_payments::Model {
    /// finds a postponed payment by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find postponed payment by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let postponed_payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::postponed_payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        postponed_payment.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all postponed payments
    ///
    /// # Errors
    ///
    /// When could not find postponed payments or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let postponed_payments = Entity::find().all(db).await?;
        Ok(postponed_payments)
    }

    /// creates a new postponed payment
    ///
    /// # Errors
    ///
    /// When could not create postponed payment or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        postponed_payment: CreateNewPostponedPayment,
    ) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _postponed_payment = ActiveModel {
            payment_id: ActiveValue::Set(postponed_payment.payment_id),
            postponed_date: ActiveValue::Set(postponed_payment.postponed_date),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// updates a postponed payment
    ///
    /// # Errors
    ///
    /// When could not update postponed payment or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        postponed_payment: CreateNewPostponedPayment,
    ) -> ModelResult<Vec<Self>> {
        let existing_postponed_payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::postponed_payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_postponed_payment = existing_postponed_payment.into_active_model();
        edited_postponed_payment.payment_id = ActiveValue::Set(postponed_payment.payment_id);
        edited_postponed_payment.postponed_date =
            ActiveValue::Set(postponed_payment.postponed_date);
        let txn = db.begin().await?;
        let _postponed_payment = edited_postponed_payment.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes a postponed payment
    ///
    /// # Errors
    ///
    /// When could not delete postponed payment or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<Vec<Self>> {
        let existing_postponed_payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::postponed_payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_postponed_payment.delete(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
