-- This file should undo anything in `up.sql`

-- Drop the trigger first
DROP TRIGGER IF EXISTS update_discounts_updated_at;

-- Drop the discounts table
DROP TABLE IF EXISTS discounts;

-- Drop the custom enum types
DROP TYPE discount_state;
DROP TYPE discount_scope;
DROP TYPE discount_type;
