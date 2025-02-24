CREATE TABLE tag (
    name TEXT NOT NULL CHECK (LENGTH(name) > 1),
    PRIMARY KEY (name)
);
