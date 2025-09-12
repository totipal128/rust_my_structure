# rust_my_structure
structure application language rust....

```
RUST_MY_STRUCTURE # project
├── migrations/ # Diesel migrations
├── src/ #
│ ├── app/
│ │ ├── app1/ # Modul aplikasi 1
│ │ │ ├── controller/ # Controller (handler API)
│ │ │ ├── models/ # Model (ORM)
│ │ │ └── mod.rs
│ │ └── mod.rs
│ ├── base/
│ │ ├── databases/ # Koneksi & setup database
│ │ ├── responses/ # Response wrapper
│ │ ├── route/ # Routing sistem
│ │ └── mod.rs
│ ├── helpers/ # Helper function
│ ├── main.rs # Entry point aplikasi
│ └── schema.rs # Diesel schema (otomatis generate)
├── .env # Konfigurasi environment
├── Cargo.toml # Dependencies
├── diesel.toml # Diesel config
├── Dockerfile # Docker image
├── makefile # Command automation
└── README.md
```

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
sebelum menjalankan perintah tersebut sebaiknya buat file `.env` dan terdapat config env `DATABASE_URL=postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}`
ganti `${}` sesuai config databases postgres
```hash
diesel setup
```
Ini akan membuat folder migrations/ dan menyiapkan database di Docker PostgreSQL sesuai DATABASE_URL.

### Table
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
- menamabahkan kolom pada table
```
diesel migration generate add_type_to_posts
```
`up.sql`
```
ALTER TABLE posts ADD COLUMN type INT;
```
`down.sql`
```
ALTER TABLE posts DROP COLUMN type;
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
depedency yang di gunakan yaitu actix-web `https://actix.rs/docs/`
```
cargo add actix-web
```