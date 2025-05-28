FROM rust:1.86.0-slim-bullseye as base
RUN apt-get update && apt-get install -y --no-install-recommends \
    libudev-dev \
    pkg-config \
    build-essential \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef

FROM base as planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo chef prepare --recipe-path recipe.json

FROM base as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --bin cli --release  && \
    mkdir -p /output && \
    cp /app/target/release/cli /output/

FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    libudev1 \
    libssl1.1 \
    ca-certificates \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /output/cli /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/cli"]