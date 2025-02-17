use serde::{Deserialize, Serialize};
use diesel::{self, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;
use crate::schema::sticker;
use crate::models::common::Model;

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
    type C = SqliteConnection;

    fn new(data: Self::NewT) -> Self {
        Sticker {
            id: Uuid::new_v4().to_string(),
            label: data.label,
            url: data.url,
        }
    }

    fn create(
        conn: &mut SqliteConnection,
        data: NewSticker
    ) -> Result<Sticker, diesel::result::Error> {
        let new_sticker = Sticker::new(data.label, data.url);

        diesel::insert_into(sticker::table)
            .values(&new_sticker)
            .execute(conn)?;

        Ok(new_sticker)
    }

    fn get_all(
        conn: &mut Self::C
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;
        let res = sticker.load(conn)?;
        Ok(res)
    }

    fn get_by_id(
        conn: &mut Self::C,
        sticker_id: &String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

        sticker.filter(id.eq(sticker_id))
            .first::<Sticker>(conn)
    }

    fn delete(
        conn: &mut Self::C,
        sticker_id: &String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

        diesel::delete(sticker.filter(id.eq(sticker_id))).execute(conn)
    }

    fn update(
        conn: &mut Self::C,
        data: Self::UpdateT,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

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
