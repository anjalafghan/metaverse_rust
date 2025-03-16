-- Enum types
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_enum') THEN
        CREATE TYPE role_enum AS ENUM ('Admin', 'User');
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'element_type_enum') THEN
        CREATE TYPE element_type_enum AS ENUM ('Static', 'Interactive', 'Decorative', 'Portal');
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'session_status_enum') THEN
        CREATE TYPE session_status_enum AS ENUM ('Active', 'Inactive', 'Away');
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'message_type_enum') THEN
        CREATE TYPE message_type_enum AS ENUM ('Text', 'Audio', 'Video', 'System');
    END IF;
END$$;

-- Avatar table
CREATE TABLE avatars (
    id SERIAL PRIMARY KEY,
    image_url VARCHAR(200) NOT NULL,
    name VARCHAR(200) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- User table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    avatar_id INTEGER,
    role role_enum NOT NULL DEFAULT 'User',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE,
    is_online BOOLEAN DEFAULT FALSE,
    -- Last position data for quick reconnection
    last_space_id INTEGER,
    last_map_id INTEGER,
    last_x INTEGER,
    last_y INTEGER,
    last_rotation INTEGER DEFAULT 0,
    FOREIGN KEY (avatar_id) REFERENCES avatars (id)
);

-- World table (top-level container)
CREATE TABLE worlds (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    thumbnail_url VARCHAR(200),
    creator_id INTEGER NOT NULL,
    is_public BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES users (id)
);

-- Map table (areas within worlds)
CREATE TABLE maps (
    id SERIAL PRIMARY KEY,
    world_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    background_url VARCHAR(200),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (world_id) REFERENCES worlds (id)
);

-- Space table (rooms/areas within maps)
CREATE TABLE spaces (
    id SERIAL PRIMARY KEY,
    map_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    background_url VARCHAR(200),
    thumbnail_url VARCHAR(200),
    max_occupancy INTEGER DEFAULT 0, -- 0 means unlimited
    is_private BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    default_spawn_x INTEGER,
    default_spawn_y INTEGER,
    FOREIGN KEY (map_id) REFERENCES maps (id)
);

-- Element template table (reusable element definitions)
CREATE TABLE element_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type element_type_enum NOT NULL,
    image_url VARCHAR(200) NOT NULL,
    model_url VARCHAR(200), -- For 3D models in Bevy
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    is_collidable BOOLEAN DEFAULT FALSE,
    interaction_data JSONB, -- Stores interaction behavior
    physics_properties JSONB, -- For Bevy physics integration
    animation_data JSONB, -- For Bevy animations
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Space Elements table (instances of elements in spaces)
CREATE TABLE space_elements (
    id SERIAL PRIMARY KEY,
    space_id INTEGER NOT NULL,
    template_id INTEGER NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z_index INTEGER DEFAULT 0,
    rotation INTEGER DEFAULT 0,
    custom_properties JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (space_id) REFERENCES spaces (id),
    FOREIGN KEY (template_id) REFERENCES element_templates (id)
);

-- Map Elements table (elements visible on the map, like portals between spaces)
CREATE TABLE map_elements (
    id SERIAL PRIMARY KEY,
    map_id INTEGER NOT NULL,
    template_id INTEGER NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z_index INTEGER DEFAULT 0,
    target_space_id INTEGER,
    custom_properties JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (map_id) REFERENCES maps (id),
    FOREIGN KEY (template_id) REFERENCES element_templates (id),
    FOREIGN KEY (target_space_id) REFERENCES spaces (id)
);

-- User Sessions table (tracks active user sessions)
CREATE TABLE user_sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    space_id INTEGER,
    map_id INTEGER,
    x INTEGER,
    y INTEGER,
    rotation INTEGER DEFAULT 0,
    status session_status_enum DEFAULT 'Active',
    connected_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    client_data JSONB, -- Store client-specific data (device, browser, etc.)
    connection_id VARCHAR(255), -- WebSocket connection identifier
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (space_id) REFERENCES spaces (id),
    FOREIGN KEY (map_id) REFERENCES maps (id),
    CHECK ((space_id IS NOT NULL AND map_id IS NULL) OR (space_id IS NULL AND map_id IS NOT NULL))
);

-- Rooms table (for chats/calls)
CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    space_id INTEGER,
    creator_id INTEGER NOT NULL,
    is_persistent BOOLEAN DEFAULT FALSE,
    is_private BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (space_id) REFERENCES spaces (id),
    FOREIGN KEY (creator_id) REFERENCES users (id)
);

