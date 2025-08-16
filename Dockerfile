# Multi-stage build for VectraEdge
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/cli.rs

# Build dependencies
RUN cargo build --release

# Remove dummy files and copy real source
RUN rm -rf src
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false vectra

# Set working directory
WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/vectra /usr/local/bin/
COPY --from=builder /app/target/release/vectra-cli /usr/local/bin/

# Create data directory
RUN mkdir -p /app/data && chown -R vectra:vectra /app

# Switch to non-root user
USER vectra

# Expose ports
EXPOSE 8080 6432

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD vectra-cli health || exit 1

# Default command
CMD ["vectra"]
