
CREATE TABLE IF NOT EXISTS contacts(
    id TEXT PRIMARY KEY,
    username TEXT,
    peer_id TEXT UNIQUE,
    public_key TEXT UNIQUE,
    bio TEXT,
    last_seen TIMESTAMP,
    note TEXT
);

-- id = is_dm ? xor(a, b) : sha(room_name)
CREATE TABLE IF NOT EXISTS rooms(
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS messages(
    id TEXT PRIMARY KEY,
    is_group BOOLEAN DEFAULT(false), 
    content TEXT NOT NULL,
    timestamp TIMESTAMP,
    room_id TEXT NOT NULL,
    CONSTRAINT fk_rooms
    FOREIGN KEY room_id
    REFERENCES rooms(id)
    sender_id TEXT NOT NULL,
    CONSTRAINT fk_sender
    FOREIGN KEY sender_id
    REFERENCES contacts(id)
);

