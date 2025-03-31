-- This file should undo anything in `up.sql`

-- Drop the index first
DROP INDEX IF EXISTS sales_order_payments_state_idx;

-- Note: SQLite doesn't support dropping columns directly
-- A full table recreation would be required to remove a column
-- This is a placeholder for documentation purposes
-- In SQLite, we would need to:
-- 1. Create a new table without the column
-- 2. Copy data from the old table to the new one
-- 3. Drop the old table
-- 4. Rename the new table to the old name
-- For simplicity, we'll just leave this comment here since this isn't a
-- critical operation for a down migration
