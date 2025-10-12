CREATE TABLE professionals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,  -- this can't be null, since we can't have professionals that are not users 
    gender_id INTEGER NOT NULL,
    birthdate DATE NOT NULL,
    license_number VARCHAR(100),
    bio TEXT,
    education TEXT,
    experience_years INTEGER,
    hourly_rate REAL,
    accepts_insurance BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

