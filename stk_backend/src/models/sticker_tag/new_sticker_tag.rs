use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerTag {
    pub sticker_id: Uuid,
    pub tag_id: String,
}

impl NewStickerTag {
    pub fn new(stk_id: String, tag_id: String) -> Result<Self, uuid::Error> {
        let uuid_stk = Uuid::parse_str(&stk_id)?;

        Ok(NewStickerTag {
            tag_id,
            sticker_id: uuid_stk,
        })
    }
}

