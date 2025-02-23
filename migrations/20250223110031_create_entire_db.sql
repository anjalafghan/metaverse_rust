-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    avatar_id INTEGER,
    role varchar(5) NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW ()
);

CREATE TABLE IF NOT EXISTS space (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    width INTEGER NOT NULL,
    height INTEGER,
    thumbnail VARCHAR(200)
);

CREATE TABLE IF NOT EXISTS space_elements (
    id SERIAL PRIMARY KEY,
    element_id INTEGER NOT NULL,
    space_id INTEGER NOT NULL,
    position_x INTEGER NOT NULL,
    position_y INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS elements (
    id SERIAL PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    image_url VARCHAR(200)
);

CREATE TABLE IF NOT EXISTS map (
    id SERIAL PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    name VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS map_elements (
    id SERIAL PRIMARY KEY,
    element_id INTEGER,
    map_id INTEGER NOT NULL,
    position_x INTEGER NOT NULL,
    position_y INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS model (
    id SERIAL PRIMARY KEY,
    image_url VARCHAR(200),
    name VARCHAR(200)
);
