# ── Build stage ───────────────────────────────────────────────────────────────
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install system dependencies needed for reqwest (TLS)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the full workspace
COPY . .

# Build only the server binary in release mode
RUN cargo build --release -p boxcar-server

# ── Runtime stage ──────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime TLS dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/boxcar-server /usr/local/bin/boxcar

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/boxcar"]
