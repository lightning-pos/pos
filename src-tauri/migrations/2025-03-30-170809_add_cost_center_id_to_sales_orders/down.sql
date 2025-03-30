-- This file should undo anything in `up.sql`

-- Drop the index first
DROP INDEX IF EXISTS idx_sales_orders_cost_center_id;

-- Then drop the column
ALTER TABLE sales_orders DROP COLUMN cost_center_id;
