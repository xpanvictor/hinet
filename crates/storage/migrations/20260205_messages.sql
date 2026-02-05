
CREATE TABLE IF NOT EXISTS rooms(
    id TEXT PRIMARY KEY
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
);

