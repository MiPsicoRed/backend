CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    patient_id UUID NOT NULL REFERENCES patients(id) ON DELETE CASCADE, 
    professional_id UUID  NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_type_id UUID REFERENCES session_types(id) ON DELETE SET NULL, 
    session_status_id INTEGER NOT NULL,
    session_date TIMESTAMP NOT NULL,
    videocall_url TEXT,
    notes TEXT,
    completed BOOLEAN DEFAULT FALSE,
    session_duration INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);