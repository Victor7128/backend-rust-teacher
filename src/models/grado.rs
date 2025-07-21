use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Grado {
    pub id: Uuid,
    pub numero: i16,
    pub bimestre_id: Uuid,
    pub creado_en: NaiveDateTime,
}