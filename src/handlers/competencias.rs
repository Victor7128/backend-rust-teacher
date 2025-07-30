use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::competencia::Competencia;

#[get("/competencias")]
pub async fn listar_competencias(pool: web::Data<PgPool>) -> impl Responder {
    let competencias = sqlx::query_as!(
        Competencia,
        r#"SELECT id, nombre, descripcion, sesion_id, orden, creado_en FROM competencias ORDER BY orden"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match competencias {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar competencias: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar competencias")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevaCompetencia {
    pub nombre: Option<String>, // Puede no enviarse para usar trigger
    pub descripcion: Option<String>,
    pub sesion_id: Uuid,
    pub orden: i16,
}

#[post("/competencias")]
pub async fn crear_competencia(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevaCompetencia>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO competencias (id, nombre, descripcion, sesion_id, orden) VALUES (gen_random_uuid(), $1, $2, $3, $4) RETURNING id, nombre, descripcion, sesion_id, orden, creado_en",
        datos.nombre,
        datos.descripcion,
        datos.sesion_id,
        datos.orden
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Competencia {
            id: registro.id,
            nombre: registro.nombre,
            descripcion: registro.descripcion,
            sesion_id: registro.sesion_id,
            orden: registro.orden,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear competencia: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear la competencia")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarCompetencia {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub sesion_id: Uuid,
    pub orden: i16,
}

#[put("/competencias/{id}")]
pub async fn editar_competencia(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarCompetencia>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE competencias SET nombre = $1, descripcion = $2, sesion_id = $3, orden = $4 WHERE id = $5 RETURNING id, nombre, descripcion, sesion_id, orden, creado_en",
        datos.nombre,
        datos.descripcion,
        datos.sesion_id,
        datos.orden,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Competencia {
            id: registro.id,
            nombre: registro.nombre,
            descripcion: registro.descripcion,
            sesion_id: registro.sesion_id,
            orden: registro.orden,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar competencia: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar la competencia")
        }
    }
}

#[delete("/competencias/{id}")]
pub async fn eliminar_competencia(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM competencias WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Competencia eliminada correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar competencia: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar la competencia")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/competencias")
            .service(listar_competencias)
            .service(crear_competencia)
            .service(editar_competencia)
            .service(eliminar_competencia)
    );
}