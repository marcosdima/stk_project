use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerTag {
    pub sticker_id: Uuid,
    pub tag_name: String,
}

impl NewStickerTag {
    pub fn new(stk_id: String, name: String) -> Result<Self, uuid::Error> {
        let uuid_stk = Uuid::parse_str(&stk_id)?;

        Ok(NewStickerTag {
            tag_name: name,
            sticker_id: uuid_stk,
        })
    }
}

