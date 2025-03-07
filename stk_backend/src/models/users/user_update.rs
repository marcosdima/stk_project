use diesel::prelude::AsChangeset;

use serde::{
    Serialize,
    Deserialize
};

use crate::schema::user;

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = user)]
pub struct UserUpdate {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub username: String,
    pub password_hash: String,
}

impl UserUpdate {
    pub fn new(
        id: String,
        name: String,
        lastname: String,
        username: String,
        password_hash: String,
    ) -> Self {
        UserUpdate {
            id,
            name,
            lastname,
            username,
            password_hash,
        }
    }
}
