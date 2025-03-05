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
    schema::artist_sticker::{
        self,
        artist_id,
        sticker_id,
    }
};

use crate::models::common::BasicModel;

use super::new_artist_sticker::NewArtistSticker;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = artist_sticker)]
pub struct ArtistSticker {
    pub artist_id: String,
    pub sticker_id: String,
}

impl BasicModel for ArtistSticker {
    type NewT = NewArtistSticker;
    type PK = (String, String);

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(artist_sticker::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::artist_sticker::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let (stk_id, arts_id) = element_id;

        Ok(
            diesel::delete(
                artist_sticker
                            .filter(sticker_id.eq(stk_id))
                            .filter(artist_id.eq(arts_id))
            ).execute(conn)?
        )
    }
    
    fn new(data: Self::NewT) -> Self {
        ArtistSticker {
            sticker_id: data.sticker_id.to_string(),
            artist_id: data.artist_id.to_string(),
        }
    } 
}

impl ArtistSticker {
    pub fn sticker_artists(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::artist_sticker::dsl::artist_sticker;

        let conn = &mut Self::get_conn(pool)?;

        let arts_stk_ids: Vec<ArtistSticker> = artist_sticker.filter(sticker_id.eq(target)).load(conn)?;
        let elements = arts_stk_ids.into_iter().map(|sc| sc.artist_id.clone()).collect();
        
        Ok(elements)
    }

    pub fn artist_stickers(
        pool: &DbPool,
        target: String,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::artist_sticker::dsl::artist_sticker;

        let conn = &mut Self::get_conn(pool)?;

        let arts_stk_ids: Vec<Self> = artist_sticker.filter(artist_id.eq(target)).load(conn)?;
        
        let elements = arts_stk_ids.into_iter().map(|sc| sc.sticker_id.clone()).collect();

        Ok(elements)
    }
}
