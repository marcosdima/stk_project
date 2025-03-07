use diesel::prelude::AsChangeset;

use serde::{
    Serialize,
    Deserialize
};

use crate::schema::tag;

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = tag)]
pub struct TagUpdate {
    pub id: String,
    pub name: String,
}

impl TagUpdate {
    pub fn new(id: String, name: String) -> Self {
        TagUpdate { id, name }
    }
}
