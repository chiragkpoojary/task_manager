# Stage 1: Build the application
FROM rust:1.80 AS builder

# Install build dependencies including OpenSSL, BoringSSL, and other tools
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    clang \
    curl \
    git

# Set the working directory
WORKDIR /usr/src/app

# Set environment variables for OpenSSL/BoringSSL
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr/lib/ssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl

# Copy the source code into the container
COPY . .

# Build the application with full backtrace for debugging
RUN cargo build --release

# Stage 2: Create the runtime image
FROM ubuntu:22.04

# Install required runtime dependencies, including OpenSSL
RUN apt-get update && apt-get install -y \
    libssl3 \
    libgcc-s1 \
    libstdc++6 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the built application from the builder stage
COPY --from=builder /usr/src/app/target/release/to-do .

# Expose the port and specify the command to run the binary
EXPOSE 8080
CMD ["./to-do"]
