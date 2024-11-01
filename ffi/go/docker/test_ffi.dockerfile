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
RUN cargo build --release

# Build Go binary with rpath
RUN cd ffi/go && \
    mkdir -p lib/linux && \
    cp ../../target/release/libbitcoin_vault_ffi.* ./lib/linux

CMD cd ffi/go && \
    LD_LIBRARY_PATH=$(pwd)/lib/linux:$LD_LIBRARY_PATH \
    CGO_LDFLAGS="-L$(pwd)/lib/linux -lbitcoin_vault_ffi" \
    CGO_CFLAGS="-I$(pwd)/lib/linux" \
        go test ./tests/... -v -cover -count=1 && \
    echo "Tests completed"
