DROP TRIGGER IF EXISTS alumnos_auditoria ON alumnos;
DROP TRIGGER IF EXISTS evaluaciones_auditoria ON evaluaciones;
DROP TRIGGER IF EXISTS secciones_auditoria ON secciones;
DROP TRIGGER IF EXISTS sesiones_auditoria ON sesiones;
DROP TRIGGER IF EXISTS competencias_auditoria ON competencias;
DROP TRIGGER IF EXISTS criterios_auditoria ON criterios;
DROP TRIGGER IF EXISTS alumnos_actualizado ON alumnos;
DROP TRIGGER IF EXISTS evaluaciones_actualizado ON evaluaciones;
DROP TRIGGER IF EXISTS trg_nombre_sesion ON sesiones;
DROP TRIGGER IF EXISTS trg_nombre_competencia ON competencias;
DROP TRIGGER IF EXISTS trg_nombre_criterio ON criterios;

DROP FUNCTION IF EXISTS registrar_auditoria;
DROP FUNCTION IF EXISTS actualizar_timestamp;
DROP FUNCTION IF EXISTS generar_nombre_sesion;
DROP FUNCTION IF EXISTS generar_nombre_competencia;
DROP FUNCTION IF EXISTS generar_nombre_criterio;

DROP SEQUENCE IF EXISTS sesion_orden_seq;
DROP SEQUENCE IF EXISTS competencia_orden_seq;
DROP SEQUENCE IF EXISTS criterio_orden_seq;

DROP TABLE IF EXISTS auditoria;
DROP TABLE IF EXISTS evaluaciones;
DROP TABLE IF EXISTS calificaciones;  -- Nueva tabla
DROP TABLE IF EXISTS criterios;
DROP TABLE IF EXISTS competencias;
DROP TABLE IF EXISTS sesiones;
DROP TABLE IF EXISTS alumnos;
DROP TABLE IF EXISTS secciones;
DROP TABLE IF EXISTS grados;
DROP TABLE IF EXISTS bimestres;