use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
}

impl NewTag {
    pub fn new(new_name: String) -> Self {
        NewTag { name: new_name }
    }
}
