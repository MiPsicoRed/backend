-- Create parent_consents table
CREATE TABLE IF NOT EXISTS parent_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES patients(id) ON DELETE CASCADE,
    guardian_name TEXT NOT NULL,
    guardian_id_document TEXT NOT NULL,
    signature_data TEXT NOT NULL, -- Base64 encoded signature
    consent_certificate_id TEXT NOT NULL, -- SHA-256 hash or unique certificate ID
    signed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add index for patient_id
CREATE INDEX idx_parent_consents_patient_id ON parent_consents(patient_id);
