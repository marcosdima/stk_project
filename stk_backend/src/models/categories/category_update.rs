use serde::{Deserialize, Serialize};
use diesel::AsChangeset;
use uuid::Uuid;
use crate::schema::category;

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = category)]
pub struct CategoryUpdate {
    pub id: Uuid,
    pub name: String,
    pub sub_category_of: Option<String>
}

impl CategoryUpdate {
    pub fn new(id: String, name: String, sco: Option<String>) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(&id)?;
        Ok(
            CategoryUpdate {
                id: uuid,
                name,
                sub_category_of: sco
            }
        )
    }
}
