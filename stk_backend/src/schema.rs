// @generated automatically by Diesel CLI.

diesel::table! {
    artist (id) {
        id -> Text,
        name -> Text,
        logo_url -> Text,
        presentation -> Nullable<Text>,
    }
}

diesel::table! {
    artist_sticker (artist_id, sticker_id) {
        artist_id -> Text,
        sticker_id -> Text,
    }
}

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
    sticker_tag (tag_id, sticker_id) {
        tag_id -> Text,
        sticker_id -> Text,
    }
}

diesel::table! {
    tag (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        name -> Text,
        lastname -> Text,
        username -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(artist_sticker -> artist (artist_id));
diesel::joinable!(artist_sticker -> sticker (sticker_id));
diesel::joinable!(sticker_category -> category (category_id));
diesel::joinable!(sticker_category -> sticker (sticker_id));
diesel::joinable!(sticker_tag -> sticker (sticker_id));
diesel::joinable!(sticker_tag -> tag (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    artist,
    artist_sticker,
    category,
    sticker,
    sticker_category,
    sticker_tag,
    tag,
    user,
);
