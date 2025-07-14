use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bimestre {
    pub id: Uuid,
    pub nombre: String, // "I", "II", "III", "IV"
}