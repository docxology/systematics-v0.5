# syntax = docker/dockerfile:1

# Build stage for frontend
FROM rust:1.83-bookworm AS frontend-builder

# Install Trunk and wasm32 target
RUN cargo install --locked trunk@0.20.3 && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend ./frontend
COPY middleware ./middleware

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Build stage for backend
FROM rust:1.83-bookworm AS backend-builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY middleware ./middleware

# Build backend in release mode
WORKDIR /app/backend
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/systematics-backend /app/systematics-backend

# Copy frontend dist
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Expose port
EXPOSE 8000

# Run the backend
CMD ["/app/systematics-backend"]
