// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Text,
        name -> Text,
        sub_category_of -> Nullable<Text>,
    }
}

diesel::table! {
    sticker (id) {
        id -> Text,
        label -> Text,
        url -> Text,
    }
}

diesel::table! {
    sticker_category (sticker_id, category_id) {
        sticker_id -> Text,
        category_id -> Text,
    }
}

diesel::table! {
    tag (name) {
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    category,
    sticker,
    sticker_category,
);
