pub use sea_schema::migration::*;

mod m20220219_224700_create_user_table;
mod m20220219_233200_create_auth_log_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220219_224700_create_user_table::Migration),
            Box::new(m20220219_233200_create_auth_log_table::Migration),
        ]
    }
}
