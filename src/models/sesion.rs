use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime; // Para fechas

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sesion {
    pub id: Uuid,
    pub nombre: String, // "Sesión 1", "Sesión 2", ...
    pub seccion_id: Uuid, // Relación con sección
    pub fecha: Option<NaiveDateTime>, // Opcional
}