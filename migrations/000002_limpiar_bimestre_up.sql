-- Función MEJORADA de limpieza (no borra alumnos permanentemente)
CREATE OR REPLACE FUNCTION limpiar_bimestre(bimestre_uuid UUID)
RETURNS VOID AS $$
BEGIN
    -- Desasociar alumnos del bimestre (no borrarlos)
    UPDATE alumnos SET seccion_id = NULL
    WHERE seccion_id IN (
        SELECT id FROM secciones WHERE bimestre_id = bimestre_uuid
    );

    -- Eliminar datos académicos en orden inverso
    DELETE FROM evaluaciones WHERE criterio_id IN (
        SELECT id FROM criterios WHERE competencia_id IN (
            SELECT id FROM competencias WHERE sesion_id IN (
                SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
            )
        )
    );

    DELETE FROM criterios WHERE competencia_id IN (
        SELECT id FROM competencias WHERE sesion_id IN (
            SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
        )
    );

    DELETE FROM competencias WHERE sesion_id IN (
        SELECT id FROM sesiones WHERE bimestre_id = bimestre_uuid
    );

    DELETE FROM sesiones WHERE bimestre_id = bimestre_uuid;
    DELETE FROM secciones WHERE bimestre_id = bimestre_uuid;

    -- Reiniciar secuencias para el próximo bimestre
    PERFORM setval('sesion_orden_seq', 1, false);
    PERFORM setval('competencia_orden_seq', 1, false);
    PERFORM setval('criterio_orden_seq', 1, false);
END;
$$ LANGUAGE plpgsql;