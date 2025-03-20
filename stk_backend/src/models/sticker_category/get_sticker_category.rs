use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStickerCategory {
    pub category_id: String,
    pub sticker_id: String,
}