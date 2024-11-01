FROM rust:1.82.0-slim as builder

ENV ROOT_DIR /app/ffi/go-psbt

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
RUN cargo build --release

# Build Go binary with rpath
RUN cd $(ROOT_DIR) && \
    mkdir -p lib/linux && \
    cp ../../target/release/libbitcoin_vault_ffi.* ./lib/linux
