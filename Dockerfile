# Stage 1: build aplikasi
FROM rust:1.81 as builder

WORKDIR /app

# copy Cargo.toml & Cargo.lock lebih dulu (supaya cache layer)
COPY Cargo.toml Cargo.lock ./

# buat dummy src/main.rs untuk compile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# build dependencies
RUN cargo build --release && rm -rf src

# copy semua source code
COPY . .

# build aplikasi
RUN cargo build --release

# Stage 2: runtime (lebih ringan)
FROM debian:bullseye-slim

# install deps minimal (kalau perlu SSL, dsb.)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# copy binary dari stage 1
COPY --from=builder /app/target/release/myapp /app/myapp

# expose port (kalau web app)
EXPOSE 8080

CMD ["./myapp"]
