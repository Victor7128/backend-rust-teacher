use actix_web::{get, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::bimestre::Bimestre;

#[get("/bimestres")]
pub async fn listar_bimestres(pool: web::Data<PgPool>) -> impl Responder {
    let bimestres = sqlx::query_as!(
        Bimestre,
        r#"SELECT id, nombre, activo FROM bimestres ORDER BY nombre"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match bimestres {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar bimestres: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar bimestres")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EstadoBimestre {
    pub activo: bool,
}

#[put("/bimestres/{id}/estado")]
pub async fn cambiar_estado_bimestre(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    estado: web::Json<EstadoBimestre>
) -> impl Responder {
    let id = path.into_inner();
    let activo = estado.activo;

    let resultado = sqlx::query!(
        "UPDATE bimestres SET activo = $1 WHERE id = $2",
        activo,
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error al cambiar estado de bimestre: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo cambiar el estado")
        }
    }
}

#[delete("/bimestres/{id}/limpiar")]
pub async fn limpiar_bimestre(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let bimestre_id = path.into_inner();

    let resultado = sqlx::query!(
        "SELECT limpiar_bimestre($1)",
        bimestre_id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Datos asociados al bimestre eliminados exitosamente"),
        Err(e) => {
            eprintln!("Error al limpiar bimestre: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo limpiar el bimestre")
        }
    }
}