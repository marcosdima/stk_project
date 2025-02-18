CREATE TABLE category (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sub_category_of TEXT,
    FOREIGN KEY(sub_category_of) REFERENCES category(id)
);
