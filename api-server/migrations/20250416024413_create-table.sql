-- SQLITE --

CREATE TABLE product (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    variation_enabled BOOLEAN NOT NULL,
    variation_name TEXT
);

CREATE TABLE product_variation (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    barcode TEXT NOT NULL UNIQUE,
    current_sell_price DECIMAL(10,2) NOT NULL,
    current_inventary DECIMAL(10,2) NOT NULL,
    product_id INTEGER NOT NULL REFERENCES product(id)
);

CREATE TABLE provider (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE buy_history (
    id INTEGER NOT NULL PRIMARY KEY,
    provider_id INTEGER REFERENCES provider(id),
    date DATETIME NOT NULL
);

CREATE TABLE product_buy_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_variation_id INTEGER NOT NULL REFERENCES product_variation(id),
    buy_history_id INTEGER NOT NULL REFERENCES buy_history(id),
    unit_value DECIMAL(10,2) NOT NULL,
    quantity DECIMAL(10,2) NOT NULL
);

CREATE TABLE client (
    id INTEGER NOT NULL PRIMARY KEY,
    phone_number TEXT,
    email TEXT,
    name TEXT
);

CREATE TABLE sell_history (
    id INTEGER NOT NULL PRIMARY KEY,
    client_id INTEGER REFERENCES client(id),
    date DATETIME NOT NULL
);

CREATE TABLE product_sell_history (
    id INTEGER NOT NULL PRIMARY KEY,
    product_variation_id INTEGER NOT NULL REFERENCES product_variation_id(id),
    sell_history_id INTEGER NOT NULL REFERENCES sell_history(id),
    unit_value DECIMAL(10,2) NOT NULL,
    quantity DECIMAL(10,2) NOT NULL
);
