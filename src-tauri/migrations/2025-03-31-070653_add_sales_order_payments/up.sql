-- Create sales_order_payments table
CREATE TABLE sales_order_payments (
    id TEXT NOT NULL PRIMARY KEY,
    order_id TEXT NOT NULL,
    payment_method_id TEXT NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    amount BIGINT NOT NULL,
    reference_number TEXT,
    notes TEXT,
    state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY(order_id) REFERENCES sales_orders(id),
    FOREIGN KEY(payment_method_id) REFERENCES payment_methods(id)
);

-- Create indexes for faster lookups
CREATE INDEX sales_order_payments_order_id_idx ON sales_order_payments(order_id);
CREATE INDEX sales_order_payments_payment_method_id_idx ON sales_order_payments(payment_method_id);
CREATE INDEX sales_order_payments_state_idx ON sales_order_payments(state);
