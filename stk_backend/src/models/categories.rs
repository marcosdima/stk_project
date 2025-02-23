use serde::{Deserialize, Serialize};
use diesel::{
    self, prelude::Insertable, query_dsl::methods::FilterDsl, AsChangeset, ExpressionMethods, Queryable, RunQueryDsl
};
use uuid::Uuid;
use crate::{errors::AppError, routes::DbPool, schema::category};

use super::common::{BasicModel, Model};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: String,
    pub name: String,
    sub_category_of: Option<String>
}

impl Category {
    pub fn test_new(
        id: String,
        name: String,
        sco: Option<String>,
    ) -> Category { 
        Category {
            id, name, sub_category_of: sco
        }
    }
    pub fn get_sub_category(&self) -> &Option<String> { &self.sub_category_of }
    pub fn validate_sub_category_of(
        pool: &DbPool,
        target: String,
        category: String,
    ) -> Result<(), AppError> {    
        let mut id = target.clone();

        loop {   
            let upper_category_id = {
                let category = Category::get_by_id(pool, id)?;
                category.get_sub_category().clone()
            };
            
            // Ask if has its own upper category...
            if let Some(sco) = upper_category_id {
                // If has it and its the same as the one received in 'upper_category_id'...
                if sco == category {
                    // Returns an error, because is trying to set a circular relation.
                    return Err(AppError::InvalidData("Circular relation is prohibited."));
                } else {
                    id = sco;
                }
            } else {
                break;
            }
        }
        
        Ok(())
    }
}

impl Model for Category {
    type UpdateT = CategoryUpdate;
    
    fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::category::dsl::*;
        let res = category.load(&mut Self::get_conn(pool)?)?;
        Ok(res)
    }

    fn get_by_id(
        pool: &DbPool,
        element_id: String,
    ) -> Result<Self, AppError> {
        use crate::schema::category::dsl::*;

        if let Ok(found) = category
            .filter(id.eq(element_id))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("Category with id provided does not exist!"))
        }
    }

    fn update(
        pool: &DbPool,
        data: Self::UpdateT,
    ) -> Result<(), AppError> {
        use crate::schema::category::dsl::*;
        // Checks if category exists...
        let _ = Self::get_by_id(pool, data.id.to_string())?;

        // Checks if is trying to set sub_category_of, then validate it.
        if let Some(sco) = data.sub_category_of.clone() {
            if let Err(err) = Category::validate_sub_category_of(
                pool,
                sco,
                data.id.to_string(),
            ) {
                return Err(err);
            }
        }

        diesel::update(category.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(())
    }

    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::category::dsl::*;

        let res = category.filter(
            id.eq_any(elements)
        ).load::<Category>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}

impl BasicModel for Category {
    type NewT = NewCategory;
    type PK = String;
    
    fn new(data: Self::NewT) -> Self {
        Category {
            id: Uuid::new_v4().to_string(),
            name: data.name,
            sub_category_of: data.sub_category_of,
        }
    }

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        // The id is valid if exists a category with it.
        if let Some(target) = data.sub_category_of.clone() {
            Category::get_by_id(pool, target)?;
        }

        let new_category = Self::new(data);

        diesel::insert_into(category::table)
            .values(&new_category)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_category)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::category::dsl::*;

        Ok(diesel::delete(category.filter(id.eq(element_id))).execute(&mut Self::get_conn(pool)?)?)
    }
}

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
