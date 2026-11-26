# ---- Build Stage ----
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

# Copy manifests and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the release binary
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Copy only the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/Rust-lesson-3 /usr/local/bin/Rust-lesson-3

# Run the binary
CMD ["Rust-lesson-3"]