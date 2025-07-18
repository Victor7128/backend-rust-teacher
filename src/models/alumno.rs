use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Alumno {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid
}