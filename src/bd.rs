use sqlx::{PgPool, postgres::PgPoolOptions, Error};
use std::{env, time::Duration};
use tokio::time;

pub async fn get_pool() -> Result<PgPool, Error> {
    let database_url = env::var("AIVEN_DATABASE_URL")
        .map_err(|e| Error::Configuration(format!(
            "Error en variable AIVEN_DATABASE_URL: {}. Verifique su configuración", e
        ).into()))?;

    // Configuración optimizada para uso móvil y sesiones largas
    let pool = PgPoolOptions::new()
        .max_connections(8)  // Balance entre recursos y disponibilidad
        .min_connections(2)  // Mantener mínimo para reconexiones rápidas
        .acquire_timeout(Duration::from_secs(60))  // 1 minuto para adquirir conexión
        .idle_timeout(None)  // Nuncer cerrar por inactividad (importante para móviles)
        .max_lifetime(Duration::from_secs(60 * 60 * 3)) // 3 horas de vida máxima
        .connect(&database_url)
        .await?;

    // Verificar conexión con reintentos
    let mut intentos = 0;
    loop {
        match sqlx::query("SELECT 1").execute(&pool).await {
            Ok(_) => break,
            Err(e) => {
                intentos += 1;
                if intentos > 5 {
                    return Err(e);
                }
                // Espera exponencial: 1s, 2s, 4s, 8s, 16s
                let delay = Duration::from_secs(2u64.pow(intentos - 1));
                time::sleep(delay).await;
            }
        }
    }

    Ok(pool)
}