CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    order_uid UUID REFERENCES orders(order_uid),
    chrt_id BIGINT,
    track_number VARCHAR(255),
    price INT,
    rid VARCHAR(255),
    name VARCHAR(255),
    sale INT,
    size VARCHAR(50),
    total_price INT,
    nm_id BIGINT,
    brand VARCHAR(255),
    status INT
);

CREATE INDEX idx_items_order_uid ON items (order_uid);
