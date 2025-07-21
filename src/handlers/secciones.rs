use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::seccion::Seccion;

#[get("/secciones")]
pub async fn listar_secciones(pool: web::Data<PgPool>) -> impl Responder {
    let secciones = sqlx::query_as!(
        Seccion,
        r#"SELECT id, letra, grado_id, bimestre_id, creado_en FROM secciones ORDER BY letra"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match secciones {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar secciones: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar secciones")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevaSeccion {
    pub letra: String,
    pub grado_id: Uuid,
    pub bimestre_id: Uuid,
}

#[post("/secciones")]
pub async fn crear_seccion(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevaSeccion>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO secciones (id, letra, grado_id, bimestre_id) VALUES (gen_random_uuid(), $1, $2, $3) RETURNING id, letra, grado_id, bimestre_id, creado_en",
        datos.letra,
        datos.grado_id,
        datos.bimestre_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Seccion {
            id: registro.id,
            letra: registro.letra,
            grado_id: registro.grado_id,
            bimestre_id: registro.bimestre_id,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear sección: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear la sección")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarSeccion {
    pub letra: String,
    pub grado_id: Uuid,
    pub bimestre_id: Uuid,
}

#[put("/secciones/{id}")]
pub async fn editar_seccion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarSeccion>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE secciones SET letra = $1, grado_id = $2, bimestre_id = $3 WHERE id = $4 RETURNING id, letra, grado_id, bimestre_id, creado_en",
        datos.letra,
        datos.grado_id,
        datos.bimestre_id,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Seccion {
            id: registro.id,
            letra: registro.letra,
            grado_id: registro.grado_id,
            bimestre_id: registro.bimestre_id,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar sección: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar la sección")
        }
    }
}

#[delete("/secciones/{id}")]
pub async fn eliminar_seccion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM secciones WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Sección eliminada correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar sección: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar la sección")
        }
    }
}