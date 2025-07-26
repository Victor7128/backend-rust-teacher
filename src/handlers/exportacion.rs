use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::exportacion::FilaExportacion;
use umya_spreadsheet::{Workbook, Worksheet};

#[get("/exportar/seccion/{seccion_id}/bimestre/{bimestre_id}")]
pub async fn exportar_notas_excel(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>
) -> impl Responder {
    let (seccion_id, bimestre_id) = path.into_inner();

    // Consulta SQL ACTUALIZADA con nuevas relaciones y campos
    let filas = sqlx::query_as!(
        FilaExportacion,
        r#"
        SELECT
          a.id AS alumno_id,
          a.nombre AS alumno_nombre,
          s.id AS seccion_id,
          s.letra AS seccion_letra,
          b.id AS bimestre_id,
          b.nombre AS bimestre_nombre,
          ses.id AS sesion_id,
          ses.nombre AS sesion_nombre,
          ses.orden AS sesion_orden,  -- Nuevo campo
          c.id AS competencia_id,
          c.nombre AS competencia_nombre,
          c.orden AS competencia_orden,  -- Nuevo campo
          cr.id AS criterio_id,
          cr.nombre AS criterio_nombre,
          cr.orden AS criterio_orden,  -- Nuevo campo
          ev.calificacion AS nota,  -- Cambiado de valor a calificacion
          ev.observacion
        FROM alumnos a
        JOIN secciones s ON a.seccion_id = s.id
        JOIN bimestres b ON s.bimestre_id = b.id
        LEFT JOIN sesiones ses ON ses.seccion_id = s.id AND ses.bimestre_id = b.id
        LEFT JOIN competencias c ON c.sesion_id = ses.id
        LEFT JOIN criterios cr ON cr.competencia_id = c.id
        LEFT JOIN evaluaciones ev ON ev.estudiante_id = a.id AND ev.criterio_id = cr.id
        WHERE s.id = $1 AND b.id = $2
        ORDER BY a.nombre, ses.orden, c.orden, cr.orden
        "#,
        seccion_id,
        bimestre_id
    )
    .fetch_all(pool.get_ref())
    .await;

    let filas = match filas {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al consultar exportación: {:?}", e);
            return HttpResponse::InternalServerError().body("Error al generar exportación");
        }
    };

    // Crear el Excel (sin cambios)
    let mut workbook = Workbook::new();
    let worksheet = workbook.new_sheet("Notas").unwrap();
    
    // Agregar encabezados
    worksheet.append_row(vec![
        "Alumno", "Sección", "Bimestre", "Sesión", "Competencia", "Criterio", "Nota", "Observación"
    ]);

    // Agregar filas
    for fila in filas.iter() {
        worksheet.append_row(vec![
            fila.alumno_nombre.clone(),
            fila.seccion_letra.clone(),
            fila.bimestre_nombre.clone(),
            fila.sesion_nombre.clone().unwrap_or_default(),
            fila.competencia_nombre.clone().unwrap_or_default(),
            fila.criterio_nombre.clone().unwrap_or_default(),
            fila.nota.clone().unwrap_or_default(),
            fila.observacion.clone().unwrap_or_default(),
        ]);
    }

    // Serializar y responder como archivo
    let mut buf = vec![];
    workbook.write(&mut buf).unwrap();

    HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .append_header(("Content-Disposition", "attachment; filename=notas.xlsx"))
        .body(buf)
}