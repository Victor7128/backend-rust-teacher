-- Eliminar todos los datos relacionados a un bimestre, pero no el bimestre en sí
CREATE OR REPLACE FUNCTION limpiar_bimestre(bimestre_uuid UUID)
RETURNS VOID AS $$
BEGIN
    -- Eliminar evaluaciones asociadas
    DELETE FROM evaluaciones WHERE criterio_id IN (
        SELECT id FROM criterios WHERE competencia_id IN (
            SELECT id FROM competencias WHERE sesion_id IN (
                SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
            )
        )
    );

    -- Eliminar criterios
    DELETE FROM criterios WHERE competencia_id IN (
        SELECT id FROM competencias WHERE sesion_id IN (
            SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
        )
    );

    -- Eliminar competencias
    DELETE FROM competencias WHERE sesion_id IN (
        SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
    );

    -- Eliminar alumnos (por secciones relacionadas al bimestre)
    DELETE FROM alumnos WHERE seccion_id IN (
        SELECT id FROM secciones WHERE bimestre_id = bimestre_uuid
    );

    -- Eliminar sesiones
    DELETE FROM sesiones WHERE bimestre_id = bimestre_uuid;

    -- Eliminar secciones
    DELETE FROM secciones WHERE bimestre_id = bimestre_uuid;

    -- El bimestre permanece intacto
END;
$$ LANGUAGE plpgsql;