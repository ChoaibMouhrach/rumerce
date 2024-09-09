-- Add migration script here
CREATE TABLE IF NOT EXISTS carts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    user_id UUID,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
)