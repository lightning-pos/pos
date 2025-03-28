use diesel::{joinable, table};

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::models::catalog::item_group_model::ItemGroupStateMapping;

    item_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        state -> ItemGroupStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamp};
    use crate::core::models::catalog::item_model::ItemNatureMapping;
    use crate::core::models::catalog::item_model::ItemStateMapping;

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
    use diesel::sql_types::{Text, Nullable, Timestamp, Integer};

    taxes (id) {
        id -> Text,
        name -> Text,
        rate -> Integer,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::models::auth::user_model::UserStateMapping;

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
        full_name -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    suppliers (id) {
        id -> Text,
        name -> Text,
        address -> Nullable<Text>,
        phone -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, BigInt, Nullable, Timestamp};

    expenses (id) {
        id -> Text,
        title -> Text,
        amount -> BigInt,
        expense_date -> Timestamp,
        category -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::models::purchases::purchase_category_model::PurchaseCategoryStateMapping;

    purchase_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        state -> PurchaseCategoryStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamp};
    use crate::core::models::sales::sales_order_model::SalesOrderStateMapping;

    sales_orders (id) {
        id -> Text,
        customer_id -> Text,
        customer_name -> Text,
        customer_phone_number -> Text,
        order_date -> Timestamp,
        net_amount -> BigInt,
        disc_amount -> BigInt,
        taxable_amount -> BigInt,
        tax_amount -> BigInt,
        total_amount -> BigInt,
        state -> SalesOrderStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sales_order_items (id) {
        id -> Text,
        order_id -> Text,
        item_id -> Text,
        item_name -> Text,
        quantity -> Integer,
        price_amount -> BigInt,
        tax_amount -> BigInt,
        total_amount -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Nullable, Text, Timestamp};

    carts (id) {
        id -> Text,
        customer_id -> Nullable<Text>,
        cart_data -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// ManyToOne (items, item_categories)
joinable!(items -> item_categories (category_id));

// ManyToMany (items, item_taxes)
joinable!(item_taxes -> items (item_id));
joinable!(item_taxes -> taxes (tax_id));

// ManyToOne (orders, customers)
joinable!(sales_orders -> customers (customer_id));

// ManyToMany (orders, order_items)
joinable!(sales_order_items -> sales_orders (order_id));
joinable!(sales_order_items -> items (item_id));
