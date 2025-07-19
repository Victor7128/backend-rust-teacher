-- Tabla de bimestres (predefinidos)
CREATE TABLE bimestres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(2) NOT NULL CHECK (nombre IN ('I', 'II', 'III', 'IV')),
    activo BOOLEAN NOT NULL DEFAULT true
);

-- Insertar bimestres predeterminados
INSERT INTO bimestres (nombre) VALUES 
('I'), ('II'), ('III'), ('IV');

-- Tabla de grados
CREATE TABLE grados (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    numero SMALLINT NOT NULL UNIQUE,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Tabla de secciones (con relación a bimestre)
CREATE TABLE secciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    letra CHAR(1) NOT NULL,
    grado_id UUID NOT NULL REFERENCES grados(id) ON DELETE CASCADE,
    bimestre_id UUID NOT NULL REFERENCES bimestres(id) ON DELETE CASCADE,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (grado_id, bimestre_id, letra)  -- Sección única por grado/bimestre
);

-- Tabla de alumnos (con auditoría)
CREATE TABLE alumnos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,
    seccion_id UUID NOT NULL REFERENCES secciones(id) ON DELETE CASCADE,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    actualizado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Función para actualizar timestamp
CREATE OR REPLACE FUNCTION actualizar_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.actualizado_en = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para alumnos
CREATE TRIGGER alumnos_actualizado
BEFORE UPDATE ON alumnos
FOR EACH ROW EXECUTE FUNCTION actualizar_timestamp();

-- Tabla de sesiones (con orden y bimestre)
CREATE TABLE sesiones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,
    seccion_id UUID NOT NULL REFERENCES secciones(id) ON DELETE CASCADE,
    bimestre_id UUID NOT NULL REFERENCES bimestres(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (seccion_id, bimestre_id, orden)  -- Orden único por sección/bimestre
);

-- Función para nombre automático de sesiones
CREATE OR REPLACE FUNCTION generar_nombre_sesion()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'Sesión ' || (
        SELECT COALESCE(MAX(orden), 0) + 1 
        FROM sesiones 
        WHERE seccion_id = NEW.seccion_id 
        AND bimestre_id = NEW.bimestre_id
    )::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para sesiones
CREATE TRIGGER trg_nombre_sesion
BEFORE INSERT ON sesiones
FOR EACH ROW
WHEN (NEW.nombre IS NULL OR NEW.nombre = '')
EXECUTE FUNCTION generar_nombre_sesion();

-- Tabla de competencias (con orden y descripción opcional)
CREATE TABLE competencias (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT, -- opcional
    sesion_id UUID NOT NULL REFERENCES sesiones(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Función para nombre automático de competencias
CREATE OR REPLACE FUNCTION generar_nombre_competencia()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'Competencia ' || (
        SELECT COALESCE(MAX(orden), 0) + 1 
        FROM competencias 
        WHERE sesion_id = NEW.sesion_id
    )::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para competencias
CREATE TRIGGER trg_nombre_competencia
BEFORE INSERT ON competencias
FOR EACH ROW
WHEN (NEW.nombre IS NULL OR NEW.nombre = '')
EXECUTE FUNCTION generar_nombre_competencia();

-- Tabla de criterios (con orden y descripción opcional)
CREATE TABLE criterios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,
    descripcion TEXT, -- opcional
    competencia_id UUID NOT NULL REFERENCES competencias(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Función para nombre automático de criterios
CREATE OR REPLACE FUNCTION generar_nombre_criterio()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'C' || (
        SELECT COALESCE(MAX(orden), 0) + 1 
        FROM criterios 
        WHERE competencia_id = NEW.competencia_id
    )::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para criterios
CREATE TRIGGER trg_nombre_criterio
BEFORE INSERT ON criterios
FOR EACH ROW
WHEN (NEW.nombre IS NULL OR NEW.nombre = '')
EXECUTE FUNCTION generar_nombre_criterio();

-- Tabla de evaluaciones (único usuario, auditoría y restricción única)
CREATE TABLE evaluaciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    estudiante_id UUID NOT NULL REFERENCES alumnos(id) ON DELETE CASCADE,
    criterio_id UUID NOT NULL REFERENCES criterios(id) ON DELETE CASCADE,
    valor CHAR(2) NOT NULL CHECK (valor IN ('AD', 'A', 'B', 'C')),
    observacion TEXT, -- opcional
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    actualizado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (estudiante_id, criterio_id)  -- Evita evaluación duplicada
);

-- Trigger para evaluaciones
CREATE TRIGGER evaluaciones_actualizado
BEFORE UPDATE ON evaluaciones
FOR EACH ROW EXECUTE FUNCTION actualizar_timestamp();

-- Tabla de auditoría para cambios críticos (sin usuario)
CREATE TABLE auditoria (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tabla_afectada VARCHAR(50) NOT NULL,
    accion VARCHAR(10) NOT NULL CHECK (accion IN ('INSERT', 'UPDATE', 'DELETE')),
    id_afectado UUID NOT NULL,
    realizado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    detalles JSONB
);

-- Función para registrar auditoría
CREATE OR REPLACE FUNCTION registrar_auditoria()
RETURNS TRIGGER AS $$
DECLARE
    detalles_json JSONB;
BEGIN
    IF TG_OP = 'DELETE' THEN
        detalles_json = to_jsonb(OLD);
    ELSE
        detalles_json = to_jsonb(NEW);
    END IF;
    
    INSERT INTO auditoria (tabla_afectada, accion, id_afectado, detalles)
    VALUES (TG_TABLE_NAME, TG_OP, COALESCE(OLD.id, NEW.id), detalles_json);
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers para auditoría en tablas críticas
CREATE TRIGGER alumnos_auditoria
AFTER INSERT OR UPDATE OR DELETE ON alumnos
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER evaluaciones_auditoria
AFTER INSERT OR UPDATE OR DELETE ON evaluaciones
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();
