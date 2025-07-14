use actix_web::{get, post, web, HttpResponse};
use crate::models::{Sesion, Competencia, Criterio, NuevaCompetencia, NuevoCriterio, ErrorResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/sesiones/{seccion_id}")]
pub async fn crear_sesion(
    db: web::Data<PgPool>,
    seccion_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    // Obtener el próximo número de sesión
    let session_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sesiones WHERE seccion_id = $1"
    )
    .bind(*seccion_id)
    .fetch_one(db.get_ref())
    .await?;
    
    let sesion = Sesion::nueva(*seccion_id, (session_count + 1) as i16);
    
    let created = sqlx::query_as!(
        Sesion,
        "INSERT INTO sesiones (id, nombre, seccion_id, orden) VALUES ($1, $2, $3, $4) RETURNING *",
        sesion.id,
        sesion.nombre,
        sesion.seccion_id,
        sesion.orden
    )
    .fetch_one(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(created))
}

#[post("/competencias")]
pub async fn crear_competencia(
    db: web::Data<PgPool>,
    nueva: web::Json<NuevaCompetencia>,
) -> Result<HttpResponse, ErrorResponse> {
    nueva.validate()?;
    
    let competencia = Competencia::from(nueva.into_inner());
    
    let created = sqlx::query_as!(
        Competencia,
        "INSERT INTO competencias (id, nombre, sesion_id) VALUES ($1, $2, $3) RETURNING *",
        competencia.id,
        competencia.nombre,
        competencia.sesion_id
    )
    .fetch_one(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(created))
}

#[post("/criterios")]
pub async fn crear_criterio(
    db: web::Data<PgPool>,
    nuevo: web::Json<NuevoCriterio>,
) -> Result<HttpResponse, ErrorResponse> {
    nuevo.validate()?;
    
    let criterio = Criterio::from(nuevo.into_inner());
    
    let created = sqlx::query_as!(
        Criterio,
        "INSERT INTO criterios (id, nombre, descripcion, competencia_id) VALUES ($1, $2, $3, $4) RETURNING *",
        criterio.id,
        criterio.nombre,
        criterio.descripcion,
        criterio.competencia_id
    )
    .fetch_one(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(created))
}

#[get("/sesiones/{seccion_id}")]
pub async fn listar_sesiones_por_seccion(
    db: web::Data<PgPool>,
    seccion_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let sesiones = sqlx::query_as!(
        Sesion,
        "SELECT * FROM sesiones WHERE seccion_id = $1 ORDER BY orden",
        *seccion_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(sesiones))
}

#[get("/competencias/{sesion_id}")]
pub async fn listar_competencias_por_sesion(
    db: web::Data<PgPool>,
    sesion_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let competencias = sqlx::query_as!(
        Competencia,
        "SELECT * FROM competencias WHERE sesion_id = $1",
        *sesion_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(competencias))
}

#[get("/criterios/{competencia_id}")]
pub async fn listar_criterios_por_competencia(
    db: web::Data<PgPool>,
    competencia_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let criterios = sqlx::query_as!(
        Criterio,
        "SELECT * FROM criterios WHERE competencia_id = $1",
        *competencia_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(criterios))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crear_sesion)
        .service(crear_competencia)
        .service(crear_criterio)
        .service(listar_sesiones_por_seccion)
        .service(listar_competencias_por_sesion)
        .service(listar_criterios_por_competencia);
}