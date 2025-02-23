use stk_backend::models::{categories::NewCategory, stickers::NewSticker};

#[allow(dead_code)] // I don't know why this happend...
pub fn get_sticker_default(n: u16) -> NewSticker {
    NewSticker::new(
        format!("Sticker {n}"),
        format!("www.stk{n}.com"),
    )
}

#[allow(dead_code)] // I don't know why this happend...
pub fn get_category_default(n: u16) -> NewCategory {
    NewCategory::new(format!("Category {n}"), None)
}