-- Add migration script here
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    user_name VARCHAR NOT NULL,
    mail_address VARCHAR NOT NULL
);