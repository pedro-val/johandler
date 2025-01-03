use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(ProcessesFees::Table)
                    .col(pk_auto(ProcessesFees::Id))
                    .col(uuid_uniq(ProcessesFees::Pid))
                    .col(integer(ProcessesFees::ProcessId))
                    .col(integer(ProcessesFees::FeeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-processes_fees-process_ids")
                            .from(ProcessesFees::Table, ProcessesFees::ProcessId)
                            .to(Processes::Table, Processes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-processes_fees-fee_ids")
                            .from(ProcessesFees::Table, ProcessesFees::FeeId)
                            .to(Fees::Table, Fees::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProcessesFees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProcessesFees {
    Table,
    Id,
    Pid,
    ProcessId,
    FeeId,
}

#[derive(DeriveIden)]
enum Processes {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Fees {
    Table,
    Id,
}
