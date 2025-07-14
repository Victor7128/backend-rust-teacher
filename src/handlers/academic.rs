use actix_web::{get, post, web, HttpResponse};
use crate::models::{Grado, Seccion, NuevaSeccion, ErrorResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/grados")]
pub async fn crear_grado(
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ErrorResponse> {
    // Obtener el máximo grado actual
    let max_grado: Option<i16> = sqlx::query_scalar(
        "SELECT MAX(numero) FROM grados"
    )
    .fetch_optional(db.get_ref())
    .await?
    .flatten();
    
    let next_num = max_grado.unwrap_or(0) + 1;
    
    let grado = Grado {
        id: Uuid::new_v4(),
        numero: next_num,
    };
    
    sqlx::query!(
        "INSERT INTO grados (id, numero) VALUES ($1, $2)",
        grado.id,
        grado.numero
    )
    .execute(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(grado))
}

#[post("/secciones")]
pub async fn crear_seccion(
    db: web::Data<PgPool>,
    nueva_seccion: web::Json<NuevaSeccion>,
) -> Result<HttpResponse, ErrorResponse> {
    nueva_seccion.validate()?;
    
    // Obtener la última letra usada para este grado
    let last_letter: Option<String> = sqlx::query_scalar(
        "SELECT MAX(letra) FROM secciones WHERE grado_id = $1"
    )
    .bind(nueva_seccion.grado_id)
    .fetch_optional(db.get_ref())
    .await?;
    
    let next_letter = match last_letter {
        Some(letter) if !letter.is_empty() => {
            let last_char = letter.chars().next().unwrap();
            (last_char as u8 + 1) as char
        }
        _ => 'A',
    };
    
    let seccion = Seccion {
        id: Uuid::new_v4(),
        letra: next_letter.to_string(),
        grado_id: nueva_seccion.grado_id,
    };
    
    sqlx::query!(
        "INSERT INTO secciones (id, letra, grado_id) VALUES ($1, $2, $3)",
        seccion.id,
        seccion.letra,
        seccion.grado_id
    )
    .execute(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().json(seccion))
}

#[get("/grados")]
pub async fn listar_grados(
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ErrorResponse> {
    let grados = sqlx::query_as!(
        Grado,
        "SELECT * FROM grados ORDER BY numero"
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(grados))
}

#[get("/secciones/{grado_id}")]
pub async fn listar_secciones_por_grado(
    db: web::Data<PgPool>,
    grado_id: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorResponse> {
    let secciones = sqlx::query_as!(
        Seccion,
        "SELECT * FROM secciones WHERE grado_id = $1 ORDER BY letra",
        *grado_id
    )
    .fetch_all(db.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(secciones))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crear_grado)
        .service(crear_seccion)
        .service(listar_grados)
        .service(listar_secciones_por_grado);
}