-- Add migration script here
CREATE TABLE IF NOT EXISTS magic_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    token UUID NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
)