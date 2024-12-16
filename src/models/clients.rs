use super::_entities::clients::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Clients = Entity;
use crate::models::_entities::clients;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::TransactionTrait;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewClient {
    pub name: String,
    pub contact: String,
    pub phone: String,
    pub phone2: Option<String>,
    pub email: String,
    pub seller_id: i32,
    pub partner_id: Option<i32>,
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

impl super::_entities::clients::Model {
    /// finds a client by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find client by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let client = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::clients::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        client.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all clients
    ///
    /// # Errors
    ///
    /// When could not find clients or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let clients = Entity::find().all(db).await?;
        Ok(clients)
    }

    /// creates a new client
    ///
    /// # Errors
    ///
    /// When could not create client or DB query error
    pub async fn create(db: &DatabaseConnection, client: CreateNewClient) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let client = clients::ActiveModel {
            name: ActiveValue::Set(client.name),
            contact: ActiveValue::Set(client.contact),
            phone: ActiveValue::Set(client.phone),
            phone2: ActiveValue::Set(client.phone2),
            email: ActiveValue::Set(client.email),
            seller_id: ActiveValue::Set(client.seller_id),
            partner_id: ActiveValue::Set(client.partner_id),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(client)
    }

    /// updates a client
    ///
    /// # Errors
    ///
    /// When could not update client or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        client: CreateNewClient,
    ) -> ModelResult<Self> {
        let existing_client = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::clients::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_client = existing_client.into_active_model();
        edited_client.name = ActiveValue::Set(client.name);
        edited_client.contact = ActiveValue::Set(client.contact);
        edited_client.phone = ActiveValue::Set(client.phone);
        edited_client.phone2 = ActiveValue::Set(client.phone2);
        edited_client.email = ActiveValue::Set(client.email);
        edited_client.seller_id = ActiveValue::Set(client.seller_id);
        edited_client.partner_id = ActiveValue::Set(client.partner_id);
        let txn = db.begin().await?;
        let client = edited_client.update(&txn).await?;
        txn.commit().await?;
        Ok(client)
    }

    /// deletes a client
    ///
    /// # Errors
    ///
    /// When could not delete client or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<()> {
        let existing_client = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::clients::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_client.delete(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}
