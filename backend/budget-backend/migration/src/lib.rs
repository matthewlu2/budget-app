// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20220101_000001_create_category_table;
mod m20220101_000002_create_purchase_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20220101_000001_create_category_table::Migration),
            Box::new(m20220101_000002_create_purchase_table::Migration),
        ]
    }
}