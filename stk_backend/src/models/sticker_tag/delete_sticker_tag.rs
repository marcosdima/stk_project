use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStickerTag {
    pub tag_id: String,
    pub sticker_id: String,
}