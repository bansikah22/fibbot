# Stage 1: Build
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:latest
WORKDIR /app

# Install OpenSSL runtime dependencies
RUN apt-get update && apt-get install -y libssl3

COPY --from=builder /app/target/release/fibbot /app/fibbot
COPY --from=builder /app/Cargo.toml /app/Cargo.toml
COPY --from=builder /app/Cargo.lock /app/Cargo.lock

CMD ["/app/fibbot"]
