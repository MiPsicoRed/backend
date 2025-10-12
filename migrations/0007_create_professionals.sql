CREATE TABLE professionals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,  -- this can't be null, since we can't have professionals that are not users 
    gender_id INTEGER NOT NULL,
    birthdate DATE NOT NULL,
    liecense_number VARCHAR(100),
    bio TEXT,
    education TEXT,
    experience_years INTEGER,
    hourly_rate DECIMAL(10,2),
    accepts_insurance BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

