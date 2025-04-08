-- Update discounts table to use snake_case for enums and add support for item-specific discounts

-- Create a new discounts table with updated constraints
CREATE TABLE discounts_new (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    discount_type TEXT NOT NULL CHECK(discount_type IN ('percentage', 'fixed_amount')),
    value BIGINT NOT NULL,
    scope TEXT NOT NULL CHECK(scope IN ('all_items', 'specific_items')),
    state TEXT NOT NULL DEFAULT 'active' CHECK(state IN ('active', 'inactive', 'scheduled', 'expired')),
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Copy data from old table to new table, converting enum values to snake_case
INSERT INTO discounts_new (
    id, name, description,
    discount_type, value, scope, state,
    start_date, end_date, created_at, updated_at
)
SELECT
    id, name, description,
    CASE discount_type
        WHEN 'Percentage' THEN 'percentage'
        WHEN 'FixedAmount' THEN 'fixed_amount'
    END,
    value,
    CASE scope
        WHEN 'AllItems' THEN 'all_items'
    END,
    CASE state
        WHEN 'Active' THEN 'active'
        WHEN 'Inactive' THEN 'inactive'
        WHEN 'Scheduled' THEN 'scheduled'
        WHEN 'Expired' THEN 'expired'
    END,
    start_date, end_date, created_at, updated_at
FROM discounts;

-- Drop the old table
DROP TABLE discounts;

-- Rename the new table to the original name
ALTER TABLE discounts_new RENAME TO discounts;

-- Recreate the trigger for updated_at
CREATE TRIGGER update_discounts_updated_at
AFTER UPDATE ON discounts
FOR EACH ROW
BEGIN
    UPDATE discounts SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
