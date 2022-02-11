use sea_orm::EntityTrait;
use std::env;

mod entity;
use entity::test_datetime;

#[actix_web::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    let db_url = env::var("DATABASE_URL").unwrap();
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let model = test_datetime::Entity::find()
        .into_model::<test_datetime::Model>()
        .all(&conn)
        .await?;

    let model_json = test_datetime::Entity::find()
        .into_json()
        .all(&conn)
        .await?;

    println!(
        "model serialized with serde_json:\n{:?}",
        serde_json::json!(model),
    );
    println!(
        "json returned from .into_json():\n{:?}",
        model_json,
    );

    Ok(())
}
