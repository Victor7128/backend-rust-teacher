use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Seccion {
    pub id: Uuid,
    pub nombre: String,
    pub grado_id: Uuid,
}