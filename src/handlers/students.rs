use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::models::{Alumno, NuevoAlumno, ErrorResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/alumnos")]
pub async fn crear_alumno(
    db: web::Data<PgPool>,
    nuevo_alumno: web::Json<NuevoAlumno>,
) -> Result<HttpResponse, ErrorResponse> {
    nuevo_alumno.validate()?;
    
    let alumno = Alumno::from(nuevo_alumno.into_inner());
    
    sqlx::query!(
        "INSERT INTO alumnos (id, nombre, seccion_id) VALUES ($1, $2, $3)",
        alumno.id,
        alumno.nombre,
        alumno.seccion_id
    )
    .execute(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(alumno))
}

#[get("/alumnos/{seccion_id}")]
pub async fn listar_alumnos_por_seccion(
    db: web::Data<PgPool>,
    seccion_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let alumnos = sqlx::query_as!(
        Alumno,
        "SELECT * FROM alumnos WHERE seccion_id = $1 ORDER BY nombre",
        *seccion_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(alumnos))
}

#[put("/alumnos/{id}")]
pub async fn actualizar_alumno(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
    actualizado: web::Json<NuevoAlumno>,
) -> Result<HttpResponse, ErrorResponse> {
    actualizado.validate()?;
    
    let alumno = sqlx::query_as!(
        Alumno,
        "UPDATE alumnos SET nombre = $1, seccion_id = $2 WHERE id = $3 RETURNING *",
        actualizado.nombre,
        actualizado.seccion_id,
        *id
    )
    .fetch_one(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(alumno))
}

#[delete("/alumnos/{id}")]
pub async fn eliminar_alumno(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    sqlx::query!(
        "DELETE FROM alumnos WHERE id = $1",
        *id
    )
    .execute(db.get_ref())
    .await?;
    
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crear_alumno)
        .service(listar_alumnos_por_seccion)
        .service(actualizar_alumno)
        .service(eliminar_alumno);
}