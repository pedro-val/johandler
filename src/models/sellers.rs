use super::_entities::sellers::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Sellers = Entity;
use crate::models::_entities::sellers;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::TransactionTrait;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewSeller {
    pub name: String,
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

impl super::_entities::sellers::Model {
    /// finds a seller by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find seller by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let seller = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::sellers::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        seller.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// find seller by id
    ///
    /// # Errors
    ///
    /// When could not find seller by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let seller = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::sellers::Column::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        seller.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all sellers
    ///
    /// # Errors
    ///
    /// When could not find sellers or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let sellers = Entity::find().all(db).await?;
        Ok(sellers)
    }

    /// creates a new seller
    ///
    /// # Errors
    ///
    /// When could not create seller or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        seller: CreateNewSeller,
    ) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _seller = sellers::ActiveModel {
            name: ActiveValue::Set(seller.name),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// updates a seller
    ///
    /// # Errors
    ///
    /// When could not update seller or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        seller: CreateNewSeller,
    ) -> ModelResult<Vec<Self>> {
        let vendor = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::sellers::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_seller = vendor.into_active_model();
        edited_seller.name = ActiveValue::Set(seller.name);
        let txn = db.begin().await?;
        let _seller = edited_seller.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes a seller
    ///
    /// # Errors
    ///
    /// When could not delete seller or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<Vec<Self>> {
        let vendor = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::sellers::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        vendor.delete(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
