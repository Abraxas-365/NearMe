CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    store_id INTEGER NOT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    sku VARCHAR(255) UNIQUE NOT NULL,
    category_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    store_id INTEGER NOT NULL,
    visible BOOLEAN DEFAULT TRUE,
    has_multiple_prices BOOLEAN DEFAULT FALSE,
    single_price DECIMAL(10, 2),
    FOREIGN KEY (category_id) REFERENCES categories(id),
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE TABLE product_images (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    discount DECIMAL(10, 2),
    is_default BOOLEAN,
    FOREIGN KEY (product_id) REFERENCES products(id),
    CHECK (is_default IN (TRUE, FALSE) OR is_default IS NULL)
);

-- To ensure only one default price per product
CREATE UNIQUE INDEX idx_single_default_price 
ON prices(product_id) 
WHERE is_default IS TRUE;
