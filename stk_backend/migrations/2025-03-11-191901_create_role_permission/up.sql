CREATE TABLE role_permission (
    role_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    PRIMARY KEY(role_id, permission_id),
    FOREIGN KEY(role_id) REFERENCES role(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(permission_id) REFERENCES permission(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
