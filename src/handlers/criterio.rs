use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Criterio;

#[get("/criterios")]
pub async fn get_criterios() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/criterios")]
pub async fn create_criterio(criterio: web::Json<Criterio>) -> impl Responder {
    HttpResponse::Created().json(criterio.into_inner())
}

#[put("/criterios/{id}")]
pub async fn update_criterio(web::Path(id): web::Path<uuid::Uuid>, criterio: web::Json<Criterio>) -> impl Responder {
    HttpResponse::Ok().json(criterio.into_inner())
}

#[delete("/criterios/{id}")]
pub async fn delete_criterio(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}