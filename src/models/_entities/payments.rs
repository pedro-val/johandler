//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    #[sea_orm(column_type = "Float")]
    pub value: f32,
    pub payment_date: Option<Date>,
    pub due_date: Date,
    pub payment_method: Option<String>,
    pub currency: Option<String>,
    pub postponed_payment: Option<bool>,
    pub order_id: i32,
    pub open: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Orders,
    #[sea_orm(has_many = "super::postponed_payments::Entity")]
    PostponedPayments,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::postponed_payments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostponedPayments.def()
    }
}
