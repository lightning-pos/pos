-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table
PRAGMA foreign_keys=off;

-- Create a new table without the cost_center_id column
CREATE TABLE new_expenses (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expense_date TIMESTAMP NOT NULL,
    category_id TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (category_id) REFERENCES purchase_categories (id)
);

-- Copy data from old table to new table, excluding the cost_center_id column
INSERT INTO new_expenses SELECT id, title, amount, expense_date, category_id, description, created_at, updated_at FROM expenses;

-- Drop the old table
DROP TABLE expenses;

-- Rename the new table to the original name
ALTER TABLE new_expenses RENAME TO expenses;

-- Turn foreign keys back on
PRAGMA foreign_keys=on;
