use diesel::prelude::AsChangeset;

use serde::{
    Serialize,
    Deserialize
};

use crate::schema::tag;

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = tag)]
pub struct TagUpdate {
    pub name: String,
}

impl TagUpdate {
    pub fn new(new_name: String) -> Self {
        TagUpdate { name: new_name }
    }
}
