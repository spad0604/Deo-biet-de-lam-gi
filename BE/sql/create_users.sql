-- SQL: create users table matching `entity::users::User`
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    password TEXT NOT NULL,
    image_url TEXT DEFAULT '',
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth TIMESTAMPTZ NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone_number TEXT,
    class TEXT,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Note: this SQL uses gen_random_uuid() from the pgcrypto/pgcrypto-like extension or pgcrypto;
-- if not available, you can use uuid_generate_v4() from the "uuid-ossp" extension or supply UUIDs from the app.
