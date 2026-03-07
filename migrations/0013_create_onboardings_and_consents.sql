CREATE TABLE user_onboardings (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    user_type VARCHAR(50) NOT NULL,
    full_name VARCHAR(150),
    phone VARCHAR(30),
    birthdate DATE,
    reason TEXT,
    experience TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_consents (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    is_monoparental BOOLEAN NOT NULL DEFAULT TRUE,
    guardian_name VARCHAR(150),
    guardian_id_document VARCHAR(100),
    signature TEXT,
    guardian2_name VARCHAR(150),
    guardian2_id_document VARCHAR(100),
    signature2 TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
