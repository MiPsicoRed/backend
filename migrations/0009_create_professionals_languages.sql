CREATE TABLE professionals_languages (
    id UUID PRIMARY KEY,
    professional_id UUID NOT NULL REFERENCES professionals(id) ON DELETE CASCADE,
    p_language VARCHAR(150),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);