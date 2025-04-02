-- Your SQL goes here
CREATE TABLE sales_order_charges (
    id TEXT PRIMARY KEY NOT NULL,
    charge_type_id TEXT NOT NULL,
    charge_type_name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL,
    order_id TEXT NOT NULL,
    tax_group_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Re-enabled Foreign Keys
    FOREIGN KEY (order_id) REFERENCES sales_orders(id),
    FOREIGN KEY (charge_type_id) REFERENCES sales_charge_types(id),
    FOREIGN KEY (tax_group_id) REFERENCES tax_groups(id) -- Assuming tax_groups table exists
);
