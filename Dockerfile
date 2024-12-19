# Backend Build Stage
FROM rust:1.83-slim AS backend-builder

WORKDIR /app/backend

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev

# Copy backend source
COPY backend/Cargo.toml ./Cargo.toml
COPY backend/src ./src

# Build backend
RUN cargo build --release

# Frontend Build Stage
FROM node:16-alpine AS frontend-builder

WORKDIR /app/frontend

# Copy frontend files
COPY frontend/package.json ./
COPY frontend/package-lock.json ./

# Install dependencies
RUN npm install

# Copy frontend source
COPY frontend/src ./src
COPY frontend/public ./public
COPY frontend/tsconfig.json ./

# Build frontend
RUN npm run build

# Final Stage
FROM debian:bullseye-slim

WORKDIR /app

# Create directory for files to be viewed
RUN mkdir /files

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/file-content-viewer /app/backend

# Copy frontend build
COPY --from=frontend-builder /app/frontend/build /app/frontend

# Expose port
EXPOSE 3000

# Run the application
CMD ["/app/backend"]