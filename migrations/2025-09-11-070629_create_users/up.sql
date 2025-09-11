-- Buat tabel users
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,       -- ✅ primary key
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    is_agent BOOLEAN NOT NULL DEFAULT false
);

-- Buat index di kolom name
CREATE INDEX idx_users_name ON users (name);
