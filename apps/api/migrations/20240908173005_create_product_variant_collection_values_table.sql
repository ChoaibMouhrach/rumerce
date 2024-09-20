-- Add migration script here
CREATE TABLE IF NOT EXISTS product_variant_collection_values (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name TEXT NOT NULL,
    key_id UUID NOT NULL,
    FOREIGN KEY (key_id) REFERENCES product_variant_collection_keys (id) ON DELETE CASCADE
)