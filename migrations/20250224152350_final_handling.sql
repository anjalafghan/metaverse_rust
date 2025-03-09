-- SQL translation of Prisma schema

-- Enum for Role
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_enum') THEN
        CREATE TYPE role_enum AS ENUM ('Admin', 'User');
    END IF;
END$$;

-- Avatar table
CREATE TABLE IF NOT EXISTS avatars (
    id SERIAL PRIMARY KEY,
    image_url VARCHAR(200),
    name VARCHAR(200)
);

-- User table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(100) UNIQUE NOT NULL,
    avatar_id INTEGER,
    role role_enum NOT NULL,
    FOREIGN KEY (avatar_id) REFERENCES avatars (id)
);

-- Map table
CREATE TABLE IF NOT EXISTS maps (
    id SERIAL PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    name VARCHAR(100)
);

-- Space table
CREATE TABLE IF NOT EXISTS space (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER,
    thumbnail VARCHAR(200),
    map_id INTEGER, -- Corrected data type
    FOREIGN KEY (map_id) REFERENCES maps (id)
);

-- Element table
CREATE TABLE IF NOT EXISTS elements (
    id SERIAL PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    image_url VARCHAR(200)
);

-- Space Elements table
CREATE TABLE IF NOT EXISTS space_elements (
    id SERIAL PRIMARY KEY,
    element_id INTEGER NOT NULL, -- Corrected data type
    space_id INTEGER NOT NULL, -- Corrected data type
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    FOREIGN KEY (element_id) REFERENCES elements (id),
    FOREIGN KEY (space_id) REFERENCES space (id)
);

-- Map Elements table
CREATE TABLE IF NOT EXISTS map_elements (
    id SERIAL PRIMARY KEY,
    map_id INTEGER NOT NULL, -- Corrected data type
    element_id INTEGER, -- Corrected data type
    x INTEGER,
    y INTEGER,
    FOREIGN KEY (map_id) REFERENCES maps (id),
    FOREIGN KEY (element_id) REFERENCES elements (id)
);
