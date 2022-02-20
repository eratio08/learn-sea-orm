use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user", schema_name = "sea-orm")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    name: String,
    last_login: Option<DateTimeWithTimeZone>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_log::Entity")]
    AuthLog,
}

impl ActiveModelBehavior for ActiveModel {}
