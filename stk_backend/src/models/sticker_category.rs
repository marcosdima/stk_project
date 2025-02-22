use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    errors::AppError,
    routes::DbPool,
    schema::{
        self,
        sticker_category::{
            self,
            sticker_id,
            category_id,
        }
    }
};
use super::{
    categories::Category,
    common::BasicModel, stickers::Sticker
};

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
    ) -> Result<Vec<Category>, AppError>{
        use crate::schema::category::dsl::category;
        use crate::schema::sticker_category::dsl::sticker_category;

        let conn = &mut Self::get_conn(pool)?;
        let conn2 = &mut Self::get_conn(pool)?;

        let categories_ids: Vec<StickerCategory> = sticker_category.filter(sticker_id.eq(target)).load(conn)?;

        let res = category.filter(
            schema::category::id.eq_any(categories_ids.into_iter().map(|sc| sc.category_id.clone()))
        ).load(conn2)?;

        Ok(res)
    }

    pub fn category_stickers(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<Sticker>, AppError>{
        use crate::schema::sticker::dsl::sticker;
        use crate::schema::sticker_category::dsl::sticker_category;

        let conn = &mut Self::get_conn(pool)?;
        let conn2 = &mut Self::get_conn(pool)?;

        let stickers_ids: Vec<StickerCategory> = sticker_category.filter(category_id.eq(target)).load(conn)?;

        let res = sticker.filter(
            schema::sticker::id.eq_any(stickers_ids.into_iter().map(|sc| sc.sticker_id.clone()))
        ).load(conn2)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStickerCategory {
    sticker_id: Uuid,
    category_id: Uuid,
}