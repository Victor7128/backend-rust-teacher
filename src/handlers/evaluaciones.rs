use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::evaluacion::Evaluacion;

#[get("/evaluaciones")]
pub async fn listar_evaluaciones(pool: web::Data<PgPool>) -> impl Responder {
    let evaluaciones = sqlx::query_as!(
        Evaluacion,
        r#"SELECT id, estudiante_id, criterio_id, calificacion, observacion, creado_en, actualizado_en 
        FROM evaluaciones ORDER BY creado_en DESC"#  // Cambiado valor por calificacion
    )
    .fetch_all(pool.get_ref())
    .await;

    match evaluaciones {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar evaluaciones: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar evaluaciones")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevaEvaluacion {
    pub estudiante_id: Uuid,
    pub criterio_id: Uuid,
    pub calificacion: String,  // Cambiado de valor a calificacion
    pub observacion: Option<String>,
}

#[post("/evaluaciones")]
pub async fn crear_evaluacion(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevaEvaluacion>
) -> impl Responder {
    // Validación de calificacion permitida
    let calificacion = datos.calificacion.as_str();
    if !["AD", "A", "B", "C"].contains(&calificacion) {
        return HttpResponse::BadRequest().body("Calificación no permitida");
    }

    let resultado = sqlx::query!(
        "INSERT INTO evaluaciones (id, estudiante_id, criterio_id, calificacion, observacion) 
        VALUES (gen_random_uuid(), $1, $2, $3, $4) 
        RETURNING id, estudiante_id, criterio_id, calificacion, observacion, creado_en, actualizado_en",
        datos.estudiante_id,
        datos.criterio_id,
        calificacion,
        datos.observacion
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Evaluacion {
            id: registro.id,
            estudiante_id: registro.estudiante_id,
            criterio_id: registro.criterio_id,
            calificacion: registro.calificacion,  // Cambiado de valor a calificacion
            observacion: registro.observacion,
            creado_en: registro.creado_en,
            actualizado_en: registro.actualizado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear evaluación: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear la evaluación")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarEvaluacion {
    pub calificacion: String,  // Cambiado de valor a calificacion
    pub observacion: Option<String>,
}

#[put("/evaluaciones/{id}")]
pub async fn editar_evaluacion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarEvaluacion>
) -> impl Responder {
    let id = path.into_inner();
    // Validación de calificacion permitida
    let calificacion = datos.calificacion.as_str();
    if !["AD", "A", "B", "C"].contains(&calificacion) {
        return HttpResponse::BadRequest().body("Calificación no permitida");
    }

    let resultado = sqlx::query!(
        "UPDATE evaluaciones SET calificacion = $1, observacion = $2 
        WHERE id = $3 
        RETURNING id, estudiante_id, criterio_id, calificacion, observacion, creado_en, actualizado_en",
        calificacion,
        datos.observacion,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Evaluacion {
            id: registro.id,
            estudiante_id: registro.estudiante_id,
            criterio_id: registro.criterio_id,
            calificacion: registro.calificacion,  // Cambiado de valor a calificacion
            observacion: registro.observacion,
            creado_en: registro.creado_en,
            actualizado_en: registro.actualizado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar evaluación: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar la evaluación")
        }
    }
}

#[delete("/evaluaciones/{id}")]
pub async fn eliminar_evaluacion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM evaluaciones WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Evaluación eliminada correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar evaluación: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar la evaluación")
        }
    }
}