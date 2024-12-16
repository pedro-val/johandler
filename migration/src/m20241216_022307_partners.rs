use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Partners::Table)
                    .col(pk_auto(Partners::Id))
                    .col(uuid_uniq(Partners::Pid))
                    .col(string(Partners::Name))
                    .col(string_null(Partners::Information))
                    .col(string_null(Partners::Phone))
                    .col(string_null(Partners::Email))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Partners::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Partners {
    Table,
    Id,
    Pid,
    Name,
    Information,
    Phone,
    Email,
}
