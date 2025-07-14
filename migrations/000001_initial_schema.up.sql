-- Tabla de usuarios (profesores)
CREATE TABLE usuarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    nombre VARCHAR(100) NOT NULL,
    google_id VARCHAR(100) UNIQUE NOT NULL
);

-- Tabla de bimestres
CREATE TABLE bimestres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(2) NOT NULL CHECK (nombre IN ('I', 'II', 'III', 'IV'))
);

-- Insertar bimestres predeterminados
INSERT INTO bimestres (nombre) VALUES ('I'), ('II'), ('III'), ('IV');

-- Tabla de grados
CREATE TABLE grados (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    numero SMALLINT NOT NULL UNIQUE
);

-- Tabla de secciones
CREATE TABLE secciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    letra CHAR(1) NOT NULL,
    grado_id UUID REFERENCES grados(id) ON DELETE CASCADE,
    UNIQUE (grado_id, letra)
);

-- Tabla de alumnos
CREATE TABLE alumnos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,
    seccion_id UUID REFERENCES secciones(id) ON DELETE CASCADE
);

-- Tabla de sesiones
CREATE TABLE sesiones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL DEFAULT 'Sesión',
    seccion_id UUID REFERENCES secciones(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL
);

-- Tabla de competencias
CREATE TABLE competencias (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100),
    sesion_id UUID REFERENCES sesiones(id) ON DELETE CASCADE
);

-- Tabla de criterios
CREATE TABLE criterios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL DEFAULT 'C',
    descripcion TEXT,
    competencia_id UUID REFERENCES competencias(id) ON DELETE CASCADE
);

-- Tabla de evaluaciones (notas)
CREATE TABLE evaluaciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    estudiante_id UUID REFERENCES alumnos(id) ON DELETE CASCADE,
    criterio_id UUID REFERENCES criterios(id) ON DELETE CASCADE,
    valor CHAR(2) NOT NULL CHECK (valor IN ('AD', 'A', 'B', 'C')),
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (estudiante_id, criterio_id)  -- Previene reevaluaciones
);

-- Tabla de relación profesor-sección
CREATE TABLE profesor_secciones (
    profesor_id UUID REFERENCES usuarios(id) ON DELETE CASCADE,
    seccion_id UUID REFERENCES secciones(id) ON DELETE CASCADE,
    PRIMARY KEY (profesor_id, seccion_id)
);

-- Función para generar nombres automáticos de sesiones
CREATE OR REPLACE FUNCTION generar_nombre_sesion()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre := 'Sesión ' || (
        SELECT COUNT(*) + 1 
        FROM sesiones 
        WHERE seccion_id = NEW.seccion_id
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para nombres de sesión
CREATE TRIGGER trg_nombre_sesion
BEFORE INSERT ON sesiones
FOR EACH ROW
EXECUTE FUNCTION generar_nombre_sesion();