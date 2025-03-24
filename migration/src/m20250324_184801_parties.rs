use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Parties::Table)
                    .col(pk_auto(Parties::Id))
                    .col(uuid_uniq(Parties::Pid))
                    .col(string_null(Parties::Name))
                    .col(integer(Parties::UserId))
                    .col(boolean_null(Parties::Active))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-parties-user_ids")
                            .from(Parties::Table, Parties::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parties::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Parties {
    Table,
    Id,
    Pid,
    Name,
    UserId,
    Active,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
