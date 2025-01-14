-- This file should undo anything in `up.sql`

-- Drop tables in reverse order of creation to respect foreign key constraints
DROP TABLE item_taxes;
DROP TABLE items;
DROP TABLE taxes;
DROP TABLE item_categories;
