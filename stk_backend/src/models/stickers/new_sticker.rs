use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSticker {
    pub label: String,
    pub url: String,
}

impl NewSticker {
    pub fn new(label: String, url: String) -> Self {
        NewSticker { label, url }
    }
}
