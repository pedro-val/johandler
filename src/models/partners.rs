use super::_entities::partners::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
pub type Partners = Entity;
use crate::models::_entities::partners;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::IntoActiveModel;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewPartner {
    pub name: String,
    pub information: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
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

impl super::_entities::partners::Model {
    /// finds a partner by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find partner by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let partner = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::partners::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        partner.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// find by partner id
    /// 
    /// # Errors
    /// 
    /// When could not find partner by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let partner = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::partners::Column::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        partner.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all partners
    ///
    /// # Errors
    ///
    /// When could not find partners or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let partners = Entity::find().all(db).await?;
        Ok(partners)
    }

    /// creates a new partner
    ///
    /// # Errors
    ///
    /// When could not create partner or DB query error
    pub async fn create(db: &DatabaseConnection, partner: CreateNewPartner) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _partner = partners::ActiveModel {
            name: ActiveValue::Set(partner.name),
            information: ActiveValue::Set(partner.information),
            phone: ActiveValue::Set(partner.phone),
            email: ActiveValue::Set(partner.email),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// updates a partner
    ///
    /// # Errors
    ///
    /// When could not update partner or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        partner: CreateNewPartner,
    ) -> ModelResult<Vec<Self>> {
        let existing_partner = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::partners::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_partner = existing_partner.into_active_model();
        edited_partner.name = ActiveValue::Set(partner.name);
        edited_partner.information = ActiveValue::Set(partner.information);
        edited_partner.phone = ActiveValue::Set(partner.phone);
        edited_partner.email = ActiveValue::Set(partner.email);
        let txn = db.begin().await?;
        let _partner = edited_partner.update(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }

    /// deletes a partner
    ///
    /// # Errors
    ///
    /// When could not delete partner or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<Vec<Self>> {
        let existing_partner = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::partners::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_partner.delete(&txn).await?;
        txn.commit().await?;
        let response = Self::find_all(db).await?;
        Ok(response)
    }
}
