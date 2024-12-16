#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241216_021800_processes;
mod m20241216_022307_partners;
mod m20241216_022614_sellers;
mod m20241216_022844_clients;
mod m20241216_025420_orders;
mod m20241216_030042_payments;
mod m20241216_030204_postponed_payments;
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
            Box::new(m20241216_030042_payments::Migration),
            Box::new(m20241216_030204_postponed_payments::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
