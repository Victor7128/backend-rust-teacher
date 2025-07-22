use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Evaluacion {
    pub id: Uuid,
    pub estudiante_id: Uuid,
    pub criterio_id: Uuid,
    pub valor: String,
    pub observacion: Option<String>,
    pub creado_en: NaiveDateTime,
    pub actualizado_en: NaiveDateTime,
}