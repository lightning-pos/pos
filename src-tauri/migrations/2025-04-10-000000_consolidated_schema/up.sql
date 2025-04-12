-- Consolidated schema migration that includes all tables from schema.rs

-- Drop all existing tables if they exist
DROP TABLE IF EXISTS item_discounts;
DROP TABLE IF EXISTS tax_group_taxes;
DROP TABLE IF EXISTS item_variant_values;
DROP TABLE IF EXISTS item_taxes;
DROP TABLE IF EXISTS sales_order_charges;
DROP TABLE IF EXISTS sales_order_items;
DROP TABLE IF EXISTS sales_order_payments;
DROP TABLE IF EXISTS sales_orders;
DROP TABLE IF EXISTS expenses;
DROP TABLE IF EXISTS carts;
DROP TABLE IF EXISTS item_variants;
DROP TABLE IF EXISTS variant_values;
DROP TABLE IF EXISTS variant_types;
DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS discounts;
DROP TABLE IF EXISTS item_categories;
DROP TABLE IF EXISTS taxes;
DROP TABLE IF EXISTS tax_groups;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS suppliers;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS purchase_categories;
DROP TABLE IF EXISTS cost_centers;
DROP TABLE IF EXISTS payment_methods;
DROP TABLE IF EXISTS sales_charge_types;
DROP TABLE IF EXISTS channels;
DROP TABLE IF EXISTS brands;
DROP TABLE IF EXISTS locations;

-- Create all tables in the correct order (respecting foreign key constraints)

-- Catalog Tables
CREATE TABLE item_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL, -- Corresponds to ItemGroupStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE taxes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rate INTEGER NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tax_groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tax_group_taxes (
    tax_group_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (tax_group_id, tax_id),
    FOREIGN KEY (tax_group_id) REFERENCES tax_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes(id) ON DELETE CASCADE
);

