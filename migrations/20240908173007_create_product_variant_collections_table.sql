-- Add migration script here
CREATE TABLE IF NOT EXISTS product_variant_collections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    variant_id UUID NOT NULL,
    key_id UUID NOT NULL,
    value_id UUID NOT NULL,
    FOREIGN KEY (variant_id) REFERENCES product_variants (id) ON DELETE CASCADE,
    FOREIGN KEY (key_id) REFERENCES product_variant_collection_keys (id) ON DELETE CASCADE,
    FOREIGN KEY (value_id) REFERENCES product_variant_collection_values (id) ON DELETE CASCADE
)