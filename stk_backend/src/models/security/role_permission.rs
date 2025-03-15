use serde::{
    Deserialize,
    Serialize,
};

use diesel:: Queryable;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
#[diesel(table_name = role)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}
