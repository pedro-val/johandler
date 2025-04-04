//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "clients")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub name: String,
    pub contact: String,
    pub phone: String,
    pub phone2: Option<String>,
    pub email: String,
    pub partner_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::orders::Entity")]
    Orders,
    #[sea_orm(
        belongs_to = "super::partners::Entity",
        from = "Column::PartnerId",
        to = "super::partners::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Partners,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::partners::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Partners.def()
    }
}
