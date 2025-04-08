-- This file should undo anything in `up.sql`

-- Drop the indexes first
DROP INDEX IF EXISTS idx_item_discounts_item_id;
DROP INDEX IF EXISTS idx_item_discounts_discount_id;

-- Drop the item_discounts table
DROP TABLE IF EXISTS item_discounts;
