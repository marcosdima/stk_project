use serde::{Deserialize, Serialize};
use diesel::{self, query_dsl::methods::FilterDsl, AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;
use crate::schema::category;

use super::common;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_category_of: Option<String>
}

impl common::Model for Category {
    type NewT = NewCategory;
    type UpdateT = CategoryUpdate;
    type C = SqliteConnection;

    fn new(data: Self::NewT) -> Self {
        Category {
            id: Uuid::new_v4().to_string(),
            name: data.name,
            sub_category_of: data.sub_category_of,
        }
    }

    fn create(
        conn: &mut Self::C,
        data: Self::NewT
    ) -> Result<Self, diesel::result::Error> {
        let new_category = Category::new(data);

        diesel::insert_into(category::table)
            .values(&new_category)
            .execute(conn)?;

        Ok(new_category)
    }

    fn get_all(
        conn: &mut Self::C
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::category::dsl::*;
        let res = category.load(conn)?;
        Ok(res)
    }

    fn get_by_id(
        conn: &mut Self::C,
        element_id: &String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::category::dsl::*;

        category.filter(id.eq(element_id))
            .first::<Self>(conn)
    }

    fn delete(
        conn: &mut Self::C,
        element_id: &String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::category::dsl::*;

        diesel::delete(category.filter(id.eq(element_id))).execute(conn)
    }

    fn update(
        conn: &mut Self::C,
        data: Self::UpdateT,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::category::dsl::*;

        diesel::update(category.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(conn)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCategory {
    pub name: String,
    pub sub_category_of: Option<String>
}

impl NewCategory {
    pub fn new(name: String, sub_category_of: Option<String>) -> Self {
        NewCategory { name, sub_category_of }
    }
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = category)]
pub struct CategoryUpdate {
    pub id: Uuid,
    pub name: String,
    pub sub_category_of: Option<String>
}

impl CategoryUpdate {
    pub fn new(id: String, name: String, sub_category_of: Option<String>) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(&id)?;
        Ok(
            CategoryUpdate {
                id: uuid,
                name,
                sub_category_of
            }
        )
    }
}
