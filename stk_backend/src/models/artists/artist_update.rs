use serde::{
    Deserialize,
    Serialize
};

use diesel::AsChangeset;

use crate::schema::artist;

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = artist)]
pub struct ArtistUpdate {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    pub presentation: Option<String>,
}

impl ArtistUpdate {
    pub fn new(id: String, name: String, logo_url: String, presentation: Option<String>) -> Self {
        ArtistUpdate {
            id,
            name,
            logo_url,
            presentation,
        }
    }
}
