use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Competencia {
    pub id: Uuid,
    pub nombre: String, // Opcional, puede ser "Comp. 1", etc.
    pub sesion_id: Uuid, // Relación con sesión
}