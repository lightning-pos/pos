-- This is a destructive change, so we can't really revert it properly
-- But we'll provide a basic structure to revert to the original schema

PRAGMA foreign_keys = OFF;

-- Recreate the original table with the foreign key constraints
CREATE TABLE sales_orders_old (
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
    location_id TEXT NOT NULL,
    cost_center_id TEXT NOT NULL,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
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

-- Copy data from the current table to the old table
-- Note: This might fail if there are records with NULL created_by or updated_by
INSERT INTO sales_orders_old
SELECT * FROM sales_orders;

-- Drop the current table
DROP TABLE sales_orders;

-- Rename the old table to the original name
ALTER TABLE sales_orders_old RENAME TO sales_orders;

-- Re-create indexes
CREATE INDEX idx_sales_orders_customer_id ON sales_orders(customer_id);
CREATE INDEX idx_sales_orders_channel_id ON sales_orders(channel_id);
CREATE INDEX idx_sales_orders_location_id ON sales_orders(location_id);
CREATE INDEX idx_sales_orders_cost_center_id ON sales_orders(cost_center_id);

-- Re-enable foreign key constraints
PRAGMA foreign_keys = ON;