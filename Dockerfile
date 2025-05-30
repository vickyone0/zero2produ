# # Builder stage
# FROM rust:latest AS builder
# WORKDIR /app
# RUN apt update && apt install lld clang -y
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build --release
# # Runtime stage
# # FROM rust:1.59.0 AS runtime
# # WORKDIR /app
# # # Copy the compiled binary from the builder environment
# # # to our runtime environment
# # COPY --from=builder /app/target/release/zero2prod zero2prod
# # # We need the configuration file at runtime!
# # COPY configuration configuration
# # ENV APP_ENVIRONMENT production
# # ENTRYPOINT ["./zero2prod"]
# FROM debian:bullseye-slim AS runtime
# WORKDIR /app
# # Install OpenSSL - it is dynamically linked by some of our dependencies
# # Install ca-certificates - it is needed to verify TLS certificates
# # when establishing HTTPS connections
# RUN apt-get update -y \
# && apt-get install -y --no-install-recommends openssl ca-certificates \
# # Clean up
# && apt-get autoremove -y \
# && apt-get clean -y \
# && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /app/target/release/zero2prod zero2prod
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
# ENTRYPOINT ["./zero2prod"]

FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app
RUN apt update && apt install lld clang -y
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json


# Install OpenSSL build dependencies
RUN apt update && \
    apt install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

    
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin zero2prod
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]