use stk_backend::models::{categories::NewCategory, stickers::NewSticker};

pub fn get_sticker_default(n: u16) -> NewSticker {
    NewSticker::new(
        format!("Sticker {n}"),
        format!("www.stk{n}.com"),
    )
}

pub fn get_category_default(n: u16) -> NewCategory {
    NewCategory::new(format!("Category {n}"), None)
}