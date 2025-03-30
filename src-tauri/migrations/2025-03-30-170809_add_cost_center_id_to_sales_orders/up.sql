-- Your SQL goes here

-- Add cost_center_id column to sales_orders table
ALTER TABLE sales_orders
ADD COLUMN cost_center_id TEXT
REFERENCES cost_centers(id);

-- Add index for the foreign key
CREATE INDEX idx_sales_orders_cost_center_id ON sales_orders(cost_center_id);
