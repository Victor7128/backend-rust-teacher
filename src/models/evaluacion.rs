use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Evaluacion {
    pub id: Uuid,
    pub sesion_id: Uuid,
    pub alumno_id: Uuid,
    pub criterio_id: Uuid,
    pub valor: String,
    pub observacion: Option<String>,
}