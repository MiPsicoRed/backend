# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code and sqlx offline data
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations

# Enable SQLx offline mode to avoid needing a database during compilation
ENV SQLX_OFFLINE=true

# Build for release with static linking
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies (OpenSSL and CA certificates)
RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/mipsicored-backend /app/mipsicored-backend

# Copy migrations (if we needed to run them at runtime)
# COPY migrations ./migrations

# Set environment variables (MUST be overridden at runtime, at least some of them)
ENV RUST_LOG="mipsicored_backend=debug,tower_http=debug,sqlx=warn"
# ENV DATABASE_URL="postgres://user:password@localhost/clean_architecture"
# ENV JWT_SECRET="replace_this_with_a_random_secret"
# ENV RESEND_KEY="add_your_resend_key"
# ENV RESEND_FROM_EMAIL="add_your_from_email"
ENV BASE_FRONTEND_URL="https://mipsicored.com"

# Expose the port the app listens on 
EXPOSE 3001

# Run the binary
CMD ["/app/mipsicored-backend"]