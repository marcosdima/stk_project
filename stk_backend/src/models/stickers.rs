use serde::{Deserialize, Serialize};
use diesel::{self, prelude::Insertable, AsChangeset, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;
use crate::{
    errors::AppError,
    routes::DbPool,
    schema::sticker,
    models::common::Model,
};

use super::common::BasicModel;

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
        let conn = &mut Self::get_conn(pool)?;

        // Checks if category exists...
        let _ = Self::get_by_id(pool, data.id.to_string())?;

        diesel::update(sticker.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(conn)?;

        Ok(())
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSticker {
    pub label: String,
    pub url: String,
}

impl NewSticker {
    pub fn new(label: String, url: String) -> Self {
        NewSticker { label, url }
    }
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = sticker)]
pub struct StickerUpdate {
    pub id: Uuid,
    pub label: String,
    pub url: String,
}

impl StickerUpdate {
    pub fn new(id: String, label: String, url: String) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(&id)?;
        Ok(
            StickerUpdate {
                id: uuid,
                label,
                url
            }
        )
    }
}
