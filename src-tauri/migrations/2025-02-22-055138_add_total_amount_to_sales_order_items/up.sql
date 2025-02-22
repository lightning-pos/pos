-- Create a new table with the total_amount column
CREATE TABLE sales_order_items_new (
    id TEXT NOT NULL PRIMARY KEY,
    order_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
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

-- Copy data from old table to new table
INSERT INTO sales_order_items_new 
SELECT 
    id,
    order_id,
    item_id,
    item_name,
    quantity,
    price_amount,
    tax_amount,
    (price_amount + tax_amount) * quantity as total_amount,
    created_at,
    updated_at
FROM sales_order_items;

-- Drop the old table
DROP TABLE sales_order_items;

-- Rename the new table to the original name
ALTER TABLE sales_order_items_new RENAME TO sales_order_items;
