use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Seccion;

#[get("/secciones")]
pub async fn get_secciones() -> impl Responder {
    HttpResponse::Ok().json(vec![])
}

#[post("/secciones")]
pub async fn create_seccion(seccion: web::Json<Seccion>) -> impl Responder {
    HttpResponse::Created().json(seccion.into_inner())
}

#[put("/secciones/{id}")]
pub async fn update_seccion(web::Path(id): web::Path<uuid::Uuid>, seccion: web::Json<Seccion>) -> impl Responder {
    HttpResponse::Ok().json(seccion.into_inner())
}

#[delete("/secciones/{id}")]
pub async fn delete_seccion(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}