use actix_web::{App, HttpServer, web};
use env_logger;
use log::info;

mod config;
mod db;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = config::Config::from_env().expect("Error loading config");
    let pool = db::get_pool(&config)
        .await
        .expect("Error connecting to DB");

    info!("Starting server on port {}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Aquí agregas los handlers:
            .service(handlers::alumno::get_alumnos)
            .service(handlers::alumno::create_alumno)
            .service(handlers::alumno::update_alumno)
            .service(handlers::alumno::delete_alumno)
            // Repite para los demás handlers...
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}