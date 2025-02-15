use serde::{Deserialize, Serialize};
use diesel::{self, query_dsl::methods::FilterDsl, AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;
use crate::schema::{self, sticker};

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

    pub fn create(
        conn: &mut SqliteConnection,
        data: NewSticker
    ) -> Result<Sticker, diesel::result::Error> {
        let new_sticker = Sticker::new(data.label, data.url);

        diesel::insert_into(schema::sticker::table)
            .values(&new_sticker)
            .execute(conn)?;

        Ok(new_sticker)
    }

    pub fn get_all(
        conn: &mut SqliteConnection
    ) -> Result<Vec<Sticker>, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;
        let res = sticker.load(conn)?;
        Ok(res)
    }

    pub fn get_by_id(
        conn: &mut SqliteConnection,
        sticker_id: &String,
    ) -> Result<Sticker, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

        sticker.filter(id.eq(sticker_id))
            .first::<Sticker>(conn)
    }

    pub fn delete(
        conn: &mut SqliteConnection,
        sticker_id: &String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

        diesel::delete(sticker.filter(id.eq(sticker_id))).execute(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        data: StickerUpdate,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::sticker::dsl::*;

        diesel::update(sticker.filter(id.eq(&data.id)))
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
    pub id: String,
    pub label: String,
    pub url: String,
}

impl StickerUpdate {
    pub fn new(id: String, label: String, url: String) -> Self {
        StickerUpdate { id, label, url }
    }
}
