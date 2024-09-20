-- Add migration script here
CREATE TABLE IF NOT EXISTS images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name TEXT NOT NULL,
    src TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)