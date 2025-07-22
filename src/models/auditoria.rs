use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Auditoria {
    pub id: Uuid,
    pub tabla_afectada: String,
    pub accion: String,
    pub id_afectado: Uuid,
    pub realizado_en: NaiveDateTime,
    pub detalles: Option<Value>, // JSONB puede ser null
}