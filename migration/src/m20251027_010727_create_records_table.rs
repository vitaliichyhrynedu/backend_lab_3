use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251026_160714_create_users_table::User;
use crate::m20251026_233421_create_categories_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut user_id_fk = ForeignKey::create()
            .name("fk_user_id")
            .from(Record::Table, Record::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut category_id_fk = ForeignKey::create()
            .name("fk_category_id")
            .from(Record::Table, Record::CategoryId)
            .to(Category::Table, Category::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Record::Table)
                    .if_not_exists()
                    .col(pk_uuid(Record::Id))
                    .col(uuid(Record::UserId).not_null())
                    .foreign_key(&mut user_id_fk)
                    .col(uuid(Record::CategoryId).not_null())
                    .foreign_key(&mut category_id_fk)
                    .col(date_time(Record::CreatedAt))
                    .col(decimal(Record::Sum))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Record::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Record {
    Table,
    Id,
    UserId,
    CategoryId,
    CreatedAt,
    Sum,
}
