use super::_entities::orders::{ActiveModel, Entity};
use super::_entities::{
    clients, fees, order_fees, orders, partners, payments, postponed_payments, processes, sellers,
};
use crate::views::orders::FeeInOrdersReturn;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
pub type Orders = Entity;
use crate::controllers::orders::JsonOrderToCreate;
use crate::views::orders::{
    ClientOrderReturn, ClientProcessReturn, CreateNewOrder, GetOrderReturn, OrderPayments,
};
use crate::views::partners::PartnerView;
use crate::views::sellers::SellerView;
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
        let client = clients::Model::find_by_id(db, order.client_id).await?;
        let seller = sellers::Model::find_by_id(db, order.seller_id).await?;
        let partner = match client.partner_id {
            Some(id) => {
                let partner = partners::Model::find_by_id(db, id).await?;
                Some(PartnerView::from(partner))
            }
            None => None,
        };
        let process = processes::Model::find_by_id(db, order.process_id).await?;
        let order_fees = order_fees::Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::order_fees::Column::OrderId, order.id)
                    .build(),
            )
            .all(db)
            .await?;
        let mut fees = vec![];
        for order_fee in order_fees {
            let fee = fees::Model::find_by_id(db, order_fee.fee_id).await?;
            fees.push({
                FeeInOrdersReturn {
                    fee_pid: fee.pid,
                    order_fee_pid: Some(order_fee.pid),
                    fee: fee.fee,
                    r#type: fee.r#type,
                    value: order_fee.value,
                    open: order_fee.open,
                    info: order_fee.info,
                }
            });
        }
        Ok(GetOrderReturn {
            pid: order.pid,
            client: ClientOrderReturn {
                pid: client.pid,
                name: client.name,
                contact: client.contact,
                phone: Some(client.phone),
                phone2: client.phone2,
                email: Some(client.email),
                partner: partner,
            },
            process: {
                ClientProcessReturn {
                    pid: process.pid,
                    case_type: process.case_type,
                }
            },
            open: order.open,
            fee: order.fee,
            fees: fees,
            payout: Some(order.payout),
            seller: SellerView::from(seller),
            partner_fee: order.partner_fee,
            payments: payments
                .into_iter()
                .map(|payment| OrderPayments {
                    pid: Some(payment.pid),
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
            let client_to_find = clients::Model::find_by_id(db, order.client_id).await?;
            let seller = sellers::Model::find_by_id(db, order.seller_id).await?;
            let partner = match client_to_find.partner_id {
                Some(id) => {
                    let partner = partners::Model::find_by_id(db, id).await?;
                    Some(PartnerView::from(partner))
                }
                None => None,
            };
            let process = processes::Model::find_by_id(db, order.process_id).await?;
            let order_fees = order_fees::Entity::find()
                .filter(
                    model::query::condition()
                        .eq(super::_entities::order_fees::Column::OrderId, order.id)
                        .build(),
                )
                .all(db)
                .await?;
            let mut fees = vec![];
            for order_fee in order_fees {
                let fee = fees::Model::find_by_id(db, order_fee.fee_id).await?;
                fees.push({
                    FeeInOrdersReturn {
                        fee_pid: fee.pid,
                        order_fee_pid: Some(order_fee.pid),
                        fee: fee.fee,
                        r#type: fee.r#type,
                        value: order_fee.value,
                        open: order_fee.open,
                        info: order_fee.info,
                    }
                });
            }
            orders_return.push(GetOrderReturn {
                pid: order.pid,
                client: ClientOrderReturn {
                    pid: client_to_find.pid,
                    name: client_to_find.name,
                    contact: client_to_find.contact,
                    phone: Some(client_to_find.phone),
                    phone2: client_to_find.phone2,
                    email: Some(client_to_find.email),
                    partner: partner,
                },
                seller: SellerView::from(seller),
                process: {
                    ClientProcessReturn {
                        pid: process.pid,
                        case_type: process.case_type,
                    }
                },
                open: order.open,
                fee: order.fee,
                fees: fees,
                payout: Some(order.payout),
                partner_fee: order.partner_fee,
                payments: payments
                    .into_iter()
                    .map(|payment| OrderPayments {
                        pid: Some(payment.pid),
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
        order: &JsonOrderToCreate,
    ) -> ModelResult<GetOrderReturn> {
        // Verifique se todas as entidades referenciadas existem
        let client = clients::Model::find_by_pid(db, order.client_pid).await?;

        let process = processes::Model::find_by_pid(db, order.process_pid).await?;

        let seller = sellers::Model::find_by_pid(db, order.seller_pid).await?;

        // Inicie uma transação
        let txn = db.begin().await?;

        // Crie a nova ordem
        let to_create_order = orders::ActiveModel {
            client_id: ActiveValue::Set(client.id),
            process_id: ActiveValue::Set(process.id),
            seller_id: ActiveValue::Set(seller.id),
            open: ActiveValue::Set(order.open),
            fee: ActiveValue::Set(order.fee),
            payout: ActiveValue::Set(order.payout.unwrap_or_default()),
            partner_fee: ActiveValue::Set(order.partner_fee),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;

        // Encontre a ordem criada
        let created_order = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::orders::Column::Pid, to_create_order.pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        // Crie as taxas associadas à ordem
        if !order.fees.is_empty() {
            for order_fee in &order.fees {
                let fee = fees::Model::find_by_pid(db, order_fee.fee_pid).await?;
                let txn = db.begin().await?;
                let _to_create_order_fee = order_fees::ActiveModel {
                    fee_id: ActiveValue::Set(fee.id),
                    order_id: ActiveValue::Set(created_order.id),
                    open: ActiveValue::Set(order_fee.open),
                    value: ActiveValue::Set(order_fee.value),
                    info: ActiveValue::Set(order_fee.info.clone()),
                    ..Default::default()
                }
                .insert(&txn)
                .await?;
                txn.commit().await?;
            }
        }

        // Crie os pagamentos associados à ordem
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
            let mut postponed_payments = vec![];
            if let Some(dates) = &payment.postponed_dates {
                for date in dates {
                    let txn = db.begin().await?;
                    let _to_create_postponed_payment = postponed_payments::ActiveModel {
                        payment_id: ActiveValue::Set(to_create_payment.id),
                        postponed_date: ActiveValue::Set(*date),
                        ..Default::default()
                    }
                    .insert(&txn)
                    .await?;
                    txn.commit().await?;
                    postponed_payments.push(*date);
                }
            }
            order_payments.push(to_create_payment);
        }

        // Encontre o cliente, vendedor e parceiro associados à ordem
        let client_to_find = clients::Model::find_by_pid(db, order.client_pid).await?;

        let seller = sellers::Model::find_by_pid(db, order.seller_pid).await?;

        let partner = match client_to_find.partner_id {
            Some(id) => {
                let partner = partners::Model::find_by_id(db, id).await?;
                Some(PartnerView::from(partner))
            }
            None => None,
        };

        let process = processes::Model::find_by_pid(db, order.process_pid).await?;

        // Encontre as taxas associadas à ordem
        let order_fees = order_fees::Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        super::_entities::order_fees::Column::OrderId,
                        created_order.id,
                    )
                    .build(),
            )
            .all(db)
            .await?;
        let mut fees = vec![];
        for order_fee in order_fees {
            let fee = fees::Model::find_by_id(db, order_fee.fee_id).await?;
            fees.push({
                FeeInOrdersReturn {
                    fee_pid: fee.pid,
                    order_fee_pid: Some(order_fee.pid),
                    fee: fee.fee,
                    r#type: fee.r#type,
                    value: order_fee.value,
                    info: order_fee.info,
                    open: order_fee.open,
                }
            });
        }

        // Retorne a ordem criada
        Ok(GetOrderReturn {
            pid: created_order.pid,
            client: ClientOrderReturn {
                pid: client_to_find.pid,
                name: client_to_find.name,
                contact: client_to_find.contact,
                phone: Some(client_to_find.phone),
                phone2: client_to_find.phone2,
                email: Some(client_to_find.email),
                partner: partner,
            },
            seller: SellerView::from(seller),
            process: {
                ClientProcessReturn {
                    pid: process.pid,
                    case_type: process.case_type,
                }
            },
            open: created_order.open,
            fee: created_order.fee,
            fees: fees,
            payout: Some(created_order.payout),
            partner_fee: created_order.partner_fee,
            payments: order_payments
                .into_iter()
                .map(|payment| OrderPayments {
                    pid: Some(payment.pid),
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
        let mut edited_order = existing_order.clone().into_active_model();
        let client = clients::Model::find_by_pid(db, order.client_pid).await?;
        let process = processes::Model::find_by_pid(db, order.process_pid).await?;
        edited_order.client_id = ActiveValue::Set(client.id);
        edited_order.process_id = ActiveValue::Set(process.id);
        edited_order.open = ActiveValue::Set(order.open);
        edited_order.fee = ActiveValue::Set(order.fee);
        edited_order.payout = ActiveValue::Set(order.payout.unwrap_or_default());
        edited_order.partner_fee = ActiveValue::Set(order.partner_fee);
        let txn = db.begin().await?;
        edited_order.update(&txn).await?;
        txn.commit().await?;

        let existing_payments = payments::Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        super::_entities::payments::Column::OrderId,
                        existing_order.id,
                    )
                    .build(),
            )
            .all(db)
            .await?;
        for order_fee in order.fees {
            if order_fee.order_fee_pid.is_none() {
                let fee = fees::Model::find_by_pid(db, order_fee.fee_pid).await?;
                let txn = db.begin().await?;
                let _to_create_order_fee = order_fees::ActiveModel {
                    fee_id: ActiveValue::Set(fee.id),
                    order_id: ActiveValue::Set(existing_order.id),
                    open: ActiveValue::Set(order_fee.open),
                    value: ActiveValue::Set(order_fee.value),
                    info: ActiveValue::Set(order_fee.info.clone()),
                    ..Default::default()
                }
                .insert(&txn)
                .await?;
                txn.commit().await?;
            } else {
                let order_fee = order_fees::Entity::find()
                    .filter(
                        model::query::condition()
                            .eq(
                                super::_entities::order_fees::Column::Pid,
                                order_fee.order_fee_pid,
                            )
                            .build(),
                    )
                    .one(db)
                    .await?
                    .ok_or_else(|| ModelError::EntityNotFound)?;
                let mut edited_order_fee = order_fee.clone().into_active_model();
                edited_order_fee.open = ActiveValue::Set(order_fee.open);
                edited_order_fee.value = ActiveValue::Set(order_fee.value);
                edited_order_fee.info = ActiveValue::Set(order_fee.info.clone());
                let txn = db.begin().await?;
                edited_order_fee.update(&txn).await?;
                txn.commit().await?;
            }
        }

        if existing_payments.len() == order.payments.len() {
            // Ação quando os comprimentos são iguais
            for (existing_payment, new_payment) in
                existing_payments.iter().zip(order.payments.iter())
            {
                let payment = payments::Entity::find()
                    .filter(
                        model::query::condition()
                            .eq(
                                super::_entities::payments::Column::Pid,
                                existing_payment.pid,
                            )
                            .build(),
                    )
                    .one(db)
                    .await?
                    .ok_or_else(|| ModelError::EntityNotFound)?;
                if new_payment.postponed_dates.is_some() {
                    for postponed_date in new_payment.postponed_dates.as_ref().unwrap() {
                        let txn = db.begin().await?;
                        let _to_create_postponed_payment = postponed_payments::ActiveModel {
                            payment_id: ActiveValue::Set(payment.id),
                            postponed_date: ActiveValue::Set(*postponed_date),
                            ..Default::default()
                        }
                        .insert(&txn)
                        .await?;
                        txn.commit().await?;
                    }
                }
                let mut edited_payment = payment.clone().into_active_model();
                edited_payment.value = ActiveValue::Set(new_payment.value);
                edited_payment.payment_date = ActiveValue::Set(new_payment.payment_date);
                edited_payment.due_date = ActiveValue::Set(new_payment.due_date);
                edited_payment.payment_method =
                    ActiveValue::Set(new_payment.payment_method.clone());
                edited_payment.currency = ActiveValue::Set(new_payment.currency.clone());
                edited_payment.postponed_payment =
                    ActiveValue::Set(Some(new_payment.postponed_dates.is_some()));
                edited_payment.open = ActiveValue::Set(new_payment.open);
                let txn = db.begin().await?;
                edited_payment.update(&txn).await?;
                txn.commit().await?;
            }
        } else {
            for new_payment in &order.payments {
                if new_payment.pid.is_none() {
                    let txn = db.begin().await?;
                    let to_create_payment = payments::ActiveModel {
                        value: ActiveValue::Set(new_payment.value),
                        payment_date: ActiveValue::Set(new_payment.payment_date),
                        due_date: ActiveValue::Set(new_payment.due_date),
                        payment_method: ActiveValue::Set(new_payment.payment_method.clone()),
                        currency: ActiveValue::Set(new_payment.currency.clone()),
                        postponed_payment: ActiveValue::Set(new_payment.postponed_payment),
                        order_id: ActiveValue::Set(existing_order.id),
                        open: ActiveValue::Set(new_payment.open),
                        ..Default::default()
                    }
                    .insert(&txn)
                    .await?;
                    txn.commit().await?;
                    if new_payment.postponed_dates.is_some() {
                        for date in new_payment.postponed_dates.as_ref().unwrap() {
                            let txn = db.begin().await?;
                            let _to_create_postponed_payment = postponed_payments::ActiveModel {
                                payment_id: ActiveValue::Set(to_create_payment.id),
                                postponed_date: ActiveValue::Set(*date),
                                ..Default::default()
                            }
                            .insert(&txn)
                            .await?;
                            txn.commit().await?;
                        }
                    }
                }
            }
        }

        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes an order
    ///
    /// # Errors
    ///
    /// When could not delete order or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<GetOrderReturn>> {
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
