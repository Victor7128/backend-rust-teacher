use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::criterio::Criterio;

#[get("/criterios")]
pub async fn listar_criterios(pool: web::Data<PgPool>) -> impl Responder {
    let criterios = sqlx::query_as!(
        Criterio,
        r#"SELECT id, nombre, descripcion, competencia_id, orden, creado_en FROM criterios ORDER BY orden"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match criterios {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar criterios: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar criterios")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevoCriterio {
    pub nombre: Option<String>, // Puede omitirse para activar el trigger
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
    pub orden: i16,
}

#[post("/criterios")]
pub async fn crear_criterio(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevoCriterio>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO criterios (id, nombre, descripcion, competencia_id, orden) VALUES (gen_random_uuid(), $1, $2, $3, $4) RETURNING id, nombre, descripcion, competencia_id, orden, creado_en",
        datos.nombre,
        datos.descripcion,
        datos.competencia_id,
        datos.orden
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Criterio {
            id: registro.id,
            nombre: registro.nombre,
            descripcion: registro.descripcion,
            competencia_id: registro.competencia_id,
            orden: registro.orden,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear criterio: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear el criterio")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarCriterio {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
    pub orden: i16,
}

#[put("/criterios/{id}")]
pub async fn editar_criterio(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarCriterio>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE criterios SET nombre = $1, descripcion = $2, competencia_id = $3, orden = $4 WHERE id = $5 RETURNING id, nombre, descripcion, competencia_id, orden, creado_en",
        datos.nombre,
        datos.descripcion,
        datos.competencia_id,
        datos.orden,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Criterio {
            id: registro.id,
            nombre: registro.nombre,
            descripcion: registro.descripcion,
            competencia_id: registro.competencia_id,
            orden: registro.orden,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar criterio: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar el criterio")
        }
    }
}

#[delete("/criterios/{id}")]
pub async fn eliminar_criterio(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM criterios WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Criterio eliminado correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar criterio: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar el criterio")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/criterios")
            .service(listar_criterios)
            .service(crear_criterio)
            .service(editar_criterio)
            .service(eliminar_criterio)
    );
}