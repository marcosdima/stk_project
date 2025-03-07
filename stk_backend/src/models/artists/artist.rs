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
    routes::DbPool,
    schema::artist
};

use crate::models::{
    artists::{
        ArtistUpdate,
        NewArtist,
    },
    common::{
        BasicModel,
        Model
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = artist)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    pub presentation: Option<String>,
}

impl Model for Artist {
    type UpdateT = ArtistUpdate;
    
    fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::artist::dsl::*;
        let res = artist.load(&mut Self::get_conn(pool)?)?;
        Ok(res)
    }

    fn get_by_id(
        pool: &DbPool,
        element_id: String,
    ) -> Result<Self, AppError> {
        use crate::schema::artist::dsl::*;

        if let Ok(found) = artist
            .filter(id.eq(element_id))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("Artist with id provided does not exist!"))
        }
    }

    fn update(
        pool: &DbPool,
        data: Self::UpdateT,
    ) -> Result<(), AppError> {
        use crate::schema::artist::dsl::*;

        diesel::update(artist.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(())
    }

    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::artist::dsl::*;

        let res = artist.filter(
            id.eq_any(elements)
        ).load::<Self>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}

impl BasicModel for Artist {
    type NewT = NewArtist;
    type PK = String;
    
    fn new(data: Self::NewT) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: data.name,
            logo_url: data.logo_url,
            presentation: data.presentation,
        }
    }

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_artist = Self::new(data);

        diesel::insert_into(artist::table)
            .values(&new_artist)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_artist)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::artist::dsl::*;

        Ok(diesel::delete(artist.filter(id.eq(element_id))).execute(&mut Self::get_conn(pool)?)?)
    }
}
