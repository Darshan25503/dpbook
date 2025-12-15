# Build stage
FROM rust:1.92-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false dpbook

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/dpbook /usr/local/bin/dpbook

# Create data directory for contacts
RUN mkdir -p /app/data && chown dpbook:dpbook /app/data

# Switch to app user
USER dpbook

# Set default data directory
ENV DPBOOK_DATA_DIR=/app/data

# Expose volume for data persistence
VOLUME ["/app/data"]

# Set the default command
ENTRYPOINT ["dpbook"]
CMD ["--help"]
