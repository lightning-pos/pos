-- Drop the trigger first
DROP TRIGGER IF EXISTS update_tax_groups_updated_at;

-- Drop the indexes
DROP INDEX IF EXISTS idx_tax_group_taxes_tax_group_id;
DROP INDEX IF EXISTS idx_tax_group_taxes_tax_id;

-- Drop the tables
DROP TABLE IF EXISTS tax_group_taxes;
DROP TABLE IF EXISTS tax_groups;
