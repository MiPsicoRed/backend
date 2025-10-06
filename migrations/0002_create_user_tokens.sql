-- user_token is deleted if user is deleted, also if we try to insert/update... and the user_id does not exist 
-- the database will reject the insert because of the foreign key constraint
CREATE TABLE user_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, 
    token VARCHAR(256) NOT NULL,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);