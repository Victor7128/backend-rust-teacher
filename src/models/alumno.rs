use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Alumno {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid, // Relación con sección
}