use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Evaluacion {
    pub id: Uuid,
    pub alumno_id: Uuid,
    pub criterio_id: Uuid,
    pub valor: String, // "AD", "A", "B", "C"
    pub fecha: Option<chrono::NaiveDateTime>,
}