# FROM rust:latest AS builder

# WORKDIR /app

# # Install build dependencies
# RUN apt-get update && \
#     apt-get install -y \
#     lld \
#     clang \
#     && rm -rf /var/lib/apt/lists/*

# # Copy dependency files first for better caching
# COPY Cargo.toml Cargo.lock ./

# RUN rustc --version && cargo --version

# # Create dummy source to build dependencies
# RUN mkdir src && \
#     echo "fn main() {}" > src/main.rs && \
#     echo "fn dummy() {}" > src/lib.rs && \
#     cargo build --release && \
#     rm -rf src

# # Copy real source code
# COPY src ./src

# # Touch real main.rs to prevent cached dummy build
# RUN touch src/main.rs src/lib.rs

# # Build application
# ENV SQLX_OFFLINE true
# RUN cargo build --release && \
#     rm -rf target/release/deps/zero2prod*

# # Runtime image
# FROM debian:bookworm-slim

# WORKDIR /app

# # Copy only the binary
# COPY --from=builder /app/target/release/zero2prod /app/

# ENTRYPOINT ["./zero2prod"]

FROM rust:latest
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]