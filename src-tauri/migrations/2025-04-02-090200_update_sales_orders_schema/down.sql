-- This file should undo anything in `up.sql`

-- Recreate the old table structure
CREATE TABLE sales_orders_old (
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
    cost_center_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id)
);

-- Copy data back from the new table to the old structure
-- Note: We lose the new columns (readable_id, billing/shipping, notes, channel, location, user, discount)
-- Map order_state back to state. Payment state is lost.
INSERT INTO sales_orders_old (
    id, customer_id, customer_name, customer_phone_number, order_date,
    net_amount, disc_amount, taxable_amount, tax_amount, total_amount,
    state, cost_center_id, created_at, updated_at
)
SELECT
    id, customer_id, customer_name, customer_phone_number, order_date,
    net_amount, disc_amount, taxable_amount, tax_amount, total_amount,
    CASE order_state WHEN 'Completed' THEN 'Completed' WHEN 'Cancelled' THEN 'Cancelled' ELSE 'Draft' END, -- Map order_state back
    cost_center_id, created_at, updated_at
FROM sales_orders;

-- Drop the new table
DROP TABLE sales_orders;

-- Rename the old table back
ALTER TABLE sales_orders_old RENAME TO sales_orders;

-- Add old indexes if they existed
-- CREATE INDEX idx_sales_orders_customer_id ON sales_orders (customer_id);
-- CREATE INDEX idx_sales_orders_order_date ON sales_orders (order_date);
