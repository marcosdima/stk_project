use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewArtist {
    pub name: String,
    pub logo_url: String,
    pub presentation: Option<String>,
}

impl NewArtist {
    pub fn new(
        name: String,
        logo_url: String,
        presentation: Option<String>
    ) -> Self {
        Self {
            name,
            logo_url,
            presentation,
        }
    }
}
