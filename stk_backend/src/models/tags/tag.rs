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
    pub name: String,
}

impl Tag { 
    pub fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::tag::dsl::*;
        let res = tag.load(&mut Self::get_conn(pool)?)?;
        Ok(res)
    }

    /*pub fn get_tag_stickers(
        pool: &DbPool,
        target: String,
    ) -> Result<Self, AppError> {
        use crate::schema::tag::dsl::*;

        if let Ok(found) = tag
            .filter(name.eq(target))
            .first::<Self>(&mut Self::get_conn(pool)?)
        {
            Ok(found)
        } else {
            Err(AppError::NotFound("Tag with the name provided does not exist!"))
        }
    }*/

    pub fn change_name(
        pool: &DbPool,
        new_name: String,
    ) -> Result<(), AppError> {
        let data = TagUpdate::new(new_name);

        Self::update(&pool, data)?;

        Ok(())
    }
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

        diesel::update(tag.filter(name.eq(&data.name)))
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
            name: data.name,
        }
    } 
}
