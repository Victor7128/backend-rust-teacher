use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use validator::ValidationError;

// Usuario (profesor)
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Usuario {
    pub id: Uuid,
    #[validate(email)]
    pub email: String,
    pub nombre: String,
    pub google_id: String,
}

// Bimestre
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Bimestre {
    pub id: Uuid,
    pub nombre: String,  // I, II, III, IV
}

// Grado
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Grado {
    pub id: Uuid,
    #[validate(range(min = 1, max = 12))]
    pub numero: i16,
}

// Sección
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Seccion {
    pub id: Uuid,
    #[validate(length(min = 1, max = 1))]
    pub letra: String,
    pub grado_id: Uuid,
}

// Alumno
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Alumno {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub nombre: String,
    pub seccion_id: Uuid,
}

// Sesión
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sesion {
    pub id: Uuid,
    pub nombre: String,
    pub seccion_id: Uuid,
    pub orden: i16,
}

// Competencia
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Competencia {
    pub id: Uuid,
    #[validate(length(max = 100))]
    pub nombre: Option<String>,
    pub sesion_id: Uuid,
}

// Criterio
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Criterio {
    pub id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub nombre: String,
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
}

// Evaluación
#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Evaluacion {
    pub id: Uuid,
    pub estudiante_id: Uuid,
    pub criterio_id: Uuid,
    #[validate(length(min = 2, max = 2))]
    #[validate(regex(path = "VALIDACION_EVALUACION"))]
    pub valor: String,
}

// Validación personalizada para valores de evaluación
static VALIDACION_EVALUACION: once_cell::sync::Lazy<regex::Regex> = 
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^(AD|A|B|C)$").unwrap());

// Relación profesor-sección
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProfesorSeccion {
    pub profesor_id: Uuid,
    pub seccion_id: Uuid,
}

// Estructuras para creación de entidades (sin ID)
#[derive(Debug, Deserialize, Validate)]
pub struct NuevoAlumno {
    #[validate(length(min = 3))]
    pub nombre: String,
    pub seccion_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NuevaSeccion {
    #[validate(length(min = 1, max = 1))]
    pub letra: String,
    pub grado_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NuevaCompetencia {
    #[validate(length(max = 100))]
    pub nombre: Option<String>,
    pub sesion_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NuevoCriterio {
    #[validate(length(min = 1, max = 50))]
    pub nombre: String,
    pub descripcion: Option<String>,
    pub competencia_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NuevaEvaluacion {
    pub estudiante_id: Uuid,
    pub criterio_id: Uuid,
    #[validate(regex(path = "VALIDACION_EVALUACION"))]
    pub valor: String,
}

impl From<NuevoAlumno> for Alumno {
    fn from(item: NuevoAlumno) -> Self {
        Alumno {
            id: Uuid::new_v4(),
            nombre: item.nombre,
            seccion_id: item.seccion_id,
        }
    }
}

impl From<NuevaSeccion> for Seccion {
    fn from(item: NuevaSeccion) -> Self {
        Seccion {
            id: Uuid::new_v4(),
            letra: item.letra,
            grado_id: item.grado_id,
        }
    }
}

impl From<NuevoGrado> for Grado {
    fn from(item: NuevoGrado) -> Self {
        Grado {
            id: Uuid::new_v4(),
            numero: item.numero,
        }
    }
}

impl From<NuevaCompetencia> for Competencia {
    fn from(item: NuevaCompetencia) -> Self {
        Competencia {
            id: Uuid::new_v4(),
            nombre: item.nombre,
            sesion_id: item.sesion_id,
        }
    }
}

impl From<NuevoCriterio> for Criterio {
    fn from(item: NuevoCriterio) -> Self {
        Criterio {
            id: Uuid::new_v4(),
            nombre: item.nombre,
            descripcion: item.descripcion,
            competencia_id: item.competencia_id,
        }
    }
}

impl From<NuevaEvaluacion> for Evaluacion {
    fn from(item: NuevaEvaluacion) -> Self {
        Evaluacion {
            id: Uuid::new_v4(),
            estudiante_id: item.estudiante_id,
            criterio_id: item.criterio_id,
            valor: item.valor,
        }
    }
}

// Implementación especial para creación automática de sesiones
impl Sesion {
    pub fn nueva(seccion_id: Uuid, orden: i16) -> Self {
        Sesion {
            id: Uuid::new_v4(),
            nombre: String::new(), // La BD generará el nombre automático
            seccion_id,
            orden,
        }
    }
}

// Implementación para creación de usuarios
impl Usuario {
    pub fn from_google(
        email: &str, 
        nombre: &str, 
        google_id: &str
    ) -> Self {
        Usuario {
            id: Uuid::new_v4(),
            email: email.to_string(),
            nombre: nombre.to_string(),
            google_id: google_id.to_string(),
        }
    }
}

// Implementación para crear relaciones profesor-sección
impl ProfesorSeccion {
    pub fn nueva(profesor_id: Uuid, seccion_id: Uuid) -> Self {
        ProfesorSeccion {
            profesor_id,
            seccion_id,
        }
    }
}

// Para manejar errores de validación en respuestas JSON
impl From<ValidationError> for actix_web::Error {
    fn from(error: ValidationError) -> Self {
        actix_web::error::ErrorBadRequest(error.to_string())
    }
}

// Para respuestas de error unificadas
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<Vec<String>>,
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(errors: validator::ValidationErrors) -> Self {
        let details = errors
            .field_errors()
            .values()
            .flat_map(|errors| errors.iter().map(|e| e.to_string()))
            .collect::<Vec<_>>();

        ErrorResponse {
            error: "Validation failed".to_string(),
            details: Some(details),
        }
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(error: sqlx::Error) -> Self {
        ErrorResponse {
            error: "Database error".to_string(),
            details: Some(vec![error.to_string()]),
        }
    }
}