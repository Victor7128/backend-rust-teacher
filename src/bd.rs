use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("AIVEN_DATABASE_URL")
        .expect("Error en la variable de entorno");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}