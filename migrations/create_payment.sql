CREATE TABLE payment (
    id SERIAL PRIMARY KEY,
    order_uid UUID REFERENCES orders(order_uid),
    transaction VARCHAR(255),
    request_id VARCHAR(255),
    currency VARCHAR(10),
    provider VARCHAR(100),
    amount INT,
    payment_dt BIGINT,
    bank VARCHAR(100),
    delivery_cost INT,
    goods_total INT,
    custom_fee INT
);

CREATE INDEX idx_items_order_uid ON items (order_uid);