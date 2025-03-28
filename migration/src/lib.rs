#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241216_021800_processes;
mod m20241216_022307_partners;
mod m20241216_022614_sellers;
mod m20241216_022844_clients;
mod m20241216_025420_orders;
mod m20241217_010835_payments;
mod m20241217_011107_postponed_payments;
mod m20241220_012355_fees;
mod m20241220_012613_order_fees;
mod m20250103_173848_processes_fees;
mod m20250324_184801_parties;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241216_021800_processes::Migration),
            Box::new(m20241216_022307_partners::Migration),
            Box::new(m20241216_022614_sellers::Migration),
            Box::new(m20241216_022844_clients::Migration),
            Box::new(m20241216_025420_orders::Migration),
            Box::new(m20241217_010835_payments::Migration),
            Box::new(m20241217_011107_postponed_payments::Migration),
            Box::new(m20241220_012355_fees::Migration),
            Box::new(m20241220_012613_order_fees::Migration),
            Box::new(m20250103_173848_processes_fees::Migration),
            Box::new(m20250324_184801_parties::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
