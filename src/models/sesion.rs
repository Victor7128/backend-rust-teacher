use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sesion {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid,
    pub bimestre_id: Uuid,
    pub orden: i16,
    pub fecha: NaiveDate,
    pub creado_en: NaiveDateTime,
}