-- Drop the trigger
DROP TRIGGER IF EXISTS trigger_purchase_categories_updated_at;

-- Drop the index
DROP INDEX IF EXISTS idx_purchase_categories_name;

-- Drop the purchase_categories table
DROP TABLE IF EXISTS purchase_categories;
