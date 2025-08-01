use actix_web::web;

// Importar módulos de rutas
pub mod health;
pub mod periods;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(health::config)
            // Cuando tengas la implementación lista, añade:
            // .configure(periods::config)
    );
}