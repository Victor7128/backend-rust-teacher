use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Grado {
    pub id: Uuid,
    pub numero: i32, // 1, 2, 3, 4, 5
    pub bimestre_id: Uuid, // Relación con bimestre
}