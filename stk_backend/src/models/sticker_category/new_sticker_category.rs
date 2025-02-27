use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerCategory {
    pub sticker_id: Uuid,
    pub category_id: Uuid,
}

impl NewStickerCategory {
    pub fn new(stk_id: String, cat_id: String) -> Result<Self, uuid::Error> {
        let uuid_stk = Uuid::parse_str(&stk_id)?;
        let uuid_cat = Uuid::parse_str(&cat_id)?;
        Ok(NewStickerCategory {
            category_id: uuid_cat,
            sticker_id: uuid_stk,
        })
    }
}