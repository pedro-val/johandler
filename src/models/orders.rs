use super::_entities::orders::{ActiveModel, Entity};
use super::_entities::{orders, payments, postponed_payments};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
pub type Orders = Entity;
use crate::views::orders::{CreateNewOrder, GetOrderReturn, OrderPayments};
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::IntoActiveModel;
use sea_orm::TransactionTrait;

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
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<GetOrderReturn> {
        let order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        let postponed_payments = postponed_payments::Entity::find().all(db).await?;
        let order = order.ok_or_else(|| ModelError::EntityNotFound)?;
        let payments = payments::Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::payments::Column::OrderId, order.id)
                    .build(),
            )
            .all(db)
            .await?;
        Ok(GetOrderReturn {
            pid: order.pid,
            client_id: order.client_id,
            process_id: order.process_id,
            open: order.open,
            fee: order.fee,
            partner_fee: order.partner_fee,
            payments: payments
                .into_iter()
                .map(|payment| OrderPayments {
                    pid: payment.pid,
                    value: payment.value,
                    payment_date: payment.payment_date,
                    due_date: payment.due_date,
                    payment_method: payment.payment_method,
                    currency: payment.currency,
                    postponed_payment: payment.postponed_payment,
                    open: payment.open,
                    postponed_dates: postponed_payments
                        .iter()
                        .filter(|postponed_payment| postponed_payment.payment_id == payment.id)
                        .map(|postponed_payment| postponed_payment.postponed_date)
                        .collect::<Vec<_>>()
                        .into(),
                })
                .collect(),
        })
    }

    /// finds orders by the provided client_id
    ///
    /// # Errors
    ///
    /// When could not find orders by the given client_id or DB query error
    pub async fn find_by_client_id(
        db: &DatabaseConnection,
        client_id: i32,
    ) -> ModelResult<Vec<Self>> {
        let orders = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::ClientId, client_id)
                    .build(),
            )
            .all(db)
            .await?;
        Ok(orders)
    }

    /// finds all orders
    ///
    /// # Errors
    ///
    /// When could not find orders or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<GetOrderReturn>> {
        let orders = Entity::find().all(db).await?;
        let postponed_payments = postponed_payments::Entity::find().all(db).await?;
        let mut orders_return = vec![];
        for order in orders {
            let payments = payments::Entity::find()
                .filter(
                    model::query::condition()
                        .eq(super::_entities::payments::Column::OrderId, order.id)
                        .build(),
                )
                .all(db)
                .await?;
            orders_return.push(GetOrderReturn {
                pid: order.pid,
                client_id: order.client_id,
                process_id: order.process_id,
                open: order.open,
                fee: order.fee,
                partner_fee: order.partner_fee,
                payments: payments
                    .into_iter()
                    .map(|payment| OrderPayments {
                        pid: payment.pid,
                        value: payment.value,
                        payment_date: payment.payment_date,
                        due_date: payment.due_date,
                        payment_method: payment.payment_method,
                        currency: payment.currency,
                        postponed_payment: payment.postponed_payment,
                        open: payment.open,
                        postponed_dates: postponed_payments
                            .iter()
                            .filter(|postponed_payment| postponed_payment.payment_id == payment.id)
                            .map(|postponed_payment| postponed_payment.postponed_date)
                            .collect::<Vec<_>>()
                            .into(),
                    })
                    .collect(),
            });
        }
        Ok(orders_return)
    }

    /// creates a new order
    ///
    /// # Errors
    ///
    /// When could not create order or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        order: &CreateNewOrder,
    ) -> ModelResult<GetOrderReturn> {
        let txn = db.begin().await?;
        let to_create_order = orders::ActiveModel {
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
        let created_order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, to_create_order.pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut order_payments = vec![];
        for payment in &order.payments {
            let txn = db.begin().await?;
            let to_create_payment = payments::ActiveModel {
                value: ActiveValue::Set(payment.value),
                payment_date: ActiveValue::Set(payment.payment_date),
                due_date: ActiveValue::Set(payment.due_date),
                payment_method: ActiveValue::Set(payment.payment_method.clone()),
                currency: ActiveValue::Set(payment.currency.clone()),
                postponed_payment: ActiveValue::Set(payment.postponed_payment),
                order_id: ActiveValue::Set(created_order.id),
                open: ActiveValue::Set(payment.open),
                ..Default::default()
            }
            .insert(&txn)
            .await?;
            txn.commit().await?;
            order_payments.push(to_create_payment);
        }
        Ok(GetOrderReturn {
            pid: created_order.pid,
            client_id: created_order.client_id,
            process_id: created_order.process_id,
            open: created_order.open,
            fee: created_order.fee,
            partner_fee: created_order.partner_fee,
            payments: order_payments
                .into_iter()
                .map(|payment| OrderPayments {
                    pid: payment.pid,
                    value: payment.value,
                    payment_date: payment.payment_date,
                    due_date: payment.due_date,
                    payment_method: payment.payment_method,
                    currency: payment.currency,
                    postponed_payment: payment.postponed_payment,
                    open: payment.open,
                    postponed_dates: None,
                })
                .collect(),
        })
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
    ) -> ModelResult<Vec<GetOrderReturn>> {
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
        edited_order.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes an order
    ///
    /// # Errors
    ///
    /// When could not delete order or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<Vec<GetOrderReturn>> {
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
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
