use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Criterio {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
    pub orden: i16,
    pub creado_en: NaiveDateTime,
}