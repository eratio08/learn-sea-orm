use sea_schema::{
    migration::{sea_query, *},
    sea_query::{ColumnDef, Index},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220219_233200_create_auth_log_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::auth_log::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entity::auth_log::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::auth_log::Column::LoginAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(entity::auth_log::Column::UserId)
                            .col(entity::auth_log::Column::LoginAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::auth_log::Entity)
                    .to_owned(),
            )
            .await
    }
}
