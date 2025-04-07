-- Create tax_groups table
CREATE TABLE tax_groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create tax_group_taxes table (many-to-many relationship between tax_groups and taxes)
CREATE TABLE tax_group_taxes (
    tax_group_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (tax_group_id, tax_id),
    FOREIGN KEY (tax_group_id) REFERENCES tax_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX idx_tax_group_taxes_tax_group_id ON tax_group_taxes (tax_group_id);
CREATE INDEX idx_tax_group_taxes_tax_id ON tax_group_taxes (tax_id);

-- Create trigger to update the updated_at timestamp
CREATE TRIGGER update_tax_groups_updated_at
AFTER UPDATE ON tax_groups
FOR EACH ROW
BEGIN
    UPDATE tax_groups SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
