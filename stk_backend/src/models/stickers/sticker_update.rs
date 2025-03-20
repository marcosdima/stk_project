use diesel::prelude::AsChangeset;

use serde::{
    Serialize,
    Deserialize
};

use crate::schema::sticker;

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = sticker)]
pub struct StickerUpdate {
    pub id: String,
    pub label: String,
    pub url: String,
}

impl StickerUpdate {
    pub fn new(id: String, label: String, url: String) -> Self {
        StickerUpdate {
            id,
            label,
            url
        }
    }
}
