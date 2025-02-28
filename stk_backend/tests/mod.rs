mod common;

#[cfg(test)]
mod artists;

#[cfg(test)]
mod categories;

#[cfg(test)]
mod stickers;

#[cfg(test)]
mod tags;

pub use actix_web::{
    http::{
        header::ContentType,
        Method
    },
    test,
    web,
    App,
};

pub use stk_backend::{
    models::{
        categories::{
            Category,
            CategoryUpdate,
        },
        sticker_category::{
            NewStickerCategory,
            StickerCategory,
        },
        stickers::{
            Sticker,
            StickerUpdate,
        },
        tags::{
            Tag,
            TagUpdate,
        },
        sticker_tag::{
            StickerTag,
            NewStickerTag,
        },
        artists:: {
            ArtistUpdate,
            Artist,
            NewArtist,
        },
        BasicModel,
        Model
    },
    routes::DbPool
};

pub use crate::common::{
    default::{
        self,
        get_category_default,
        get_sticker_default,
        get_tag_default,
        get_artist_default,
    },
    get_element,
    get_app
};

pub use uuid::Uuid;
