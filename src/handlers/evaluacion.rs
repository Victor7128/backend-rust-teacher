use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Evaluacion;

#[get("/evaluaciones")]
pub async fn get_evaluaciones() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/evaluaciones")]
pub async fn create_evaluacion(evaluacion: web::Json<Evaluacion>) -> impl Responder {
    HttpResponse::Created().json(evaluacion.into_inner())
}

#[put("/evaluaciones/{id}")]
pub async fn update_evaluacion(web::Path(id): web::Path<uuid::Uuid>, evaluacion: web::Json<Evaluacion>) -> impl Responder {
    HttpResponse::Ok().json(evaluacion.into_inner())
}

#[delete("/evaluaciones/{id}")]
pub async fn delete_evaluacion(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}