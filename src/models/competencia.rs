use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Competencia {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub sesion_id: Uuid,
}