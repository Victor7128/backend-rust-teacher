pub mod alumno;
pub mod bimestre;
pub mod grado;
pub mod seccion;
pub mod sesion;
pub mod competencia;
pub mod criterio;
pub mod evaluacion;

// Reexportar structs directamente para fácil acceso
pub use alumno::Alumno;
pub use bimestre::Bimestre;
pub use grado::Grado;
pub use seccion::Seccion;
pub use sesion::Sesion;
pub use competencia::Competencia;
pub use criterio::Criterio;
pub use evaluacion::Evaluacion;