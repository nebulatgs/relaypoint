FROM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install mtr -y
COPY --from=builder /app/target/release/relaypoint /usr/local/bin
ENTRYPOINT ["/usr/local/bin/relaypoint"]