-- Room Members table
CREATE TABLE room_members (
    id SERIAL PRIMARY KEY,
    room_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_admin BOOLEAN DEFAULT FALSE,
    media_preferences JSONB, -- WebRTC preferences (camera/mic enabled, etc.)
    FOREIGN KEY (room_id) REFERENCES rooms (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (room_id, user_id)
);

-- WebRTC connections table
CREATE TABLE webrtc_connections (
    id SERIAL PRIMARY KEY,
    initiator_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    room_id INTEGER NOT NULL,
    connection_state VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (initiator_id) REFERENCES users (id),
    FOREIGN KEY (receiver_id) REFERENCES users (id),
    FOREIGN KEY (room_id) REFERENCES rooms (id)
);

-- Messages table
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    room_id INTEGER NOT NULL,
    sender_id INTEGER NOT NULL,
    message_type message_type_enum DEFAULT 'Text',
    content TEXT,
    media_url VARCHAR(200),
    sent_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (room_id) REFERENCES rooms (id),
    FOREIGN KEY (sender_id) REFERENCES users (id)
);

-- User Relationships table
CREATE TABLE user_relationships (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    related_user_id INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL, -- 'pending', 'accepted', 'blocked'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (related_user_id) REFERENCES users (id),
    UNIQUE (user_id, related_user_id)
);

-- User Permissions for Worlds/Maps/Spaces
CREATE TABLE user_permissions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    world_id INTEGER,
    map_id INTEGER,
    space_id INTEGER,
    can_edit BOOLEAN DEFAULT FALSE,
    can_moderate BOOLEAN DEFAULT FALSE,
    can_invite BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (world_id) REFERENCES worlds (id),
    FOREIGN KEY (map_id) REFERENCES maps (id),
    FOREIGN KEY (space_id) REFERENCES spaces (id),
    CHECK (
        (world_id IS NOT NULL AND map_id IS NULL AND space_id IS NULL) OR
        (world_id IS NULL AND map_id IS NOT NULL AND space_id IS NULL) OR
        (world_id IS NULL AND map_id IS NULL AND space_id IS NOT NULL)
    )
);

-- Asset References table (for Bevy)
CREATE TABLE assets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(50) NOT NULL, -- 'model', 'texture', 'audio', etc.
    url VARCHAR(255) NOT NULL,
    hash VARCHAR(64), -- For asset versioning/caching
    creator_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES users (id)
);

-- Create indexes for frequently queried columns
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_last_space_id ON users(last_space_id) WHERE last_space_id IS NOT NULL;
CREATE INDEX idx_users_last_map_id ON users(last_map_id) WHERE last_map_id IS NOT NULL;
CREATE INDEX idx_spaces_map_id ON spaces(map_id);
CREATE INDEX idx_space_elements_space_id ON space_elements(space_id);
CREATE INDEX idx_map_elements_map_id ON map_elements(map_id);
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_space_id ON user_sessions(space_id) WHERE space_id IS NOT NULL;
CREATE INDEX idx_user_sessions_map_id ON user_sessions(map_id) WHERE map_id IS NOT NULL;
CREATE INDEX idx_user_sessions_connection_id ON user_sessions(connection_id);
CREATE INDEX idx_messages_room_id ON messages(room_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at);
CREATE INDEX idx_user_relationships_user_id ON user_relationships(user_id);
CREATE INDEX idx_webrtc_connections_initiator_receiver ON webrtc_connections(initiator_id, receiver_id);

-- Create functions and triggers for position synchronization
CREATE OR REPLACE FUNCTION update_user_last_position()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE users
    SET
        last_space_id = NEW.space_id,
        last_map_id = NEW.map_id,
        last_x = NEW.x,
        last_y = NEW.y,
        last_rotation = NEW.rotation
    WHERE id = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sync_user_position
AFTER UPDATE OF x, y, rotation, space_id, map_id ON user_sessions
FOR EACH ROW
EXECUTE FUNCTION update_user_last_position();

-- Function to handle user disconnect
CREATE OR REPLACE FUNCTION handle_user_disconnect()
RETURNS TRIGGER AS $$
BEGIN
    -- If session is deleted, ensure last position is saved
    UPDATE users
    SET
        last_space_id = OLD.space_id,
        last_map_id = OLD.map_id,
        last_x = OLD.x,
        last_y = OLD.y,
        last_rotation = OLD.rotation,
        is_online = FALSE
    WHERE id = OLD.user_id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_session_delete
BEFORE DELETE ON user_sessions
FOR EACH ROW
EXECUTE FUNCTION handle_user_disconnect();
