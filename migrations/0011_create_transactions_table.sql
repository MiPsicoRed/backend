CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    payment_intent_id TEXT,
    session_id TEXT NOT NULL,
    amount BIGINT,
    currency VARCHAR(3),
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
