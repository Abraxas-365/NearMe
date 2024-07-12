-- Stores table
CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- Categories table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    store_id INTEGER NOT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

-- Products table (removed single_price)
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    sku VARCHAR(255) UNIQUE NOT NULL,
    category_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    store_id INTEGER NOT NULL,
    visible BOOLEAN DEFAULT TRUE,
    has_multiple_prices BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (category_id) REFERENCES categories(id),
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

-- Product images table
CREATE TABLE product_images (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

-- Prices table (modified to handle single prices as well)
CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    discount DECIMAL(10, 2),
    is_default BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (product_id) REFERENCES products(id),
    CONSTRAINT unique_default_price UNIQUE (product_id, is_default)
);

-- Customers table
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE
);

-- Orders table
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL,
    customer_id INTEGER NOT NULL,
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(50) NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    order_type VARCHAR(50) NOT NULL,
    table_number INTEGER,
    delivery_address TEXT,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    CONSTRAINT check_order_type CHECK (order_type IN ('delivery', 'on_site', 'takeaway')),
    CONSTRAINT check_table_number CHECK (
        (order_type = 'on_site' AND table_number IS NOT NULL) OR
        (order_type != 'on_site' AND table_number IS NULL)
    ),
    CONSTRAINT check_delivery_address CHECK (
        (order_type = 'delivery' AND delivery_address IS NOT NULL) OR
        (order_type != 'delivery' AND delivery_address IS NULL)
    )
);

-- Order items table
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL,
    price_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    item_price DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (price_id) REFERENCES prices(id)
);

-- Create indexes for faster queries
CREATE INDEX idx_product_store ON products(store_id);
CREATE INDEX idx_product_category ON products(category_id);
CREATE INDEX idx_price_product ON prices(product_id);
CREATE INDEX idx_order_type ON orders(order_type);
CREATE INDEX idx_order_status ON orders(status);
CREATE INDEX idx_customer_phone ON customers(phone);
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_price_id ON order_items(price_id);


-- Store Order Types table
CREATE TABLE store_order_types (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL,
    order_type VARCHAR(50) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    CONSTRAINT unique_store_order_type UNIQUE (store_id, order_type),
    CONSTRAINT valid_order_type CHECK (order_type IN ('delivery', 'on_site', 'takeaway'))
);

-- Create index for faster queries
CREATE INDEX idx_store_order_types ON store_order_types(store_id, order_type);


-- Function to initialize store order types
CREATE OR REPLACE FUNCTION initialize_store_order_types()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO store_order_types (store_id, order_type, is_active)
    VALUES 
        (NEW.id, 'delivery', TRUE),
        (NEW.id, 'on_site', TRUE),
        (NEW.id, 'takeaway', TRUE);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically initialize order types when a new store is created
CREATE TRIGGER trigger_initialize_store_order_types
AFTER INSERT ON stores
FOR EACH ROW
EXECUTE FUNCTION initialize_store_order_types();
