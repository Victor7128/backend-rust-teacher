use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::Alumno;

// GET /alumnos
#[get("/alumnos")]
pub async fn get_alumnos() -> impl Responder {
    // Lógica para obtener alumnos
    HttpResponse::Ok().json(vec![])
}

// POST /alumnos
#[post("/alumnos")]
pub async fn create_alumno(alumno: web::Json<Alumno>) -> impl Responder {
    // Lógica para crear alumno
    HttpResponse::Created().json(alumno.into_inner())
}

// PUT /alumnos/{id}
#[put("/alumnos/{id}")]
pub async fn update_alumno(web::Path(id): web::Path<uuid::Uuid>, alumno: web::Json<Alumno>) -> impl Responder {
    // Lógica para actualizar alumno
    HttpResponse::Ok().json(alumno.into_inner())
}

// DELETE /alumnos/{id}
#[delete("/alumnos/{id}")]
pub async fn delete_alumno(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    // Lógica para borrar alumno
    HttpResponse::NoContent().finish()
}