use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Clients::Table)
                    .col(pk_auto(Clients::Id))
                    .col(uuid_uniq(Clients::Pid))
                    .col(string(Clients::Name))
                    .col(string(Clients::Contact))
                    .col(string(Clients::Phone))
                    .col(string_null(Clients::Phone2))
                    .col(string(Clients::Email))
                    .col(integer_null(Clients::PartnerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clients-partner_ids")
                            .from(Clients::Table, Clients::PartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Clients::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Clients {
    Table,
    Id,
    Pid,
    Name,
    Contact,
    Phone,
    Phone2,
    Email,
    PartnerId,
}

#[derive(DeriveIden)]
enum Partners {
    Table,
    Id,
}
