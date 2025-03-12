CREATE TABLE user_role (
    user_id TEXT NOT NULL UNIQUE,
    role_id INTEGER NOT NULL,
    PRIMARY KEY(user_id, role_id),
    FOREIGN KEY(user_id) REFERENCES user(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(role_id) REFERENCES role(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);