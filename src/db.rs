use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::config::Config;

pub async fn get_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}