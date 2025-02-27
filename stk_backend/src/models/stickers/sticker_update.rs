use diesel::prelude::AsChangeset;

use serde::{
    Serialize,
    Deserialize
};

use uuid::Uuid;

use crate::schema::sticker;

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = sticker)]
pub struct StickerUpdate {
    pub id: Uuid,
    pub label: String,
    pub url: String,
}

impl StickerUpdate {
    pub fn new(id: String, label: String, url: String) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(&id)?;
        Ok(
            StickerUpdate {
                id: uuid,
                label,
                url
            }
        )
    }
}
