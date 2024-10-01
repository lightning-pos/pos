/* Manual Migration */
CREATE TABLE IF NOT EXISTS item_taxes_backup (
    item_id TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    PRIMARY KEY (item_id, tax_id),
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE,
    FOREIGN KEY (tax_id) REFERENCES taxes (id) ON DELETE RESTRICT
);

INSERT INTO item_taxes_backup SELECT item_id, tax_id FROM item_taxes;

DROP TABLE item_taxes;

ALTER TABLE item_taxes_backup RENAME TO item_taxes;
