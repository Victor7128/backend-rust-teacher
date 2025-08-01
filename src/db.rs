use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use dotenv::dotenv;
use anyhow::Result;

// Tipo público para el pool de conexiones
pub type DbPool = PgPool;

// Inicializa el pool de conexiones usando DATABASE_URL
pub async fn init_db() -> Result<DbPool> {
    dotenv().ok();  // Carga variables de entorno desde .env
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    // Configuración del pool con parámetros óptimos
    let pool = PgPoolOptions::new()
        .max_connections(5)  // Máximo de conexiones simultáneas
        .min_connections(1)  // Mínimo de conexiones en reposo
        .acquire_timeout(Duration::from_secs(5))  // Tiempo de espera para conexiones
        .idle_timeout(Duration::from_secs(300))  // Tiempo máximo de inactividad
        .max_lifetime(Duration::from_secs(1800))  // Tiempo máximo de vida
        .connect(&database_url)
        .await?;
    
    // Verificar conexión
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    println!("✅ Conexión a la base de datos establecida exitosamente");
    
    Ok(pool)
}