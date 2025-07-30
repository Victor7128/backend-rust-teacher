use actix_web::{web::ServiceConfig, web};
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Arc;

mod bd;
mod handlers;
mod models;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Crear pool de conexiones con manejo robusto
    let pool = Arc::new(
        bd::get_pool()
            .await
            .map_err(|e| {
                eprintln!("🔥 Error crítico de conexión a DB: {}", e);
                std::process::exit(1);
            })
    );

    // Configuración de servicios y rutas
    let config = move |cfg: &mut ServiceConfig| {
        // Compartir pool de conexiones con todos los handlers
        cfg.app_data(web::Data::from(pool.clone()));
        
        // Configurar rutas API
        cfg.service(
            web::scope("/api")
                // Auditoría
                .configure(handlers::auditoria_config)
                
                // Configuración académica
                .configure(handlers::bimestres_config)
                .configure(handlers::grados_config)
                .configure(handlers::secciones_config)
                .configure(handlers::sesiones_config)
                .configure(handlers::competencias_config)
                .configure(handlers::criterios_config)
                
                // Gestión de estudiantes
                .configure(handlers::alumnos_config)
                .configure(handlers::evaluaciones_config)
                
                // Exportación
                .configure(handlers::exportacion_config)
        );
    };

    Ok(config.into())
}