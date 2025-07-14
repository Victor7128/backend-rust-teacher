-- Crear tabla de bimestres
CREATE TABLE bimestres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(10) NOT NULL  -- "I", "II", "III", "IV"
);

-- Crear tabla de grados
CREATE TABLE grados (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    numero INTEGER NOT NULL,     -- 1, 2, 3, 4, 5
    bimestre_id UUID NOT NULL REFERENCES bimestres(id) ON DELETE CASCADE
);

-- Crear tabla de secciones
CREATE TABLE secciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    letra VARCHAR(5) NOT NULL,   -- "A", "B", ...
    grado_id UUID NOT NULL REFERENCES grados(id) ON DELETE CASCADE
);

-- Crear tabla de alumnos
CREATE TABLE alumnos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,
    seccion_id UUID NOT NULL REFERENCES secciones(id) ON DELETE CASCADE
);

-- Crear tabla de sesiones
CREATE TABLE sesiones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,     -- "Sesión 1", etc.
    seccion_id UUID NOT NULL REFERENCES secciones(id) ON DELETE CASCADE,
    fecha TIMESTAMP NULL
);

-- Crear tabla de competencias
CREATE TABLE competencias (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,    -- "Competencia 1", etc.
    sesion_id UUID NOT NULL REFERENCES sesiones(id) ON DELETE CASCADE
);

-- Crear tabla de criterios
CREATE TABLE criterios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,     -- "C1", "C2", etc.
    descripcion TEXT,
    competencia_id UUID NOT NULL REFERENCES competencias(id) ON DELETE CASCADE
);

-- Crear tabla de evaluaciones
CREATE TABLE evaluaciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alumno_id UUID NOT NULL REFERENCES alumnos(id) ON DELETE CASCADE,
    criterio_id UUID NOT NULL REFERENCES criterios(id) ON DELETE CASCADE,
    valor VARCHAR(4) NOT NULL,       -- "AD", "A", "B", "C"
    fecha TIMESTAMP NULL,
    UNIQUE (alumno_id, criterio_id)  -- No se puede evaluar dos veces el mismo criterio para el mismo alumno
);