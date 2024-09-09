-- Add migration script here
CREATE TABLE IF NOT EXISTS cart_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    quantity INTEGER NOT NULL,
    variant_id UUID NOT NULL,
    cart_id UUID NOT NULL,
    FOREIGN KEY (variant_id) REFERENCES product_variants (id) ON DELETE CASCADE,
    FOREIGN KEY (cart_id) REFERENCES carts (id) ON DELETE CASCADE
)