use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Sesion;

#[get("/sesiones")]
pub async fn get_sesiones() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/sesiones")]
pub async fn create_sesion(sesion: web::Json<Sesion>) -> impl Responder {
    HttpResponse::Created().json(sesion.into_inner())
}

#[put("/sesiones/{id}")]
pub async fn update_sesion(web::Path(id): web::Path<uuid::Uuid>, sesion: web::Json<Sesion>) -> impl Responder {
    HttpResponse::Ok().json(sesion.into_inner())
}

#[delete("/sesiones/{id}")]
pub async fn delete_sesion(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}