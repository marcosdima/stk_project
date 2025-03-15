use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserRole {
    pub user_id: String,
    pub role_id: i32,
}

impl NewUserRole {
    pub fn new(user_id: String, role_id: i32) -> Self {
        NewUserRole { user_id, role_id }
    }
}