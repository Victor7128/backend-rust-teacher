use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Seccion {
    pub id: Uuid,
    pub letra: String, // "A", "B", ...
    pub grado_id: Uuid, // Relación con grado
}