CREATE TABLE session_types (
    id UUID PRIMARY KEY,
    session_type_name VARCHAR(100) NOT NULL, 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
