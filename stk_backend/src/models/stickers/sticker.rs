use serde::{
    Deserialize,
    Serialize
};

use diesel::{
    self,
    prelude::Insertable,
    ExpressionMethods,
    QueryDsl,
    Queryable,
    RunQueryDsl
};

use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        common::Model,
        BasicModel
    },
    routes::DbPool,
    schema::sticker
};

use super::{
    StickerUpdate,
    NewSticker,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = sticker)]
pub struct Sticker {
    pub id: String,
    pub label: String,
    pub url: String,
}

impl Sticker {
    pub fn new(
        label: String,
        url: String
    ) -> Self {
        Sticker {
            id: Uuid::new_v4().to_string(),
            label,
            url,
        }
    }
}

impl Model for Sticker { 
    type UpdateT = StickerUpdate;

    fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::sticker::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let res = sticker.load(conn)?;
        Ok(res)
    }

    fn get_by_id(
        pool: &DbPool,
        element_id: String,
    ) -> Result<Self, AppError> {
        use crate::schema::sticker::dsl::*;

        if let Ok(found) = sticker
            .filter(id.eq(element_id))
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
        use crate::schema::sticker::dsl::*;
        // Checks if category exists...
        let _ = Self::get_by_id(pool, data.id.to_string())?;

        diesel::update(sticker.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(())
    }
    
    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::sticker::dsl::*;
        
        let res = sticker.filter(
            id.eq_any(elements)
        ).load::<Sticker>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}

impl BasicModel for Sticker {
    type NewT = NewSticker;
    type PK = String;
    
    fn new(data: Self::NewT) -> Self {
        Sticker {
            id: Uuid::new_v4().to_string(),
            label: data.label,
            url: data.url,
        }
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::sticker::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        Ok(
            diesel::delete(sticker.filter(id.eq(element_id)))
                .execute(conn)?
        )
    }
    
    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(sticker::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

}
