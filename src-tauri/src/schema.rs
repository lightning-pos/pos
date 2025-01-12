diesel::table! {
    item_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        state -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::table! {
    items (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        nature -> Text,
        state -> Text,
        category_id -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::joinable!(items -> item_categories (category_id));
