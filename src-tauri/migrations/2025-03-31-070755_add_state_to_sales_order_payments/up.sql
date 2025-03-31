-- Your SQL goes here

-- Add state column to sales_order_payments with a default value of 'Completed'
ALTER TABLE sales_order_payments ADD COLUMN state TEXT NOT NULL DEFAULT 'Completed';

-- Create an index on the state column for faster lookups
CREATE INDEX sales_order_payments_state_idx ON sales_order_payments(state);
