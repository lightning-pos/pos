-- Create the item_discounts table for many-to-many relationship between items and discounts
CREATE TABLE item_discounts (
    item_id TEXT NOT NULL,
    discount_id TEXT NOT NULL,
    PRIMARY KEY (item_id, discount_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (discount_id) REFERENCES discounts(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_item_discounts_item_id ON item_discounts (item_id);
CREATE INDEX idx_item_discounts_discount_id ON item_discounts (discount_id);
