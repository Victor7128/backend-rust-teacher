use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Criterio {
    pub id: Uuid,
    pub nombre: String, // "C1", "C2", etc.
    pub descripcion: Option<String>,
    pub competencia_id: Uuid, // Relación con competencia
}