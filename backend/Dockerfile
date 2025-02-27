# Use the official Rust image as a builder
FROM rust:1.85.0 as builder
WORKDIR /app

# Set CARGO_HOME so that installed binaries go to /usr/local/cargo
ENV CARGO_HOME=/usr/local/cargo

# Install SQLx CLI for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy the full source code
COPY . .

# Copy the migrations folder
COPY migrations ./migrations

# Build for release (ensure the binary is named `backend` in Cargo.toml's [package].name)
RUN cargo build --release

# Use a minimal image for running the app
FROM ubuntu:22.04
WORKDIR /app

# Install dependencies needed for migration (libpq-dev + postgresql-client)
RUN apt-get update && apt-get install -y libpq-dev postgresql-client && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend .
# Copy the migrations folder from the builder stage
COPY --from=builder /app/migrations ./migrations
# Copy the sqlx binary from the builder stage (now installed in /usr/local/cargo/bin)
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Wait for PostgreSQL to be ready, run migrations, then start the backend
CMD ["sh", "-c", "until pg_isready -h postgres -p 5432; do sleep 2; done && sqlx migrate run && ./backend"]