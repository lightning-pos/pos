-- Your SQL goes here

-- Create the discounts table for SQLite
CREATE TABLE discounts (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    discount_type TEXT NOT NULL CHECK(discount_type IN ('Percentage', 'FixedAmount')),
    value REAL NOT NULL, -- Using REAL for floating-point numbers in SQLite
    scope TEXT NOT NULL CHECK(scope IN ('AllItems')),
    state TEXT NOT NULL DEFAULT 'Active' CHECK(state IN ('Active', 'Inactive', 'Scheduled', 'Expired')),
    start_date DATETIME,
    end_date DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trigger to automatically update updated_at timestamp in SQLite
CREATE TRIGGER update_discounts_updated_at
AFTER UPDATE ON discounts
FOR EACH ROW
BEGIN
    UPDATE discounts SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
