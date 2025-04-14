-- SQLITE

CREATE TABLE product (
    id INTEGER NOT NULL PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    current_sell_price INTEGER NOT NULL,
    current_buy_price INTEGER NOT NULL,
);

CREATE TABLE sell_price_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    price INTEGER NOT NULL,
    saved_at TEXT NOT NULL,
)

CREATE TABLE buy_price_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    price INTEGER NOT NULL,
    saved_at TEXT NOT NULL,
)
