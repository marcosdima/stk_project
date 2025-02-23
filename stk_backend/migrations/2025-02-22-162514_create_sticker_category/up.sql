CREATE TABLE sticker_category (
    sticker_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    PRIMARY KEY (sticker_id, category_id),
    FOREIGN KEY(category_id) REFERENCES category(id),
    FOREIGN KEY(sticker_id) REFERENCES sticker(id)
);
