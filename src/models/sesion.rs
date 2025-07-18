use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sesion {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid,
    pub fecha: NaiveDate,
}