use serde::{Deserialize, Serialize};
use diesel::{self, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;
use crate::{
    errors::AppError,
    routes::DbPool,
    schema::sticker,
    models::common::Model,
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
    type NewT = NewSticker;
    type UpdateT = StickerUpdate;

    fn new(data: Self::NewT) -> Self {
        Sticker {
            id: Uuid::new_v4().to_string(),
            label: data.label,
            url: data.url,
        }
    }

    fn create(
        pool: &DbPool,
        data: NewSticker
    ) -> Result<Sticker, AppError> {
        let new_sticker = Sticker::new(data.label, data.url);
        let conn = &mut Self::get_conn(pool)?;

        diesel::insert_into(sticker::table)
            .values(&new_sticker)
            .execute(conn)?;

        Ok(new_sticker)
    }

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
        let conn = &mut Self::get_conn(pool)?;

        Ok(sticker.filter(id.eq(element_id))
            .first::<Sticker>(conn)?)
    }

    fn delete(
        pool: &DbPool,
        element_id: String,
    ) -> Result<usize, AppError> {
        use crate::schema::sticker::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        Ok(diesel::delete(sticker.filter(id.eq(element_id))).execute(conn)?)
    }

    fn update(
        pool: &DbPool,
        data: Self::UpdateT,
    ) -> Result<(), AppError> {
        use crate::schema::sticker::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        diesel::update(sticker.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(conn)?;

        Ok(())
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
