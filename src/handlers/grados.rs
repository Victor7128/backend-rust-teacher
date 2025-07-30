use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::grado::Grado;

#[get("/grados")]
pub async fn listar_grados(pool: web::Data<PgPool>) -> impl Responder {
    let grados = sqlx::query_as!(
        Grado,
        r#"SELECT numero, creado_en FROM grados ORDER BY numero"#
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
}

#[post("/grados")]
pub async fn crear_grado(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevoGrado>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO grados (numero) VALUES ($1) RETURNING numero, creado_en",
        datos.numero
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Grado {
            numero: registro.numero,
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
}

#[put("/grados/{numero}")]
pub async fn editar_grado(
    pool: web::Data<PgPool>,
    path: web::Path<i16>,
    datos: web::Json<EditarGrado>
) -> impl Responder {
    let numero_actual = path.into_inner();
    let nuevo_numero = datos.numero;

    let resultado = sqlx::query!(
        "UPDATE grados SET numero = $1 WHERE numero = $2 RETURNING numero, creado_en",
        nuevo_numero,
        numero_actual
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Grado {
            numero: registro.numero,
            creado_en: registro.creado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar grado: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar el grado")
        }
    }
}

#[delete("/grados/{numero}")]
pub async fn eliminar_grado(
    pool: web::Data<PgPool>,
    path: web::Path<i16>
) -> impl Responder {
    let numero = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM grados WHERE numero = $1",
        numero
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/grados")
            .service(listar_grados)
            .service(crear_grado)
            .service(editar_grado)
            .service(eliminar_grado)
    );
}