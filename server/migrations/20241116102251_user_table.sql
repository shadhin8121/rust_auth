-- Add migration script here
-- Ensure pgcrypto extension is available for gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE
    users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL
    );