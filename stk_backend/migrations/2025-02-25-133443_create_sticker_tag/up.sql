CREATE TABLE sticker_tag (
    tag_id TEXT NOT NULL,
    sticker_id TEXT NOT NULL,
    PRIMARY KEY(tag_id, sticker_id),
    FOREIGN KEY(tag_id) REFERENCES tag(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(sticker_id) REFERENCES sticker(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
