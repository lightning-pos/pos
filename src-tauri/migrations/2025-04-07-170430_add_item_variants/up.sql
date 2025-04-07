-- Create variant_types table
CREATE TABLE variant_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create variant_values table
CREATE TABLE variant_values (
    id TEXT PRIMARY KEY NOT NULL,
    variant_type_id TEXT NOT NULL,
    value TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (variant_type_id) REFERENCES variant_types(id)
);

-- Create item_variants table
CREATE TABLE item_variants (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    sku TEXT,
    price_adjustment BIGINT,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items(id)
);

-- Create item_variant_values junction table
CREATE TABLE item_variant_values (
    item_variant_id TEXT NOT NULL,
    variant_value_id TEXT NOT NULL,
    PRIMARY KEY (item_variant_id, variant_value_id),
    FOREIGN KEY (item_variant_id) REFERENCES item_variants(id),
    FOREIGN KEY (variant_value_id) REFERENCES variant_values(id)
);
