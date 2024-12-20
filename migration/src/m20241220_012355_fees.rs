use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Fees::Table)
                    .col(pk_auto(Fees::Id))
                    .col(uuid_uniq(Fees::Pid))
                    .col(string(Fees::Fee))
                    .col(string_null(Fees::Type))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Fees {
    Table,
    Id,
    Pid,
    Fee,
    Type,
    
}

