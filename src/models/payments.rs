use super::_entities::payments::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Payments = Entity;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::ActiveValue;
use sea_orm::{IntoActiveModel, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewPayment {
    pub value: f32,
    pub payment_date: chrono::NaiveDate,
    pub due_date: chrono::NaiveDate,
    pub payment_method: Option<String>,
    pub currency: Option<String>,
    pub postponed_payment: Option<bool>,
    pub open: bool,
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

impl super::_entities::payments::Model {
    /// finds a payment by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find payment by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        payment.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all payments
    ///
    /// # Errors
    ///
    /// When could not find payments or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let payments = Entity::find().all(db).await?;
        Ok(payments)
    }

    /// creates a new payment
    ///
    /// # Errors
    ///
    /// When could not create payment or DB query error
    pub async fn create(db: &DatabaseConnection, payment: CreateNewPayment) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let payment = ActiveModel {
            value: ActiveValue::Set(payment.value),
            payment_date: ActiveValue::Set(payment.payment_date),
            due_date: ActiveValue::Set(payment.due_date),
            payment_method: ActiveValue::Set(payment.payment_method),
            currency: ActiveValue::Set(payment.currency),
            postponed_payment: ActiveValue::Set(payment.postponed_payment),
            open: ActiveValue::Set(payment.open),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(payment)
    }

    /// updates a payment
    ///
    /// # Errors
    ///
    /// When could not update payment or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        payment: CreateNewPayment,
    ) -> ModelResult<Self> {
        let existing_payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_payment = existing_payment.into_active_model();
        edited_payment.value = ActiveValue::Set(payment.value);
        edited_payment.payment_date = ActiveValue::Set(payment.payment_date);
        edited_payment.due_date = ActiveValue::Set(payment.due_date);
        edited_payment.payment_method = ActiveValue::Set(payment.payment_method);
        edited_payment.currency = ActiveValue::Set(payment.currency);
        edited_payment.postponed_payment = ActiveValue::Set(payment.postponed_payment);
        edited_payment.open = ActiveValue::Set(payment.open);
        let txn = db.begin().await?;
        let payment = edited_payment.update(&txn).await?;
        txn.commit().await?;
        Ok(payment)
    }

    /// deletes a payment
    ///
    /// # Errors
    ///
    /// When could not delete payment or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<()> {
        let existing_payment = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::payments::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_payment.delete(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}
