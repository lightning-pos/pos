use diesel::{joinable, table};

table! {
    item_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        state -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    items (id) {
        id -> Text,
        category_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        nature -> Text,
        state -> Text,
        price -> Integer,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    item_taxes (item_id, tax_id) {
        item_id -> Text,
        tax_id -> Text,
    }
}

table! {
    taxes (id) {
        id -> Text,
        name -> Text,
        rate -> BigInt,
        description -> Nullable<Text>,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

joinable!(items -> item_categories (category_id));
joinable!(item_taxes -> items (item_id));
joinable!(item_taxes -> taxes (tax_id));
