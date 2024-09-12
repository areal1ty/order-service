CREATE TABLE orders (
    order_uid UUID PRIMARY KEY,
    track_number VARCHAR(255),
    entry VARCHAR(50),
    locale VARCHAR(10),
    customer_id VARCHAR(50),
    delivery_service VARCHAR(100),
    shardkey VARCHAR(10),
    sm_id INT,
    date_created TIMESTAMP,
    oof_shard VARCHAR(10)
);
CREATE INDEX idx_orders_order_uid ON orders (order_uid);
