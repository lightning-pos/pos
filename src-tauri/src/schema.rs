use diesel::{joinable, table};

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::entities::catalog::item_category::ItemCategoryStateMapping;

    item_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        state -> ItemCategoryStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Integer, Nullable, Timestamp};
    use crate::core::entities::catalog::item::ItemNatureMapping;
    use crate::core::entities::catalog::item::ItemStateMapping;

    items (id) {
        id -> Text,
        category_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        nature -> ItemNatureMapping,
        state -> ItemStateMapping,
        price -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(items -> item_categories (category_id));
joinable!(item_taxes -> items (item_id));
joinable!(item_taxes -> taxes (tax_id));
