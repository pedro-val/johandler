use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Orders::Table)
                    .col(pk_auto(Orders::Id))
                    .col(uuid_uniq(Orders::Pid))
                    .col(integer(Orders::ClientId))
                    .col(integer(Orders::ProcessId))
                    .col(boolean(Orders::Open))
                    .col(float(Orders::Payout))
                    .col(float(Orders::Fee))
                    .col(float_null(Orders::PartnerFee))
                    .col(integer(Orders::SellerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-seller_ids")
                            .from(Orders::Table, Orders::SellerId)
                            .to(Sellers::Table, Sellers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-client_ids")
                            .from(Orders::Table, Orders::ClientId)
                            .to(Clients::Table, Clients::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-process_ids")
                            .from(Orders::Table, Orders::ProcessId)
                            .to(Processes::Table, Processes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    Pid,
    ClientId,
    ProcessId,
    Open,
    Fee,
    Payout,
    SellerId,
    PartnerFee,
}

#[derive(DeriveIden)]
enum Clients {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Processes {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Sellers {
    Table,
    Id,
}
