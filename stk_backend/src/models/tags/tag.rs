use diesel::{
    prelude::{
        Insertable,
        Queryable
    },
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl
};

use serde::{
    Serialize,
    Deserialize
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        tags::TagUpdate,
        Model,
    },
    routes::DbPool,
    schema::tag
};

use crate::models::common::BasicModel;

use super::NewTag;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = tag)]
pub struct Tag {
    pub id: String,
    pub name: String
}

impl Model for Tag { 
    type UpdateT = TagUpdate;

    fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::tag::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let res = tag.load(conn)?;
        Ok(res)
    }

    fn get_by_id(
        pool: &DbPool,
        element_id: String,
    ) -> Result<Self, AppError> {
        use crate::schema::tag::dsl::*;

        if let Ok(found) = tag
            .filter(name.eq(element_id))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("Sticker with id provided does not exist!"))
        }
    }

    fn update(
        pool: &DbPool,
        data: Self::UpdateT,
    ) -> Result<(), AppError> {
        use crate::schema::tag::dsl::*;

        diesel::update(tag.filter(id.eq(&data.id)))
            .set(&data)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(())
    }
    
    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::tag::dsl::*;
        
        let res = tag.filter(
            name.eq_any(elements)
        ).load::<Self>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}

impl BasicModel for Tag {
    type NewT = NewTag;
    type PK = String;

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(tag::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::tag::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        Ok(
            diesel::delete(
                tag
                            .filter(name.eq(element_id))
            ).execute(conn)?
        )
    }
    
    fn new(data: Self::NewT) -> Self {
        Tag {
            id: Uuid::new_v4().to_string(),
            name: data.name,
        }
    } 
}
