-- Create a new table with the old structure (including category column)
CREATE TABLE new_expenses (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    amount BIGINT NOT NULL,
    expense_date TIMESTAMP NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Update the category name from the linked category
INSERT INTO new_expenses
SELECT
    e.id,
    e.title,
    e.amount,
    e.expense_date,
    COALESCE(pc.name, 'Miscellaneous') as category,
    e.description,
    e.created_at,
    e.updated_at
FROM expenses e
LEFT JOIN purchase_categories pc ON e.category_id = pc.id;

-- Drop the original table and rename the new one
DROP TABLE expenses;
ALTER TABLE new_expenses RENAME TO expenses;
