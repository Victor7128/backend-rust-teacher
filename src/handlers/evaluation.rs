use actix_web::{get, post, web, HttpResponse};
use crate::models::{Evaluacion, NuevaEvaluacion, ErrorResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/evaluaciones")]
pub async fn crear_evaluacion(
    db: web::Data<PgPool>,
    nueva: web::Json<NuevaEvaluacion>,
) -> Result<HttpResponse, ErrorResponse> {
    nueva.validate()?;
    
    let evaluacion = Evaluacion::from(nueva.into_inner());
    
    let created = sqlx::query_as!(
        Evaluacion,
        r#"
        INSERT INTO evaluaciones (id, estudiante_id, criterio_id, valor)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (estudiante_id, criterio_id) DO UPDATE
        SET valor = EXCLUDED.valor
        RETURNING *
        "#,
        evaluacion.id,
        evaluacion.estudiante_id,
        evaluacion.criterio_id,
        evaluacion.valor
    )
    .fetch_one(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(created))
}

#[get("/evaluaciones/{criterio_id}")]
pub async fn obtener_evaluaciones_por_criterio(
    db: web::Data<PgPool>,
    criterio_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let evaluaciones = sqlx::query_as!(
        Evaluacion,
        "SELECT * FROM evaluaciones WHERE criterio_id = $1",
        *criterio_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(evaluaciones))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crear_evaluacion)
        .service(obtener_evaluaciones_por_criterio);
}