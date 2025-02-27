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
    routes::DbPool,
    schema::sticker_tag::{
        self,
        tag_name,
        sticker_id,
    }
};

use crate::models::common::BasicModel;

use super::new_sticker_tag::NewStickerTag;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = sticker_tag)]
pub struct StickerTag {
    // The order is important... wierd.
    tag_name: String,
    sticker_id: String,
}

impl BasicModel for StickerTag {
    type NewT = NewStickerTag;
    type PK = (String, String);

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(sticker_tag::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::sticker_tag::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let (name, stk_id) = element_id;

        Ok(
            diesel::delete(
                sticker_tag
                            .filter(sticker_id.eq(stk_id))
                            .filter(tag_name.eq(name))
            ).execute(conn)?
        )
    }
    
    fn new(data: Self::NewT) -> Self {
        StickerTag {
            sticker_id: data.sticker_id.to_string(),
            tag_name: data.tag_name,
        }
    } 
}

impl StickerTag {
    pub fn sticker_tags(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::sticker_tag::dsl::sticker_tag;

        let conn = &mut Self::get_conn(pool)?;

        let stk_tag_ids: Vec<StickerTag> = sticker_tag.filter(sticker_id.eq(target)).load(conn)?;
        println!("{:?}", stk_tag_ids);
        let elements = stk_tag_ids.into_iter().map(|sc| sc.tag_name.clone()).collect();
        
        Ok(elements)
    }

    pub fn tag_stickers(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::sticker_tag::dsl::sticker_tag;

        let conn = &mut Self::get_conn(pool)?;

        let stk_tag_ids: Vec<StickerTag> = sticker_tag.filter(tag_name.eq(target)).load(conn)?;
        
        let elements = stk_tag_ids.into_iter().map(|sc| sc.sticker_id.clone()).collect();

        Ok(elements)
    }
}
