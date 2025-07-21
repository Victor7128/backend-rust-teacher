use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::grado::Grado;

#[get("/grados")]
pub async fn listar_grados(pool: web::Data<PgPool>) -> impl Responder {
    let grados = sqlx::query_as!(
        Grado,
        r#"SELECT id, numero, bimestre_id, creado_en FROM grados ORDER BY numero"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match grados {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar grados: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar grados")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevoGrado {
    pub numero: i16,
    pub bimestre_id: Uuid,
}

#[post("/grados")]
pub async fn crear_grado(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevoGrado>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO grados (id, numero, bimestre_id) VALUES (gen_random_uuid(), $1, $2) RETURNING id, numero, bimestre_id, creado_en",
        datos.numero,
        datos.bimestre_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Grado {
            id: registro.id,
            numero: registro.numero,
            bimestre_id: registro.bimestre_id,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear grado: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear el grado")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarGrado {
    pub numero: i16,
    pub bimestre_id: Uuid,
}

#[put("/grados/{id}")]
pub async fn editar_grado(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarGrado>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE grados SET numero = $1, bimestre_id = $2 WHERE id = $3 RETURNING id, numero, bimestre_id, creado_en",
        datos.numero,
        datos.bimestre_id,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Grado {
            id: registro.id,
            numero: registro.numero,
            bimestre_id: registro.bimestre_id,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar grado: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar el grado")
        }
    }
}

#[delete("/grados/{id}")]
pub async fn eliminar_grado(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM grados WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Grado eliminado correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar grado: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar el grado")
        }
    }
}