FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin media-bot

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/media-bot /usr/local/bin
RUN apt-get update && apt install -y openssl && apt install ca-certificates
ENTRYPOINT ["/usr/local/bin/media-bot"]