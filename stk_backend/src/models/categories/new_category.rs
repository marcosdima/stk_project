use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCategory {
    pub name: String,
    pub sub_category_of: Option<String>
}

impl NewCategory {
    pub fn new(name: String, sco: Option<String>) -> Self {
        NewCategory { name, sub_category_of: sco }
    }
}
