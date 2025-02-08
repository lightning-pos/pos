-- Create item_categories table
CREATE TABLE item_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create taxes table
CREATE TABLE taxes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rate INTEGER NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create items table
CREATE TABLE items (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    nature TEXT NOT NULL,
    state TEXT NOT NULL,
    price INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (category_id) REFERENCES item_categories(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Create item_taxes table
CREATE TABLE item_taxes (
    item_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (item_id, tax_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- Create users table
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    pin_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    state TEXT NOT NULL,
    last_login_at TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create customer table
CREATE TABLE customers (
    id TEXT PRIMARY KEY NOT NULL,
    full_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create carts table
CREATE TABLE carts (
    id TEXT PRIMARY KEY NOT NULL,
    customer_id TEXT NULL,
    cart_data TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE SET NULL ON UPDATE CASCADE
);

-- Create sales_orders table
CREATE TABLE sales_orders (
    id TEXT PRIMARY KEY NOT NULL,
    customer_id TEXT NOT NULL,
    customer_name TEXT NOT NULL,
    customer_phone_number TEXT NOT NULL,
    order_date TIMESTAMP NOT NULL,
    net_amount INTEGER NOT NULL,
    disc_amount INTEGER NOT NULL,
    taxable_amount INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL,
    total_amount INTEGER NOT NULL,
    state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Create sales_order_items table
CREATE TABLE sales_order_items (
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    quantity BIGINT NOT NULL,
    price_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE RESTRICT ON UPDATE CASCADE
);
