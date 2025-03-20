use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistSticker {
    pub artist_id: String,
    pub sticker_id: String,
}