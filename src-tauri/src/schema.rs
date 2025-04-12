use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};

    variant_types (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Integer, Timestamp};

    variant_values (id) {
        id -> Text,
        variant_type_id -> Text,
        value -> Text,
        display_order -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp, BigInt, Bool};

    item_variants (id) {
        id -> Text,
        item_id -> Text,
        sku -> Nullable<Text>,
        price_adjustment -> Nullable<BigInt>,
        is_default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::Text;

    item_variant_values (item_variant_id, variant_value_id) {
        item_variant_id -> Text,
        variant_value_id -> Text,
    }
}

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
        category_id -> Text,
        cost_center_id -> Text,
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
    use crate::core::models::sales::sales_order_model::SalesOrderPaymentStateMapping;

    sales_orders (id) {
        id -> Text,
        order_readable_id -> Text, // Human readable ID
        order_date -> Timestamp,

        // Customer
        customer_id -> Nullable<Text>,
        customer_name -> Nullable<Text>,
        customer_phone_number -> Nullable<Text>,
        billing_address -> Nullable<Text>,
        shipping_address -> Nullable<Text>,

        // Amounts
        net_amount -> BigInt,
        disc_amount -> BigInt,
        taxable_amount -> BigInt,
        tax_amount -> BigInt,
        total_amount -> BigInt,

        // State
        order_state -> SalesOrderStateMapping,
        payment_state -> SalesOrderPaymentStateMapping,

        // Notes
        notes -> Nullable<Text>,

        // Mappings
        channel_id -> Text, // For channel-wise reporting (Eg. Dine-in, Take-away, etc.)
        location_id -> Text, // For inventory tracking
        cost_center_id -> Text, // For accounting
        created_by -> Text, // User ID for audit trail
        updated_by -> Text, // User ID for audit trail

        // Optional Mappings
        discount_id -> Nullable<Text>, // For discount tracking

        // Timestamps
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sales_order_items (id) {
        id -> Text,

        // Item
        item_id -> Nullable<Text>,
        item_name -> Text,
        quantity -> Integer,
        sku -> Nullable<Text>,

        // Amounts
        price_amount -> BigInt,
        disc_amount -> BigInt,
        taxable_amount -> BigInt,
        tax_amount -> BigInt,
        total_amount -> BigInt,

        // Mappings
        order_id -> Text,

        // Timestamps
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sales_order_charges (id) {
        id -> Text,

        // Charge
        charge_type_id -> Text,
        charge_type_name -> Text,

        // Amounts
        amount -> BigInt,
        tax_amount -> BigInt,

        // Mappings
        order_id -> Text,
        tax_group_id -> Nullable<Text>,

        // Timestamps
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sales_charge_types (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamp};
    use crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentStateMapping;

    sales_order_payments (id) {
        id -> Text,
        order_id -> Text,
        payment_method_id -> Text,
        payment_date -> Timestamp,
        amount -> BigInt,
        reference_number -> Nullable<Text>,
        notes -> Nullable<Text>,
        state -> SalesOrderPaymentStateMapping,
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

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp, Bool};

    channels (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp, Bool};

    brands (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp, Bool};

    locations (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        address -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::models::finance::cost_center_model::CostCenterStateMapping;

    cost_centers (id) {
        id -> Text,
        name -> Text,
        code -> Text,
        description -> Nullable<Text>,
        state -> CostCenterStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};
    use crate::core::models::finance::payment_method_model::PaymentMethodStateMapping;

    payment_methods (id) {
        id -> Text,
        name -> Text,
        code -> Text,
        description -> Nullable<Text>,
        state -> PaymentMethodStateMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamp};
    use crate::core::models::catalog::discount_model::DiscountStateMapping;
    use crate::core::models::catalog::discount_model::DiscountTypeMapping;
    use crate::core::models::catalog::discount_model::DiscountScopeMapping;

    discounts (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        discount_type -> DiscountTypeMapping,
        value -> BigInt,
        scope -> DiscountScopeMapping,
        state -> DiscountStateMapping,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// ManyToOne (items, item_categories)
joinable!(items -> item_categories (category_id));

// ManyToMany (items, item_taxes)
joinable!(item_taxes -> items (item_id));
joinable!(item_taxes -> taxes (tax_id));

// ManyToOne (variant_values, variant_types)
joinable!(variant_values -> variant_types (variant_type_id));

// ManyToOne (item_variants, items)
joinable!(item_variants -> items (item_id));

// ManyToMany (item_variants, variant_values)
joinable!(item_variant_values -> item_variants (item_variant_id));
joinable!(item_variant_values -> variant_values (variant_value_id));

// ManyToOne (orders, customers)
joinable!(sales_orders -> customers (customer_id));

// ManyToMany (orders, order_items)
joinable!(sales_order_items -> sales_orders (order_id));
joinable!(sales_order_items -> items (item_id));

// ManyToMany (orders, order_charges)
joinable!(sales_order_charges -> sales_orders (order_id));
joinable!(sales_order_charges -> sales_charge_types (charge_type_id));

// ManyToOne (expenses, purchase_categories)
joinable!(expenses -> purchase_categories (category_id));

// ManyToOne (expenses, cost_centers)
joinable!(expenses -> cost_centers (cost_center_id));

// ManyToOne (orders, cost_centers)
joinable!(sales_orders -> cost_centers (cost_center_id));

// ManyToOne (orders, locations)
joinable!(sales_orders -> locations (location_id));

// ManyToOne (sales_order_payments, sales_orders)
joinable!(sales_order_payments -> sales_orders (order_id));

// ManyToOne (sales_order_payments, payment_methods)
joinable!(sales_order_payments -> payment_methods (payment_method_id));

table! {
    use diesel::sql_types::{Text, Nullable, Timestamp};

    tax_groups (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tax_group_taxes (tax_group_id, tax_id) {
        tax_group_id -> Text,
        tax_id -> Text,
    }
}

table! {
    use diesel::sql_types::Text;

    item_discounts (item_id, discount_id) {
        item_id -> Text,
        discount_id -> Text,
    }
}

// ManyToMany (tax_groups, taxes)
joinable!(tax_group_taxes -> tax_groups (tax_group_id));
joinable!(tax_group_taxes -> taxes (tax_id));

// ManyToOne (sales_order_charges, tax_groups)
joinable!(sales_order_charges -> tax_groups (tax_group_id));

// ManyToMany (items, discounts)
joinable!(item_discounts -> items (item_id));
joinable!(item_discounts -> discounts (discount_id));

allow_tables_to_appear_in_same_query!(
    items,
    item_categories,
    item_taxes,
    taxes,
    variant_types,
    variant_values,
    item_variants,
    item_variant_values,
    sales_orders,
    customers,
    sales_order_items,
    sales_order_charges,
    sales_charge_types,
    expenses,
    purchase_categories,
    cost_centers,
    sales_order_payments,
    payment_methods,
    tax_groups,
    tax_group_taxes,
    discounts,
    item_discounts,
    locations
);
