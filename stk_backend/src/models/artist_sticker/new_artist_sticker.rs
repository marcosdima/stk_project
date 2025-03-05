use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewArtistSticker {
    pub sticker_id: String,
    pub artist_id: String,
}

impl NewArtistSticker {
    pub fn new(stk_id: String, arts_id: String) -> Self {
        NewArtistSticker {
            artist_id: arts_id,
            sticker_id: stk_id,
        }
    }
}
