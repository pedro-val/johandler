use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(PostponedPayments::Table)
                    .col(pk_auto(PostponedPayments::Id))
                    .col(uuid_uniq(PostponedPayments::Pid))
                    .col(integer(PostponedPayments::PaymentId))
                    .col(date(PostponedPayments::PostponedDate))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-postponed_payments-payment_ids")
                            .from(PostponedPayments::Table, PostponedPayments::PaymentId)
                            .to(Payments::Table, Payments::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostponedPayments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostponedPayments {
    Table,
    Id,
    Pid,
    PaymentId,
    PostponedDate,
    
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
}
