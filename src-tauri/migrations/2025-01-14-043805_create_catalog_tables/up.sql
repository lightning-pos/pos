-- Create item_categories table
CREATE TABLE item_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create taxes table
CREATE TABLE taxes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rate INTEGER NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create items table
CREATE TABLE items (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    nature TEXT NOT NULL,
    state TEXT NOT NULL,
    price INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (category_id) REFERENCES item_categories(id) ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Create item_taxes table
CREATE TABLE item_taxes (
    item_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (item_id, tax_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes(id) ON DELETE CASCADE ON UPDATE CASCADE
);
