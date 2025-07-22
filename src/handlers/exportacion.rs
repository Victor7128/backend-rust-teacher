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

    // Consulta SQL
    let filas = sqlx::query_as!(
        FilaExportacion,
        r#"
        SELECT
          a.id as alumno_id,
          a.nombre as alumno_nombre,
          s.id as seccion_id,
          s.letra as seccion_letra,
          b.id as bimestre_id,
          b.nombre as bimestre_nombre,
          ses.id as sesion_id,
          ses.nombre as sesion_nombre,
          c.id as competencia_id,
          c.nombre as competencia_nombre,
          cr.id as criterio_id,
          cr.nombre as criterio_nombre,
          ev.valor as nota,
          ev.observacion as observacion
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

    // Crear el Excel
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