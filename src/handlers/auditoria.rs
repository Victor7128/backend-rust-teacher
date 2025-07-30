use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/auditoria")]
pub async fn listar_auditoria(pool: web::Data<PgPool>) -> impl Responder {
    let auditorias = sqlx::query_as!(
        Auditoria,
        r#"SELECT id, tabla_afectada, accion, id_afectado, realizado_en, detalles
           FROM auditoria
           ORDER BY realizado_en DESC"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match auditorias {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar auditorías: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar auditorías")
        }
    }
}

#[get("/auditoria/{id}")]
pub async fn obtener_auditoria(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let auditoria = sqlx::query_as!(
        Auditoria,
        r#"SELECT id, tabla_afectada, accion, id_afectado, realizado_en, detalles
           FROM auditoria
           WHERE id = $1"#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match auditoria {
        Ok(Some(registro)) => HttpResponse::Ok().json(registro),
        Ok(None) => HttpResponse::NotFound().body("Registro de auditoría no encontrado"),
        Err(e) => {
            eprintln!("Error al obtener auditoría: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener auditoría")
        }
    }
}

// Ejemplo de filtro por tabla afectada y/o acción:
#[get("/auditoria/filtro")]
pub async fn filtrar_auditoria(
    _pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let tabla = query.get("tabla_afectada");
    let accion = query.get("accion");

    let mut sql = String::from("SELECT id, tabla_afectada, accion, id_afectado, realizado_en, detalles FROM auditoria WHERE 1=1");
    let mut params: Vec<String> = Vec::new();

    if let Some(tabla) = tabla {
        sql.push_str(" AND tabla_afectada = $1");
        params.push(tabla.clone());
    }
    if let Some(accion) = accion {
        sql.push_str(&format!(" AND accion = ${}", params.len() + 1));
        params.push(accion.clone());
    }
    sql.push_str(" ORDER BY realizado_en DESC");

    HttpResponse::NotImplemented().body("Este endpoint es un ejemplo, implementa la lógica de parámetros según tus necesidades.")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auditoria")
            .service(listar_auditoria)
            .service(obtener_auditoria)
            .service(filtrar_auditoria)
    );
}