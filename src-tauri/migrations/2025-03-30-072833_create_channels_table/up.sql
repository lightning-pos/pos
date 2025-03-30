-- Your SQL goes here

CREATE TABLE channels (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Add some default channels
INSERT INTO channels (id, name, description, is_active, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000001', 'POS', 'Point of Sale', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('00000000-0000-0000-0000-000000000002', 'Zomato', 'Zomato Online Delivery', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('00000000-0000-0000-0000-000000000003', 'Swiggy', 'Swiggy Online Delivery', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
