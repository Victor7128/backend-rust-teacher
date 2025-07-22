use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Alumno {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid,
    pub creado_en: NaiveDateTime,
    pub actualizado_en: NaiveDateTime,
}