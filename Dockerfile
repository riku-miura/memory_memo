# Multi-stage build for Memory Memo App
FROM rust:1.83 AS backend-builder

# Set working directory
WORKDIR /app/backend

# Copy backend files
COPY backend/Cargo.toml ./
COPY backend/src ./src
COPY backend/migrations ./migrations

# Build release version
RUN cargo build --release

# Frontend build stage
FROM node:18-alpine AS frontend-builder

# Set working directory
WORKDIR /app/frontend

# Copy frontend files
COPY frontend/ ./

# Install a simple HTTP server for serving static files
RUN npm install -g http-server

# Production stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/memory_memo ./memory_memo

# Copy frontend files
COPY --from=frontend-builder /app/frontend ./frontend

# Create data directory for SQLite
RUN mkdir -p /app/data

# Set environment variables
ENV DATABASE_URL=sqlite:///app/data/memory_memo.db
ENV PORT=3000
ENV RUST_LOG=info

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./memory_memo"]