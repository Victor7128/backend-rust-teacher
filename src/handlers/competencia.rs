use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Competencia;

#[get("/competencias")]
pub async fn get_competencias() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/competencias")]
pub async fn create_competencia(competencia: web::Json<Competencia>) -> impl Responder {
    HttpResponse::Created().json(competencia.into_inner())
}

#[put("/competencias/{id}")]
pub async fn update_competencia(web::Path(id): web::Path<uuid::Uuid>, competencia: web::Json<Competencia>) -> impl Responder {
    HttpResponse::Ok().json(competencia.into_inner())
}

#[delete("/competencias/{id}")]
pub async fn delete_competencia(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}