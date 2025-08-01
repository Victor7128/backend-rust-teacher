use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/health")]
pub async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    // Verificar conexión a la base de datos
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "message": "API is healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
        Err(e) => {
            println!("Error en health check: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "message": format!("Database connection error: {}", e),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        },
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}