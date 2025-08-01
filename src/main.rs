mod db;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::web;
use db::init_db;
use shuttle_actix_web::ShuttleActixWeb;
use actix_web::web::ServiceConfig;
use anyhow::Error;

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Habilitar logs de depuración
    std::env::set_var("RUST_LOG", "debug");
    
    println!("🚀 Iniciando servidor...");
    
    // Inicializar pool de base de datos
    let db_pool = match init_db().await {
        Ok(pool) => {
            println!("✅ Conexión a la base de datos establecida exitosamente");
            pool
        },
        Err(e) => {
            eprintln!("❌ Error al conectar a la base de datos: {}", e);
            return Err(Error::from(e).into());
        }
    };
    
    println!("🌐 Servidor configurado con Shuttle");
    
    let config = move |cfg: &mut ServiceConfig| {
        // Configurar CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        // Registrar el pool de base de datos como app data
        cfg.app_data(web::Data::new(db_pool.clone())) // Compartir pool con toda la app
           .service(
               web::scope("")
                   .wrap(cors)
                   .configure(routes::config)
           );
    };

    Ok(config.into())
}