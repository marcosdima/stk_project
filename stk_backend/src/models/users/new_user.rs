use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub lastname: String,
    pub username: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(
        name: String,
        lastname: String,
        username: String,
        password_hash: String,
    ) -> Self {
        NewUser { 
            name,
            lastname,
            username,
            password_hash,
        }
    }
}
