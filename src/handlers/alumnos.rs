use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::alumno::Alumno;

#[get("/alumnos")]
pub async fn listar_alumnos(pool: web::Data<PgPool>) -> impl Responder {
    let alumnos = sqlx::query_as!(
        Alumno,
        r#"SELECT id, nombre, seccion_id, creado_en, actualizado_en FROM alumnos ORDER BY nombre"#
    )
    .fetch_all(pool.get_ref())
    .await;

    match alumnos {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(e) => {
            eprintln!("Error al listar alumnos: {:?}", e);
            HttpResponse::InternalServerError().body("Error al listar alumnos")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevoAlumno {
    pub nombre: String,
    pub seccion_id: Option<Uuid>,  // Ahora es opcional
}

#[post("/alumnos")]
pub async fn crear_alumno(
    pool: web::Data<PgPool>,
    datos: web::Json<NuevoAlumno>
) -> impl Responder {
    let resultado = sqlx::query!(
        "INSERT INTO alumnos (id, nombre, seccion_id) 
        VALUES (gen_random_uuid(), $1, $2) 
        RETURNING id, nombre, seccion_id, creado_en, actualizado_en",
        datos.nombre,
        datos.seccion_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Created().json(Alumno {
            id: registro.id,
            nombre: registro.nombre,
            seccion_id: registro.seccion_id,
            creado_en: registro.creado_en,
            actualizado_en: registro.actualizado_en,
        }),
        Err(e) => {
            eprintln!("Error al crear alumno: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo crear el alumno")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EditarAlumno {
    pub nombre: String,
    pub seccion_id: Option<Uuid>,  // Ahora es opcional
}

#[put("/alumnos/{id}")]
pub async fn editar_alumno(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    datos: web::Json<EditarAlumno>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "UPDATE alumnos SET nombre = $1, seccion_id = $2 
        WHERE id = $3 
        RETURNING id, nombre, seccion_id, creado_en, actualizado_en",
        datos.nombre,
        datos.seccion_id,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match resultado {
        Ok(registro) => HttpResponse::Ok().json(Alumno {
            id: registro.id,
            nombre: registro.nombre,
            seccion_id: registro.seccion_id,
            creado_en: registro.creado_en,
            actualizado_en: registro.actualizado_en,
        }),
        Err(e) => {
            eprintln!("Error al editar alumno: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo editar el alumno")
        }
    }
}

#[delete("/alumnos/{id}")]
pub async fn eliminar_alumno(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner();
    let resultado = sqlx::query!(
        "DELETE FROM alumnos WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Alumno eliminado correctamente"),
        Err(e) => {
            eprintln!("Error al eliminar alumno: {:?}", e);
            HttpResponse::InternalServerError().body("No se pudo eliminar el alumno")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/alumnos")
            .service(listar_alumnos)
            .service(crear_alumno)
            .service(editar_alumno)
            .service(eliminar_alumno)
    );
}