-- This file creates the payment_methods table for tracking payment methods

CREATE TABLE payment_methods (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    state TEXT NOT NULL DEFAULT 'Active', -- Uses the PaymentMethodState enum
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
