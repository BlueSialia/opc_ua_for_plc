# Stage 1: build the runtime binary from source
FROM rust:1-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

COPY . .

RUN cargo build --release -p runtime

# Stage 2: minimal runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/runtime /usr/local/bin/runtime
COPY e2e-tests/gateway/config.toml /etc/opcua/config.toml

EXPOSE 4840

ENV RUST_LOG=info

ENTRYPOINT ["runtime", "--config", "/etc/opcua/config.toml"]
