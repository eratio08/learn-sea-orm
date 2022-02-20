use sea_schema::{
    migration::{sea_query, *},
    sea_query::ColumnDef,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220219_224700_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entity::user::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::user::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(entity::user::Column::LastLogin).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::user::Entity)
                    .to_owned(),
            )
            .await
    }
}
