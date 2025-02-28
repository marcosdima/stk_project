CREATE TABLE sticker_tag (
    tag_name TEXT NOT NULL,
    sticker_id TEXT NOT NULL,
    PRIMARY KEY(tag_name, sticker_id),
    FOREIGN KEY(tag_name) REFERENCES tag(name)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(sticker_id) REFERENCES sticker(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
