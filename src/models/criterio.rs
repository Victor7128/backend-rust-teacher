use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Criterio {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
}