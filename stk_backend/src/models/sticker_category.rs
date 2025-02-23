use diesel::{
    prelude::{
        Insertable,
        Queryable},
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
    routes::DbPool,
    schema::sticker_category::{
        self,
        category_id, sticker_id,
    }
};
use super::common::BasicModel;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = sticker_category)]
pub struct StickerCategory {
    sticker_id: String,
    category_id: String,
}

impl BasicModel for StickerCategory {
    type NewT = NewStickerCategory;
    type PK = (String, String);

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(sticker_category::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::sticker_category::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let (stk_id, cat_id) = element_id;

        Ok(
            diesel::delete(
                sticker_category
                            .filter(sticker_id.eq(stk_id))
                            .filter(category_id.eq(cat_id))
            ).execute(conn)?
        )
    }
    
    fn new(data: Self::NewT) -> Self {
        StickerCategory {
            sticker_id: data.sticker_id.to_string(),
            category_id: data.category_id.to_string(),
        }
    } 
}

impl StickerCategory {
    pub fn sticker_categories(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::sticker_category::dsl::sticker_category;

        let conn = &mut Self::get_conn(pool)?;

        let stk_cat_ids: Vec<StickerCategory> = sticker_category.filter(sticker_id.eq(target)).load(conn)?;
        let elements = stk_cat_ids.into_iter().map(|sc| sc.category_id.clone()).collect();
        
        Ok(elements)
    }

    pub fn category_stickers(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::sticker_category::dsl::sticker_category;

        let conn = &mut Self::get_conn(pool)?;

        let stk_cat_ids: Vec<StickerCategory> = sticker_category.filter(category_id.eq(target)).load(conn)?;
        
        let elements = stk_cat_ids.into_iter().map(|sc| sc.sticker_id.clone()).collect();

        Ok(elements)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerCategory {
    sticker_id: Uuid,
    category_id: Uuid,
}

impl NewStickerCategory {
    pub fn new(stk_id: String, cat_id: String) -> Result<Self, uuid::Error> {
        let uuid_stk = Uuid::parse_str(&stk_id)?;
        let uuid_cat = Uuid::parse_str(&cat_id)?;
        Ok(NewStickerCategory {
            category_id: uuid_cat,
            sticker_id: uuid_stk,
        })
    }
}