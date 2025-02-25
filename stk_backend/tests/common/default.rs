use stk_backend::models::{
    categories::NewCategory,
    stickers::NewSticker,
    tags::NewTag
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
