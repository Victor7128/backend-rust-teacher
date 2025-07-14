mod handlers;
mod models;
mod errors;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use handlers::{auth, academic, students, sessions, evaluation};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] connection_string: String,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Configuración especial para Shuttle
    let conn_string = if connection_string.contains('?') {
        format!("{}&sslmode=require", connection_string)
    } else {
        format!("{}?sslmode=require", connection_string)
    };

    // Crear pool de conexiones
    let pool = PgPool::connect(&conn_string)
        .await
        .expect("Failed to create PgPool");
    
    // Ejecutar migraciones
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed migrations");

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .wrap(Logger::default())
            .configure(auth::config)
            .configure(academic::config)
            .configure(students::config)
            .configure(sessions::config)
            .configure(evaluation::config);
    };

    Ok(config.into())
}