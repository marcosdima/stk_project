mod common;

#[cfg(test)]
mod artists;

#[cfg(test)]
mod categories;

#[cfg(test)]
mod stickers;

#[cfg(test)]
mod tags;

#[cfg(test)]
mod users;

#[cfg(test)]
mod utils;

pub use actix_web::{
    http::{
        header::{
            ContentType,
            AUTHORIZATION,
        },
        Method
    },
    web::Bytes,
    test,
    web,
    App,
};

pub use stk_backend::{
    models::{
        categories::*,
        sticker_category::*,
        stickers::*,
        tags::*,
        sticker_tag::*,
        artists::*,
        artist_sticker::*,
        users::*,
        BasicModel,
        Model,
    },
    routes::DbPool,
    utils::generate_token,
    errors::AppError,
};

pub use crate::common::{
    default::{
        self,
        *
    },
    admin::*,
    requests::*,
    get_element,
    get_app,
    get_just_app,
    get_just_pool,
    expect_error,
};

pub use uuid::Uuid;
