use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Grado {
    pub numero: i16,
    pub creado_en: NaiveDateTime,
}