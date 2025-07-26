use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Seccion {
    pub id: Uuid,
    pub letra: String,
    pub grado_numero: i16,
    pub bimestre_id: Uuid,
    pub creado_en: NaiveDateTime,
}