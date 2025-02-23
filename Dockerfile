FROM rust:1.75

# Set working directory
WORKDIR /app

# Copy Rust project files
COPY . .

# Build the Rust project
RUN cargo build --release

# Set the command to run the compiled binary
CMD ["./target/release/fibbot"]
