# Stage 1: Build the Rust project
FROM rust:latest AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the source code
COPY . .

# Update Cargo and build the project
RUN rustup update && cargo build --release

# Stage 2: Create the final image
FROM debian:buster-slim

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/fibbot .

# Ensure the binary is executable
RUN chmod +x fibbot

CMD ["./fibbot"]