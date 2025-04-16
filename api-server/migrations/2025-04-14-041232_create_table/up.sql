-- SQLITE

CREATE TABLE product (
    id INTEGER NOT NULL PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    current_sell_price DECIMAL(10,2) NOT NULL,
    current_buy_price DECIMAL(10,2) NOT NULL,
    current_inventary DECIMAL(10,2) NOT NULL,
    variation_enabled BOOLEAN NOT NULL,
);

CREATE TABLE sell_price_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    price DECIMAL(10,2) NOT NULL,
    saved_at DATETIME NOT NULL,
);

CREATE TABLE buy_price_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    price DECIMAL(10,2) NOT NULL,
    saved_at DATETIME NOT NULL,
);

CREATE TABLE variation_type (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    name TEXT NOT NULL,
);

CREATE TABLE variation (
    id INTEGER NOT NULL PRIMARY KEY,
    variation_type_id INTEGER NOT NULL FOREIGN KEY REFERENCES variation_type(id),
    value TEXT NOT NULL,
);

CREATE TABLE BUY (
    id INTEGER NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL FOREIGN KEY REFERENCES product(id),
    variation_id INTEGER FOREIGN KEY REFERENCES variation(id),
);

CREATE TABLE SELL (

);