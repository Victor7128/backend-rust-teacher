use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Calificacion {
    pub codigo: String,        // 'AD', 'A', 'B', 'C'
    pub descripcion: String,   // 'Logro destacado', etc.
}