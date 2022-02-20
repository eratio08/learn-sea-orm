use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue::NotSet, Database, Set};

use entity::sea_orm;

pub use entity::auth_log;
pub use entity::user;

#[async_std::main]
async fn main() {
    let db = Database::connect("postgres://sea-orm:changeme@localhost:3002/sea-orm")
        .await
        .unwrap();

    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set("Test".to_owned()),
        last_login: NotSet,
    };

    let user: user::Model = user.insert(&db).await.unwrap();
    println!("{:?}", user);
}
