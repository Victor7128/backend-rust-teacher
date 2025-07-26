-- Tabla de bimestres (predefinidos)
CREATE TABLE bimestres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(2) NOT NULL CHECK (nombre IN ('I', 'II', 'III', 'IV')),
    activo BOOLEAN NOT NULL DEFAULT true
);

-- Insertar bimestres predeterminados
INSERT INTO bimestres (nombre) VALUES 
('I'), ('II'), ('III'), ('IV');

-- Tabla de grados (INDEPENDIENTE de bimestres)
CREATE TABLE grados (
    numero SMALLINT PRIMARY KEY,  -- Clave primaria natural
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Tabla de secciones (con relación CORREGIDA)
CREATE TABLE secciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    letra CHAR(1) NOT NULL,
    grado_numero SMALLINT NOT NULL REFERENCES grados(numero) ON DELETE CASCADE,
    bimestre_id UUID NOT NULL REFERENCES bimestres(id) ON DELETE CASCADE,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (grado_numero, bimestre_id, letra)  -- Sección única por grado/bimestre
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

-- SECUENCIAS para nombres automáticos (EVITAN problemas de concurrencia)
CREATE SEQUENCE sesion_orden_seq;
CREATE SEQUENCE competencia_orden_seq;
CREATE SEQUENCE criterio_orden_seq;

-- Tabla de sesiones (con orden y bimestre)
CREATE TABLE sesiones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,
    seccion_id UUID NOT NULL REFERENCES secciones(id) ON DELETE CASCADE,
    bimestre_id UUID NOT NULL REFERENCES bimestres(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL DEFAULT nextval('sesion_orden_seq'),
    fecha DATE NOT NULL DEFAULT CURRENT_DATE,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (seccion_id, bimestre_id, orden)
);

-- Función para nombre automático de sesiones (SEGURO para concurrencia)
CREATE OR REPLACE FUNCTION generar_nombre_sesion()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'Sesión ' || NEW.orden::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para sesiones
CREATE TRIGGER trg_nombre_sesion
BEFORE INSERT ON sesiones
FOR EACH ROW
EXECUTE FUNCTION generar_nombre_sesion();

-- Tabla de competencias (con orden y descripción opcional)
CREATE TABLE competencias (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    sesion_id UUID NOT NULL REFERENCES sesiones(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL DEFAULT nextval('competencia_orden_seq'),
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Función para nombre automático de competencias
CREATE OR REPLACE FUNCTION generar_nombre_competencia()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'Competencia ' || NEW.orden::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para competencias
CREATE TRIGGER trg_nombre_competencia
BEFORE INSERT ON competencias
FOR EACH ROW
EXECUTE FUNCTION generar_nombre_competencia();

-- Tabla de criterios (con orden y descripción opcional)
CREATE TABLE criterios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(50) NOT NULL,
    descripcion TEXT,
    competencia_id UUID NOT NULL REFERENCES competencias(id) ON DELETE CASCADE,
    orden SMALLINT NOT NULL DEFAULT nextval('criterio_orden_seq'),
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Función para nombre automático de criterios
CREATE OR REPLACE FUNCTION generar_nombre_criterio()
RETURNS TRIGGER AS $$
BEGIN
    NEW.nombre = 'C' || NEW.orden::TEXT;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para criterios
CREATE TRIGGER trg_nombre_criterio
BEFORE INSERT ON criterios
FOR EACH ROW
EXECUTE FUNCTION generar_nombre_criterio();

-- Tabla NORMALIZADA de calificaciones
CREATE TABLE calificaciones (
    codigo CHAR(2) PRIMARY KEY,
    descripcion TEXT NOT NULL
);

INSERT INTO calificaciones (codigo, descripcion) VALUES
('AD', 'Logro destacado'),
('A', 'Logro esperado'),
('B', 'En proceso'),
('C', 'En inicio');

-- Tabla de evaluaciones (con calificación normalizada)
CREATE TABLE evaluaciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    estudiante_id UUID NOT NULL REFERENCES alumnos(id) ON DELETE CASCADE,
    criterio_id UUID NOT NULL REFERENCES criterios(id) ON DELETE CASCADE,
    calificacion CHAR(2) NOT NULL REFERENCES calificaciones(codigo),
    observacion TEXT,
    creado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    actualizado_en TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (estudiante_id, criterio_id)
);

-- Trigger para evaluaciones
CREATE TRIGGER evaluaciones_actualizado
BEFORE UPDATE ON evaluaciones
FOR EACH ROW EXECUTE FUNCTION actualizar_timestamp();

-- Tabla de auditoría
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

-- Triggers de auditoría para TODAS las tablas críticas
CREATE TRIGGER alumnos_auditoria
AFTER INSERT OR UPDATE OR DELETE ON alumnos
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER evaluaciones_auditoria
AFTER INSERT OR UPDATE OR DELETE ON evaluaciones
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER secciones_auditoria
AFTER INSERT OR UPDATE OR DELETE ON secciones
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER sesiones_auditoria
AFTER INSERT OR UPDATE OR DELETE ON sesiones
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER competencias_auditoria
AFTER INSERT OR UPDATE OR DELETE ON competencias
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

CREATE TRIGGER criterios_auditoria
AFTER INSERT OR UPDATE OR DELETE ON criterios
FOR EACH ROW EXECUTE FUNCTION registrar_auditoria();

-- ÍNDICES para mejorar rendimiento
CREATE INDEX idx_secciones_bimestre ON secciones(bimestre_id);
CREATE INDEX idx_sesiones_bimestre ON sesiones(bimestre_id);
CREATE INDEX idx_sesiones_seccion ON sesiones(seccion_id);
CREATE INDEX idx_competencias_sesion ON competencias(sesion_id);
CREATE INDEX idx_criterios_competencia ON criterios(competencia_id);
CREATE INDEX idx_evaluaciones_estudiante ON evaluaciones(estudiante_id);
CREATE INDEX idx_evaluaciones_criterio ON evaluaciones(criterio_id);