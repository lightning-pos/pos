-- This file creates the sales_order_payments table for tracking payments for sales orders

CREATE TABLE sales_order_payments (
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    payment_method_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    reference_number TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (order_id) REFERENCES sales_orders(id),
    FOREIGN KEY (payment_method_id) REFERENCES payment_methods(id)
);
