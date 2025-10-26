pub use sea_orm_migration::prelude::*;

mod m20251026_160714_create_users_table;
mod m20251026_233421_create_categories_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251026_160714_create_users_table::Migration),
            Box::new(m20251026_233421_create_categories_table::Migration),
        ]
    }
}
