use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(OrderFees::Table)
                    .col(pk_auto(OrderFees::Id))
                    .col(uuid_uniq(OrderFees::Pid))
                    .col(integer(OrderFees::FeeId))
                    .col(integer(OrderFees::OrderId))
                    .col(boolean(OrderFees::Open))
                    .col(float(OrderFees::Value))
                    .col(string_null(OrderFees::Info))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_fees-fee_ids")
                            .from(OrderFees::Table, OrderFees::FeeId)
                            .to(Fees::Table, Fees::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_fees-order_ids")
                            .from(OrderFees::Table, OrderFees::OrderId)
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
            .drop_table(Table::drop().table(OrderFees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderFees {
    Table,
    Id,
    Pid,
    FeeId,
    OrderId,
    Open,
    Value,
    Info,
    
}

#[derive(DeriveIden)]
enum Fees {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}
