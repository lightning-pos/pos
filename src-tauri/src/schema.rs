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
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamp};
    use crate::core::entities::catalog::item::ItemNatureMapping;
    use crate::core::entities::catalog::item::ItemStateMapping;

    items (id) {
        id -> Text,
        category_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        nature -> ItemNatureMapping,
        state -> ItemStateMapping,
        price -> BigInt,
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

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::entities::auth::user::UserStateMapping;

    users (id) {
        id -> Text,
        username -> Text,
        pin_hash -> Text,
        full_name -> Text,
        state -> UserStateMapping,
        last_login_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    customers (id) {
        id -> Text,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        country_code -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    orders (id) {
        id -> Text,
        customer_id -> Text,
        customer_name -> Text,
        customer_phone_number -> Text,
        order_date -> BigInt,
        net_amount -> BigInt,
        disc_amount -> BigInt,
        taxable_amount -> BigInt,
        tax_amount -> BigInt,
        total_amount -> BigInt,
        state -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    order_items (id) {
        id -> Text,
        order_id -> Text,
        item_id -> Text,
        item_name -> Text,
        quantity -> BigInt,
        price_amount -> BigInt,
        tax_amount -> BigInt,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

// ManyToOne (items, item_categories)
joinable!(items -> item_categories (category_id));

// ManyToMany (items, item_taxes)
joinable!(item_taxes -> items (item_id));
joinable!(item_taxes -> taxes (tax_id));

// ManyToOne (orders, customers)
joinable!(orders -> customers (customer_id));

// ManyToMany (orders, order_items)
joinable!(order_items -> orders (order_id));
joinable!(order_items -> items (item_id));