CREATE TABLE variant_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE variant_values (
    id TEXT PRIMARY KEY NOT NULL,
    variant_type_id TEXT NOT NULL,
    value TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (variant_type_id) REFERENCES variant_types(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE items (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    nature TEXT NOT NULL, -- Corresponds to ItemNatureMapping
    state TEXT NOT NULL, -- Corresponds to ItemStateMapping
    price BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES item_categories(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE item_taxes (
    item_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (item_id, tax_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE item_variants (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    sku TEXT,
    price_adjustment BIGINT,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE item_variant_values (
    item_variant_id TEXT NOT NULL,
    variant_value_id TEXT NOT NULL,
    PRIMARY KEY (item_variant_id, variant_value_id),
    FOREIGN KEY (item_variant_id) REFERENCES item_variants(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (variant_value_id) REFERENCES variant_values(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE discounts (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    discount_type TEXT NOT NULL CHECK(discount_type IN ('percentage', 'fixed_amount')),
    value BIGINT NOT NULL,
    scope TEXT NOT NULL CHECK(scope IN ('all_items', 'specific_items')),
    state TEXT NOT NULL DEFAULT 'active' CHECK(state IN ('active', 'inactive', 'scheduled', 'expired')),
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE item_discounts (
    item_id TEXT NOT NULL,
    discount_id TEXT NOT NULL,
    PRIMARY KEY (item_id, discount_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (discount_id) REFERENCES discounts(id) ON DELETE CASCADE ON UPDATE CASCADE
);

-- Auth Tables
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    pin_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    state TEXT NOT NULL, -- Corresponds to UserStateMapping
    last_login_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Customer Tables
CREATE TABLE customers (
    id TEXT PRIMARY KEY NOT NULL,
    full_name TEXT NOT NULL,
    email TEXT UNIQUE,
    phone TEXT UNIQUE,
    address TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Supplier Tables
CREATE TABLE suppliers (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Purchase & Expense Tables
CREATE TABLE purchase_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    state TEXT NOT NULL, -- Corresponds to PurchaseCategoryStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cost_centers (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    state TEXT NOT NULL, -- Corresponds to CostCenterStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Channel, Brand, and Location Tables
CREATE TABLE channels (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1, -- Use INTEGER 1 for TRUE in SQLite
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE brands (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1, -- Use INTEGER 1 for TRUE in SQLite
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE locations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    address TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1, -- Use INTEGER 1 for TRUE in SQLite
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Sales Charge Types
CREATE TABLE sales_charge_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Payment Methods
CREATE TABLE payment_methods (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    state TEXT NOT NULL, -- Corresponds to PaymentMethodStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Expenses
CREATE TABLE expenses (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expense_date TIMESTAMP NOT NULL,
    category_id TEXT NOT NULL,
    cost_center_id TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES purchase_categories(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Sales Tables
CREATE TABLE sales_orders (
    id TEXT PRIMARY KEY NOT NULL,
    order_readable_id TEXT NOT NULL, -- Human readable ID
    order_date TIMESTAMP NOT NULL,

    -- Customer
    customer_id TEXT,
    customer_name TEXT,
    customer_phone_number TEXT,
    billing_address TEXT,
    shipping_address TEXT,

    -- Amounts
    net_amount BIGINT NOT NULL,
    disc_amount BIGINT NOT NULL,
    taxable_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,

    -- State
    order_state TEXT NOT NULL, -- Corresponds to SalesOrderStateMapping
    payment_state TEXT NOT NULL, -- Corresponds to SalesOrderPaymentStateMapping

    -- Notes
    notes TEXT,

    -- Mappings
    channel_id TEXT NOT NULL,
    location_id TEXT NOT NULL, -- For inventory tracking
    cost_center_id TEXT NOT NULL,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,

    -- Optional Mappings
    discount_id TEXT,

    -- Timestamps
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (discount_id) REFERENCES discounts(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE sales_order_items (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT,
    item_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    sku TEXT,
    price_amount BIGINT NOT NULL,
    disc_amount BIGINT NOT NULL,
    taxable_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    order_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE sales_order_charges (
    id TEXT PRIMARY KEY NOT NULL,
    charge_type_id TEXT NOT NULL,
    charge_type_name TEXT NOT NULL,
    amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    order_id TEXT NOT NULL,
    tax_group_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (charge_type_id) REFERENCES sales_charge_types(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (tax_group_id) REFERENCES tax_groups(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE sales_order_payments (
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    payment_method_id TEXT NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    amount BIGINT NOT NULL,
    reference_number TEXT,
    notes TEXT,
    state TEXT NOT NULL, -- Corresponds to SalesOrderPaymentStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (payment_method_id) REFERENCES payment_methods(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Cart Table
CREATE TABLE carts (
    id TEXT PRIMARY KEY NOT NULL,
    customer_id TEXT,
    cart_data TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE SET NULL ON UPDATE CASCADE
);

-- Add Indexes for better performance
CREATE INDEX idx_items_category_id ON items(category_id);
CREATE INDEX idx_variant_values_variant_type_id ON variant_values(variant_type_id);
CREATE INDEX idx_item_variants_item_id ON item_variants(item_id);
CREATE INDEX idx_sales_orders_customer_id ON sales_orders(customer_id);
CREATE INDEX idx_sales_orders_channel_id ON sales_orders(channel_id);
CREATE INDEX idx_sales_orders_location_id ON sales_orders(location_id);
CREATE INDEX idx_sales_orders_cost_center_id ON sales_orders(cost_center_id);
CREATE INDEX idx_sales_order_items_order_id ON sales_order_items(order_id);
CREATE INDEX idx_sales_order_items_item_id ON sales_order_items(item_id);
CREATE INDEX idx_sales_order_charges_order_id ON sales_order_charges(order_id);
CREATE INDEX idx_sales_order_charges_charge_type_id ON sales_order_charges(charge_type_id);
CREATE INDEX idx_expenses_category_id ON expenses(category_id);
CREATE INDEX idx_expenses_cost_center_id ON expenses(cost_center_id);
CREATE INDEX idx_sales_order_payments_order_id ON sales_order_payments(order_id);
CREATE INDEX idx_sales_order_payments_payment_method_id ON sales_order_payments(payment_method_id);
CREATE INDEX idx_carts_customer_id ON carts(customer_id);
CREATE INDEX idx_tax_group_taxes_tax_group_id ON tax_group_taxes(tax_group_id);
CREATE INDEX idx_tax_group_taxes_tax_id ON tax_group_taxes(tax_id);
CREATE INDEX idx_item_discounts_item_id ON item_discounts(item_id);
CREATE INDEX idx_item_discounts_discount_id ON item_discounts(discount_id);

-- Create triggers for updated_at timestamps
CREATE TRIGGER update_item_categories_updated_at
AFTER UPDATE ON item_categories
FOR EACH ROW
BEGIN
    UPDATE item_categories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_taxes_updated_at
AFTER UPDATE ON taxes
FOR EACH ROW
BEGIN
    UPDATE taxes SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_tax_groups_updated_at
AFTER UPDATE ON tax_groups
FOR EACH ROW
BEGIN
    UPDATE tax_groups SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_variant_types_updated_at
AFTER UPDATE ON variant_types
FOR EACH ROW
BEGIN
    UPDATE variant_types SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_variant_values_updated_at
AFTER UPDATE ON variant_values
FOR EACH ROW
BEGIN
    UPDATE variant_values SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_items_updated_at
AFTER UPDATE ON items
FOR EACH ROW
BEGIN
    UPDATE items SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_item_variants_updated_at
AFTER UPDATE ON item_variants
FOR EACH ROW
BEGIN
    UPDATE item_variants SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_discounts_updated_at
AFTER UPDATE ON discounts
FOR EACH ROW
BEGIN
    UPDATE discounts SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_customers_updated_at
AFTER UPDATE ON customers
FOR EACH ROW
BEGIN
    UPDATE customers SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_suppliers_updated_at
AFTER UPDATE ON suppliers
FOR EACH ROW
BEGIN
    UPDATE suppliers SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_purchase_categories_updated_at
AFTER UPDATE ON purchase_categories
FOR EACH ROW
BEGIN
    UPDATE purchase_categories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_cost_centers_updated_at
AFTER UPDATE ON cost_centers
FOR EACH ROW
BEGIN
    UPDATE cost_centers SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_channels_updated_at
AFTER UPDATE ON channels
FOR EACH ROW
BEGIN
    UPDATE channels SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_brands_updated_at
AFTER UPDATE ON brands
FOR EACH ROW
BEGIN
    UPDATE brands SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_locations_updated_at
AFTER UPDATE ON locations
FOR EACH ROW
BEGIN
    UPDATE locations SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_sales_charge_types_updated_at
AFTER UPDATE ON sales_charge_types
FOR EACH ROW
BEGIN
    UPDATE sales_charge_types SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_payment_methods_updated_at
AFTER UPDATE ON payment_methods
FOR EACH ROW
BEGIN
    UPDATE payment_methods SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_expenses_updated_at
AFTER UPDATE ON expenses
FOR EACH ROW
BEGIN
    UPDATE expenses SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_sales_orders_updated_at
AFTER UPDATE ON sales_orders
FOR EACH ROW
BEGIN
    UPDATE sales_orders SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_sales_order_items_updated_at
AFTER UPDATE ON sales_order_items
FOR EACH ROW
BEGIN
    UPDATE sales_order_items SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_sales_order_charges_updated_at
AFTER UPDATE ON sales_order_charges
FOR EACH ROW
BEGIN
    UPDATE sales_order_charges SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_sales_order_payments_updated_at
AFTER UPDATE ON sales_order_payments
FOR EACH ROW
BEGIN
    UPDATE sales_order_payments SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_carts_updated_at
AFTER UPDATE ON carts
FOR EACH ROW
BEGIN
    UPDATE carts SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;