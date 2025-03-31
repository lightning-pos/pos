-- Consolidated initial schema generated from schema.rs

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
    customer_id TEXT NOT NULL,
    customer_name TEXT NOT NULL,
    customer_phone_number TEXT NOT NULL,
    order_date TIMESTAMP NOT NULL,
    net_amount BIGINT NOT NULL,
    disc_amount BIGINT NOT NULL,
    taxable_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    state TEXT NOT NULL, -- Corresponds to SalesOrderStateMapping
    cost_center_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE sales_order_items (
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    item_id TEXT, -- Nullable based on schema.rs and previous migration
    item_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    price_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE RESTRICT ON UPDATE CASCADE -- item_id can be NULL
);

-- Finance Tables
CREATE TABLE payment_methods (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    state TEXT NOT NULL, -- Corresponds to PaymentMethodStateMapping
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
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
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE, -- Cascade delete payments if order deleted
    FOREIGN KEY (payment_method_id) REFERENCES payment_methods(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Cart Table
CREATE TABLE carts (
    id TEXT PRIMARY KEY NOT NULL,
    customer_id TEXT, -- Nullable based on schema.rs
    cart_data TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE SET NULL ON UPDATE CASCADE -- Set null if customer deleted
);

-- Channel & Brand Tables
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

-- Add Indexes
CREATE INDEX idx_items_category_id ON items(category_id);
CREATE INDEX idx_sales_orders_customer_id ON sales_orders(customer_id);
CREATE INDEX idx_sales_orders_cost_center_id ON sales_orders(cost_center_id);
CREATE INDEX idx_sales_order_items_order_id ON sales_order_items(order_id);
CREATE INDEX idx_sales_order_items_item_id ON sales_order_items(item_id);
CREATE INDEX idx_expenses_category_id ON expenses(category_id);
CREATE INDEX idx_expenses_cost_center_id ON expenses(cost_center_id);
CREATE INDEX idx_sales_order_payments_order_id ON sales_order_payments(order_id);
CREATE INDEX idx_sales_order_payments_payment_method_id ON sales_order_payments(payment_method_id);
CREATE INDEX idx_carts_customer_id ON carts(customer_id);

-- Note: updated_at triggers are omitted as Diesel typically handles this at the application level.
