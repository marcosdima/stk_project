use stk_backend::models::{
    artists::NewArtist,
    categories::NewCategory,
    stickers::NewSticker,
    tags::NewTag,
    users::NewUser,
};

pub fn get_sticker_default(n: u16) -> NewSticker {
    NewSticker::new(
        format!("Sticker {n}"),
        format!("www.stk{n}.com"),
    )
}

pub fn get_category_default(n: u16) -> NewCategory {
    NewCategory::new(format!("Category {n}"), None)
}

pub fn get_tag_default(n: u16) -> NewTag {
    NewTag::new(format!("Tag {n}"))
}

pub fn get_artist_default(n: u16) -> NewArtist {
    NewArtist::new(
        format!("Artist name {n}"),
        format!("www.artist-{n}.com"),
        Some(format!("This is artist {n}")),
    )
}

pub fn get_user_default(n: u16) -> NewUser {
    NewUser::new(
        format!("User name {n}"),
        format!("Lastname {n}"),
        format!("Username {n}"),
        format!("Pass {n}"),
    )
}
