-- Add the category_id column to expenses (SQLite doesn't support ALTER COLUMN SET NOT NULL directly)
ALTER TABLE expenses ADD COLUMN category_id TEXT REFERENCES purchase_categories(id);

-- Update existing expenses to reference the appropriate category_id
UPDATE expenses
SET category_id = (
    SELECT id FROM purchase_categories
    WHERE name = expenses.category
    LIMIT 1
);

-- If a matching category wasn't found, set it to the default/first category
UPDATE expenses
SET category_id = (SELECT id FROM purchase_categories LIMIT 1)
WHERE category_id IS NULL;

-- Create a new table with the desired structure
CREATE TABLE new_expenses (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expense_date TIMESTAMP NOT NULL,
    category_id TEXT NOT NULL REFERENCES purchase_categories(id),
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Copy data from old to new table
INSERT INTO new_expenses
SELECT id, title, amount, expense_date, category_id, description, created_at, updated_at
FROM expenses;

-- Drop the old table and rename the new one
DROP TABLE expenses;
ALTER TABLE new_expenses RENAME TO expenses;
