# Multi-stage build for tfdiff
FROM rust:1.75-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/tfdiff

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests
COPY benches ./benches

# Build the application
RUN cargo build --release --bin tfdiff

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r tfdiff && useradd -r -g tfdiff tfdiff

# Copy the binary from builder stage
COPY --from=builder /usr/src/tfdiff/target/release/tfdiff /usr/local/bin/tfdiff

# Set permissions
RUN chmod +x /usr/local/bin/tfdiff

# Switch to non-root user
USER tfdiff

# Set entrypoint
ENTRYPOINT ["tfdiff"]

# Default command
CMD ["--help"]

# Metadata
LABEL \
    org.opencontainers.image.title="tfdiff" \
    org.opencontainers.image.description="Beautiful Terraform plan and apply output formatter" \
    org.opencontainers.image.vendor="tfdiff" \
    org.opencontainers.image.source="https://github.com/yourusername/tfdiff" \
    org.opencontainers.image.documentation="https://github.com/yourusername/tfdiff/README.md"