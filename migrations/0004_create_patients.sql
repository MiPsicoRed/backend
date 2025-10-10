CREATE TABLE patients (
    id UUID PRIMARY KEY,
    user_id UUID, -- this can be null, since we can have patients that are not users 
    gender_id INTEGER NOT NULL,
    sexual_orientation_id INTEGER NOT NULL,
    birthdate DATE NOT NULL, -- repeated from user
    phone VARCHAR(30) NOT NULL, -- repeated from user
    emergency_contact_name VARCHAR(150), 
    emergency_contact_phone VARCHAR(20), 
    insurance_policy_number VARCHAR(100),
    medical_history TEXT,
    current_medications TEXT,
    allergies TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
