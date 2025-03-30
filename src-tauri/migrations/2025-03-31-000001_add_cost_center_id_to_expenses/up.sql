-- Add the new column with default empty string
ALTER TABLE expenses ADD COLUMN cost_center_id TEXT DEFAULT '';

-- Get the ID of the default cost center
UPDATE expenses
SET cost_center_id = (SELECT id FROM cost_centers LIMIT 1)
WHERE cost_center_id = '' OR cost_center_id IS NULL;

-- In SQLite, we need to create a new table with the constraint and migrate data
PRAGMA foreign_keys=off;

-- Create a new table with the foreign key constraint
CREATE TABLE new_expenses (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expense_date TIMESTAMP NOT NULL,
    category_id TEXT NOT NULL,
    cost_center_id TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (category_id) REFERENCES purchase_categories (id),
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers (id)
);

-- Copy data from old table to new table
INSERT INTO new_expenses SELECT * FROM expenses;

-- Drop the old table
DROP TABLE expenses;

-- Rename the new table to the original name
ALTER TABLE new_expenses RENAME TO expenses;

-- Create index for cost_center_id
CREATE INDEX idx_expenses_cost_center_id ON expenses(cost_center_id);

-- Turn foreign keys back on
PRAGMA foreign_keys=on;


