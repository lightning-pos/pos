-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
-- Create a new table with the updated schema
CREATE TABLE sales_order_items_new (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL,
    item_id TEXT,  -- Now nullable
    item_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    price_amount BIGINT NOT NULL,
    tax_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id),
    FOREIGN KEY (item_id) REFERENCES items(id)
);

-- Copy data from the old table to the new table
INSERT INTO sales_order_items_new
SELECT * FROM sales_order_items;

-- Drop the old table
DROP TABLE sales_order_items;

-- Rename the new table to the original name
ALTER TABLE sales_order_items_new RENAME TO sales_order_items;

-- Create indexes for the foreign keys
CREATE INDEX idx_sales_order_items_order_id ON sales_order_items(order_id);
CREATE INDEX idx_sales_order_items_item_id ON sales_order_items(item_id);
