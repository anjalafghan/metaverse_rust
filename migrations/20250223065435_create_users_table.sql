-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    avatar_id INTEGER,
    role varchar(5) NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW ()
);
