use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct FilaExportacion {
    pub alumno_id: Uuid,
    pub alumno_nombre: String,
    pub seccion_id: Uuid,
    pub seccion_letra: String,
    pub bimestre_id: Uuid,
    pub bimestre_nombre: String,
    pub sesion_id: Option<Uuid>,
    pub sesion_nombre: Option<String>,
    pub sesion_orden: Option<i16>,  // Nuevo campo añadido
    pub competencia_id: Option<Uuid>,
    pub competencia_nombre: Option<String>,
    pub competencia_orden: Option<i16>,  // Nuevo campo añadido
    pub criterio_id: Option<Uuid>,
    pub criterio_nombre: Option<String>,
    pub criterio_orden: Option<i16>,  // Nuevo campo añadido
    pub nota: Option<String>,
    pub observacion: Option<String>,
}