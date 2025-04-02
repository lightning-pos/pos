-- This file should undo anything in `up.sql`

-- Recreate the old table structure
CREATE TABLE sales_order_items_old (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT,
    item_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    price_amount INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL,
    total_amount INTEGER NOT NULL,
    order_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id),
    FOREIGN KEY (item_id) REFERENCES items(id)
);

-- Copy data back, losing the new columns
INSERT INTO sales_order_items_old (
    id, item_id, item_name, quantity, price_amount, tax_amount, total_amount, order_id, created_at, updated_at
)
SELECT
    id, item_id, item_name, quantity, price_amount, tax_amount, total_amount, order_id, created_at, updated_at
FROM sales_order_items;

-- Drop the new table
DROP TABLE sales_order_items;

-- Rename the old table back
ALTER TABLE sales_order_items_old RENAME TO sales_order_items;
