use super::_entities::processes::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Processes = Entity;
use super::_entities::processes;
use loco_rs::model::ModelError;
use loco_rs::model::{self, ModelResult};
use sea_orm::ActiveValue;
use sea_orm::{IntoActiveModel, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewProcess {
    pub case_type: String,
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

impl super::_entities::processes::Model {
    /// finds a process by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find process by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let process = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        process.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a process by the provided id
    ///
    /// # Errors
    ///
    /// When could not find process by the given id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let process = Entity::find_by_id(id).one(db).await?;
        process.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds all processes
    ///
    /// # Errors
    ///
    /// When could not find processes or DB query error
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let processes = Entity::find().all(db).await?;
        Ok(processes)
    }

    /// creates a new process
    ///
    /// # Errors
    ///
    /// When could not create process or DB query error
    pub async fn create(
        db: &DatabaseConnection,
        process: CreateNewProcess,
    ) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let _process = processes::ActiveModel {
            case_type: ActiveValue::Set(process.case_type),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }

    /// updates a process
    ///
    /// # Errors
    ///
    /// When could not update process or DB query error
    pub async fn update(
        db: &DatabaseConnection,
        pid: &str,
        process: CreateNewProcess,
    ) -> ModelResult<Vec<Self>> {
        let existing_process = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let mut edited_process = existing_process.into_active_model();
        edited_process.case_type = ActiveValue::Set(process.case_type);
        let txn = db.begin().await?;
        let _process = edited_process.update(&txn).await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }

    /// deletes a process
    ///
    /// # Errors
    ///
    /// When could not delete process or DB query error
    pub async fn delete(db: &DatabaseConnection, pid: &str) -> ModelResult<Vec<Self>> {
        let existing_process = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::processes::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        let txn = db.begin().await?;
        existing_process.delete(&txn).await?;
        txn.commit().await?;
        let response = Entity::find().all(db).await?;
        Ok(response)
    }
}
