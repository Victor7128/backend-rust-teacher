use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Grado;

#[get("/grados")]
pub async fn get_grados() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/grados")]
pub async fn create_grado(grado: web::Json<Grado>) -> impl Responder {
    HttpResponse::Created().json(grado.into_inner())
}

#[put("/grados/{id}")]
pub async fn update_grado(web::Path(id): web::Path<uuid::Uuid>, grado: web::Json<Grado>) -> impl Responder {
    HttpResponse::Ok().json(grado.into_inner())
}

#[delete("/grados/{id}")]
pub async fn delete_grado(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}