use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Bimestre;

#[get("/bimestres")]
pub async fn get_bimestres() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/bimestres")]
pub async fn create_bimestre(bimestre: web::Json<Bimestre>) -> impl Responder {
    HttpResponse::Created().json(bimestre.into_inner())
}

#[put("/bimestres/{id}")]
pub async fn update_bimestre(web::Path(id): web::Path<uuid::Uuid>, bimestre: web::Json<Bimestre>) -> impl Responder {
    HttpResponse::Ok().json(bimestre.into_inner())
}

#[delete("/bimestres/{id}")]
pub async fn delete_bimestre(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}