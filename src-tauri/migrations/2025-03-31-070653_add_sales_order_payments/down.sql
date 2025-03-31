-- This file should undo anything in `up.sql`

-- Drop indexes first
DROP INDEX IF EXISTS sales_order_payments_order_id_idx;
DROP INDEX IF EXISTS sales_order_payments_payment_method_id_idx;
DROP INDEX IF EXISTS sales_order_payments_state_idx;

-- Drop the table
DROP TABLE IF EXISTS sales_order_payments;
