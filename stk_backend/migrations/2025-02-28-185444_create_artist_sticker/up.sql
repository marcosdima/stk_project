CREATE TABLE artist_sticker (
    artist_id TEXT NOT NULL,
    sticker_id TEXT NOT NULL UNIQUE,
    PRIMARY KEY(artist_id, sticker_id),
    FOREIGN KEY(artist_id) REFERENCES artist(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(sticker_id) REFERENCES sticker(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);