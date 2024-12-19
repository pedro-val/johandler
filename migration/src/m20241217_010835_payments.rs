use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Payments::Table)
                    .col(pk_auto(Payments::Id))
                    .col(uuid_uniq(Payments::Pid))
                    .col(float(Payments::Value))
                    .col(date_null(Payments::PaymentDate))
                    .col(date(Payments::DueDate))
                    .col(string_null(Payments::PaymentMethod))
                    .col(string_null(Payments::Currency))
                    .col(boolean_null(Payments::PostponedPayment))
                    .col(integer(Payments::OrderId))
                    .col(boolean(Payments::Open))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payments-order_ids")
                            .from(Payments::Table, Payments::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    Pid,
    Value,
    PaymentDate,
    DueDate,
    PaymentMethod,
    Currency,
    PostponedPayment,
    OrderId,
    Open,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}
