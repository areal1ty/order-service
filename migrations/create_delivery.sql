CREATE TABLE delivery (
    id SERIAL PRIMARY KEY,
    order_uid UUID REFERENCES orders(order_uid),
    name VARCHAR(255),
    phone VARCHAR(20),
    zip VARCHAR(20),
    city VARCHAR(100),
    address VARCHAR(255),
    region VARCHAR(100),
    email VARCHAR(255)
);

CREATE INDEX idx_delivery_order_uid ON delivery (order_uid);