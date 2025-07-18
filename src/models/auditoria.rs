use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Auditoria {
    pub id: Uuid,
    pub accion: String,
    pub entidad: String,
    pub entidad_id: Uuid,
    pub fecha: NaiveDateTime,
    pub descripcion: Option<String>,
}