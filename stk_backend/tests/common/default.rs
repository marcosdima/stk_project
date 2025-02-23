#[allow(dead_code)] // I don't know why this happend...
pub fn get_sticker_default(n: u16) -> (String, String) {
    (
        format!("Sticker {n}"),
        format!("www.stk{n}.com"),
    )
}

#[allow(dead_code)] // I don't know why this happend...
pub fn get_category_default(n: u16) -> String {
    format!("Category {n}")
}