# syntax=docker/dockerfile:1.7

FROM rust:1.98-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked && \
    cp /app/target/release/realtor_bot /realtor_bot

FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.source="https://github.com/rusyaz/realtor_bot"

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /realtor_bot /usr/local/bin/realtor_bot

ENV RUST_LOG=info
CMD ["realtor_bot"]
