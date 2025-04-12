-- Make created_by and updated_by nullable again
PRAGMA foreign_keys = OFF;

-- Create a new table with nullable fields
CREATE TABLE sales_orders_new (
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
    created_by TEXT, -- Changed back to nullable
    updated_by TEXT, -- Changed back to nullable
    discount_id TEXT,

    -- Timestamps
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (discount_id) REFERENCES discounts(id) ON DELETE RESTRICT ON UPDATE CASCADE
    -- Removed foreign key constraints for created_by and updated_by
);

-- Copy data from the old table to the new table
INSERT INTO sales_orders_new
SELECT * FROM sales_orders;

-- Drop the old table
DROP TABLE sales_orders;

-- Rename the new table to the original name
ALTER TABLE sales_orders_new RENAME TO sales_orders;

-- Re-create indexes
CREATE INDEX idx_sales_orders_customer_id ON sales_orders(customer_id);
CREATE INDEX idx_sales_orders_channel_id ON sales_orders(channel_id);
CREATE INDEX idx_sales_orders_location_id ON sales_orders(location_id);
CREATE INDEX idx_sales_orders_cost_center_id ON sales_orders(cost_center_id);

-- Re-enable foreign key constraints
PRAGMA foreign_keys = ON;