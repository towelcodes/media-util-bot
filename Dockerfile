FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./database ./database
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin media-bot

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/media-bot /usr/local/bin
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    libcurl4 \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/local/bin/media-bot"]
