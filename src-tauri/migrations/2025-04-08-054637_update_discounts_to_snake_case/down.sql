-- This file should undo anything in `up.sql`

-- Create a new discounts table with original constraints
CREATE TABLE discounts_new (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    discount_type TEXT NOT NULL CHECK(discount_type IN ('Percentage', 'FixedAmount')),
    value BIGINT NOT NULL,
    scope TEXT NOT NULL CHECK(scope IN ('AllItems')),
    state TEXT NOT NULL DEFAULT 'Active' CHECK(state IN ('Active', 'Inactive', 'Scheduled', 'Expired')),
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Copy data from current table to new table, converting enum values back to PascalCase
INSERT INTO discounts_new (
    id, name, description,
    discount_type, value, scope, state,
    start_date, end_date, created_at, updated_at
)
SELECT
    id, name, description,
    CASE discount_type
        WHEN 'percentage' THEN 'Percentage'
        WHEN 'fixed_amount' THEN 'FixedAmount'
    END,
    value,
    CASE scope
        WHEN 'all_items' THEN 'AllItems'
        WHEN 'specific_items' THEN 'AllItems' -- Convert specific_items back to AllItems
    END,
    CASE state
        WHEN 'active' THEN 'Active'
        WHEN 'inactive' THEN 'Inactive'
        WHEN 'scheduled' THEN 'Scheduled'
        WHEN 'expired' THEN 'Expired'
    END,
    start_date, end_date, created_at, updated_at
FROM discounts;

-- Drop the current table
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
