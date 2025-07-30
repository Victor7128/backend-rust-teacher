use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::sesion::Sesion;
use chrono::NaiveDate;

#[get("/sesiones")]
pub async fn listar_sesiones(pool: web::Data<PgPool>) -> impl Responder {
    let sesiones = sqlx::query_as!(
        Sesion,
        r#"SELECT id, nombre, seccion_id, bimestre_id, orden, fecha, creado_en FROM sesiones ORDER BY orden"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match sesiones {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar sesiones: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar sesiones")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevaSesion {
    pub nombre: String,
    pub seccion_id: Uuid,
    pub bimestre_id: Uuid,
    pub orden: i16,
    pub fecha: Option<NaiveDate>,
}

#[post("/sesiones")]
pub async fn crear_sesion(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevaSesion>
) -> impl Responder {
    let fecha = datos.fecha.unwrap_or_else(|| chrono::Local::now().date_naive());
    let resultado = sqlx::query!(
        "INSERT INTO sesiones (id, nombre, seccion_id, bimestre_id, orden, fecha) VALUES (gen_random_uuid(), $1, $2, $3, $4, $5) RETURNING id, nombre, seccion_id, bimestre_id, orden, fecha, creado_en",
        datos.nombre,
        datos.seccion_id,
        datos.bimestre_id,
        datos.orden,
        fecha
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Sesion {
            id: registro.id,
            nombre: registro.nombre,
            seccion_id: registro.seccion_id,
            bimestre_id: registro.bimestre_id,
            orden: registro.orden,
            fecha: registro.fecha,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear sesión: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear la sesión")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarSesion {
    pub nombre: String,
    pub seccion_id: Uuid,
    pub bimestre_id: Uuid,
    pub orden: i16,
    pub fecha: NaiveDate,
}

#[put("/sesiones/{id}")]
pub async fn editar_sesion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarSesion>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE sesiones SET nombre = $1, seccion_id = $2, bimestre_id = $3, orden = $4, fecha = $5 WHERE id = $6 RETURNING id, nombre, seccion_id, bimestre_id, orden, fecha, creado_en",
        datos.nombre,
        datos.seccion_id,
        datos.bimestre_id,
        datos.orden,
        datos.fecha,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Sesion {
            id: registro.id,
            nombre: registro.nombre,
            seccion_id: registro.seccion_id,
            bimestre_id: registro.bimestre_id,
            orden: registro.orden,
            fecha: registro.fecha,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar sesión: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar la sesión")
        }
    }
}

#[delete("/sesiones/{id}")]
pub async fn eliminar_sesion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM sesiones WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Sesión eliminada correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar sesión: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar la sesión")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sesiones")
            .service(listar_sesiones)
            .service(crear_sesion)
            .service(editar_sesion)
            .service(eliminar_sesion)
    );
}