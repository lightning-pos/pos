-- Your SQL goes here

-- Create a new table with the added columns
CREATE TABLE sales_order_items_new (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT,
    item_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    sku TEXT, -- Added
    price_amount INTEGER NOT NULL,
    disc_amount INTEGER NOT NULL, -- Added
    taxable_amount INTEGER NOT NULL, -- Added
    tax_amount INTEGER NOT NULL,
    total_amount INTEGER NOT NULL,
    order_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id),
    FOREIGN KEY (item_id) REFERENCES items(id)
);

-- Copy data from the old table, providing default values for new columns
INSERT INTO sales_order_items_new (
    id, item_id, item_name, quantity,
    sku, -- Default NULL
    price_amount,
    disc_amount, -- Default 0
    taxable_amount, -- Need calculation logic (e.g., price_amount - disc_amount)
    tax_amount, total_amount, order_id, created_at, updated_at
)
SELECT
    id, item_id, item_name, quantity,
    NULL, -- Default for sku
    price_amount,
    0, -- Default for disc_amount
    price_amount, -- Default for taxable_amount (assuming disc_amount was 0)
    tax_amount, total_amount, order_id, created_at, updated_at
FROM sales_order_items;

-- Drop the old table
DROP TABLE sales_order_items;

-- Rename the new table
ALTER TABLE sales_order_items_new RENAME TO sales_order_items;
