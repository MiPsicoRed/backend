CREATE TABLE professionals_specializations (
    id UUID PRIMARY KEY,
    professional_id UUID NOT NULL REFERENCES professionals(id) ON DELETE CASCADE,
    s_name VARCHAR(150),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);