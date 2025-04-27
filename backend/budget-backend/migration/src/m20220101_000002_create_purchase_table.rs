// m20220101_000002_create_purchase_table.rs

use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_category_table::Category;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_purchase_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Purchase table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Purchase::Table)
                    .col(
                        ColumnDef::new(Purchase::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Purchase::Desc).string().not_null())
                    .col(ColumnDef::new(Purchase::Amount).integer().not_null())
                    .col(ColumnDef::new(Purchase::Date).string().not_null())
                    .col(ColumnDef::new(Purchase::Cat_Id).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-purchase-category_id")
                            .from(Purchase::Table, Purchase::Cat_Id)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Purchase table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Purchase::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Purchase {
    Table,
    Id,
    Desc,
    Amount,
    Date,
    Cat_Id,
}
