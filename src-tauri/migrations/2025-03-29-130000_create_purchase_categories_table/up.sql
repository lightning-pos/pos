-- SQLite doesn't support ENUM types, so we'll use a CHECK constraint
-- Create purchase_categories table
CREATE TABLE IF NOT EXISTS purchase_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create index for name column
CREATE INDEX idx_purchase_categories_name ON purchase_categories(name);

-- Create trigger for automatic updated_at
CREATE TRIGGER trigger_purchase_categories_updated_at
AFTER UPDATE ON purchase_categories
BEGIN
    UPDATE purchase_categories SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
