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
    sticker_tag (tag_name, sticker_id) {
        tag_name -> Text,
        sticker_id -> Text,
    }
}

diesel::table! {
    tag (name) {
        name -> Text,
    }
}

diesel::joinable!(sticker_category -> category (category_id));
diesel::joinable!(sticker_category -> sticker (sticker_id));
diesel::joinable!(sticker_tag -> sticker (sticker_id));
diesel::joinable!(sticker_tag -> tag (tag_name));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    sticker,
    sticker_category,
    sticker_tag,
    tag,
);
