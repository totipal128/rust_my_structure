# rust_my_structure
structure application language rust....

## Databases
pada connection ke databases postgrest dengan menggunakan depedency diesel `https://diesel.rs/guides/getting-started`

### instalasi
- Install Diesel CLI (di host atau container Rust)
Di host macOS/Linux:
```hash
cargo install diesel_cli --no-default-features --features postgres
```
- menambahakan depedensi ke rust
```
cargo add diesel --features postgres,r2d2
cargo add dotenvy
```
- Setup Diesel
```hash
diesel setup
```
Ini akan membuat folder migrations/ dan menyiapkan database di Docker PostgreSQL sesuai DATABASE_URL.

- Create Table ke databases
```
diesel migration generate create_posts
```
buat struktur tabel databses pada table migration :
contoh pada folder `migrations/2025-09-11-070629_create_posts/up.sql`:
```
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)

```
- Revert migration lama:
```
diesel migration revert
```
- migration databases
```
diesel migration run
```
- create schema databases dengan nama schema.rs
```zsh
diesel print-schema > src/schema.rs
```
- Drop table
```
diesel migration redo
```

## ROUTE
depedency tang di gunakan yaitu actix-web `https://actix.rs/docs/`
```
cargo add actix-web
```