# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y clang
COPY . .
# Builds dependencies - cached unless recipe.json changes
RUN cargo chef cook --release --recipe-path recipe.json
# Now copy actual source and build app
RUN cargo build --release --bin edge

FROM --platform=$BUILDPLATFORM debian:bookworm-slim AS runtime

COPY --from=builder /app/target/release/edge /edge
CMD ["edge"]