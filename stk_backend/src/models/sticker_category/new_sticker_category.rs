use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerCategory {
    pub sticker_id: String,
    pub category_id: String,
}

impl NewStickerCategory {
    pub fn new(stk_id: String, cat_id: String) -> Self {
       NewStickerCategory {
            category_id: cat_id,
            sticker_id: stk_id,
        }
    }
}