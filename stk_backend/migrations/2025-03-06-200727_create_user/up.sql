CREATE TABLE user (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (LENGTH(name) > 1),
    lastname TEXT NOT NULL CHECK (LENGTH(lastname) > 1),
    username TEXT NOT NULL UNIQUE CHECK (LENGTH(username) > 1),
    password_hash TEXT NOT NULL
);
