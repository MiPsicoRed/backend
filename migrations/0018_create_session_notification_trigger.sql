CREATE OR REPLACE FUNCTION create_session_notifications()
RETURNS TRIGGER AS $$
DECLARE
    patient_user_uuid UUID;
    professional_user_uuid UUID;
BEGIN
    -- Get patient's user_id
    SELECT user_id INTO patient_user_uuid FROM patients WHERE id = NEW.patient_id;
    -- Get professional's user_id
    SELECT user_id INTO professional_user_uuid FROM professionals WHERE id = NEW.professional_id;

    -- Create notification for patient if they have a user_id
    IF patient_user_uuid IS NOT NULL THEN
        INSERT INTO notifications (user_id, type, title, message, is_read, created_at)
        VALUES (patient_user_uuid, 'info', 'Nueva sesión programada', 'Se ha programado una nueva sesión con tu profesional.', FALSE, CURRENT_TIMESTAMP);
    END IF;

    -- Create notification for professional if they have a user_id
    IF professional_user_uuid IS NOT NULL THEN
        INSERT INTO notifications (user_id, type, title, message, is_read, created_at)
        VALUES (professional_user_uuid, 'info', 'Nueva sesión programada', 'Se ha programado una nueva sesión con tu paciente.', FALSE, CURRENT_TIMESTAMP);
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_create_session_notifications
AFTER INSERT ON sessions
FOR EACH ROW
EXECUTE FUNCTION create_session_notifications();
