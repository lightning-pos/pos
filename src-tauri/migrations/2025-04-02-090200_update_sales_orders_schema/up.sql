-- Your SQL goes here

-- Create a new table with the desired schema
CREATE TABLE sales_orders_new (
    id TEXT PRIMARY KEY NOT NULL,
    order_readable_id TEXT NOT NULL, -- Added
    order_date TIMESTAMP NOT NULL,

    -- Customer (Made Optional)
    customer_id TEXT,
    customer_name TEXT,
    customer_phone_number TEXT,
    billing_address TEXT, -- Added
    shipping_address TEXT, -- Added

    -- Amounts (Assuming BigInt maps to INTEGER for SQLite)
    net_amount INTEGER NOT NULL,
    disc_amount INTEGER NOT NULL,
    taxable_amount INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL,
    total_amount INTEGER NOT NULL,

    -- State (Renamed state -> order_state, added payment_state)
    order_state TEXT NOT NULL,
    payment_state TEXT NOT NULL,

    -- Notes (Added)
    notes TEXT,

    -- Mappings (Added channel_id, location_id, created_by, updated_by)
    channel_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    cost_center_id TEXT NOT NULL,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,

    -- Optional Mappings (Added discount_id, made nullable)
    discount_id TEXT,

    -- Timestamps
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Re-enabled Foreign Keys
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id),
    FOREIGN KEY (channel_id) REFERENCES channels(id),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    FOREIGN KEY (discount_id) REFERENCES discounts(id)
);

-- Copy data from the old table to the new table
-- Note: We need default values or logic for the new NOT NULL columns
--       For readable_id, generate something unique or use id initially.
--       For channel_id, location_id, created_by, updated_by, use a placeholder or default.
--       Map old `state` to new `order_state` and `payment_state`.
INSERT INTO sales_orders_new (
    id, order_readable_id, order_date,
    customer_id, customer_name, customer_phone_number,
    net_amount, disc_amount, taxable_amount, tax_amount, total_amount,
    order_state, payment_state,
    channel_id, location_id, cost_center_id, created_by, updated_by,
    created_at, updated_at
    -- billing_address, shipping_address, notes, discount_id are new and nullable, default to NULL
)
SELECT
    id, id, order_date, -- Using id as placeholder for order_readable_id initially
    customer_id, customer_name, customer_phone_number,
    net_amount, disc_amount, taxable_amount, tax_amount, total_amount,
    CASE state WHEN 'Completed' THEN 'Completed' WHEN 'Cancelled' THEN 'Cancelled' ELSE 'Draft' END, -- Map old state to order_state
    CASE state WHEN 'Completed' THEN 'Pending' WHEN 'Cancelled' THEN 'Voided' ELSE 'Pending' END, -- Map old state to payment_state
    'placeholder_channel', 'placeholder_location', cost_center_id, 'placeholder_user', 'placeholder_user', -- Placeholders for new NOT NULL fields
    created_at, updated_at
FROM sales_orders;

-- Drop the old table
DROP TABLE sales_orders;

-- Rename the new table to the original name
ALTER TABLE sales_orders_new RENAME TO sales_orders;

-- Add indexes if they existed on the old table
-- CREATE INDEX idx_sales_orders_customer_id ON sales_orders (customer_id);
-- CREATE INDEX idx_sales_orders_order_date ON sales_orders (order_date);

-- Note: Placeholder values for channel, location, user need actual IDs from respective tables or a default.
--       This migration might fail if foreign key constraints aren't met by placeholders.
--       Consider adding default values or making these columns nullable temporarily.
