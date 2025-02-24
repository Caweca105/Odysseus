# Use the official Rust image as a builder
FROM rust:1.85.0 as builder
WORKDIR /app

# (Optional) Cache dependencies by copying Cargo.toml and Cargo.lock first:
# COPY Cargo.toml Cargo.lock ./
# RUN cargo fetch

# Copy the full source code
COPY . .

# Build for release (ensure the binary is named `backend` in Cargo.toml's [package].name)
RUN cargo build --release

# Use a minimal image for running the app
FROM ubuntu:22.04
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend .

# Expose port 8000 and run the binary
EXPOSE 8000
CMD ["./backend"]