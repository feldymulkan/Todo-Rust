use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn conn_db() -> PgPool {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}