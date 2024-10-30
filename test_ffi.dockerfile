FROM rust:1.82.0-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    golang \
    git

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build Rust library
RUN cd ffi && cargo build --release

# Build Go binary with rpath
RUN cd ffi/go-example && \
    mkdir -p lib && \
    cp ../../target/release/libbitcoin_vault_ffi.* ./lib/ && \
    go build -ldflags="-r /app/lib" -o bin/main main.go

# Create runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built artifacts from builder
COPY --from=builder /app/ffi/go-example/bin/main /app/main
COPY --from=builder /app/ffi/go-example/lib/libbitcoin_vault_ffi.* /app/lib/

# Set library path
ENV LD_LIBRARY_PATH=/app/lib

# Run the binary
CMD ["/app/main"] 