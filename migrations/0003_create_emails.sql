CREATE TABLE emails (
    id UUID PRIMARY KEY,
    from_mail VARCHAR(100) NOT NULL, 
    to_mail VARCHAR(100) NOT NULL,
    mail_subject  VARCHAR(256) NOT NULL,
    mail_body  TEXT NOT NULL,
    email_kind  INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);