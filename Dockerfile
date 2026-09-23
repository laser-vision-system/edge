#FROM rust:slim-bookworm as base
#RUN cargo install cargo-chef --version ^0.1
#
#FROM base AS planner
#WORKDIR /app
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM base as builder
#WORKDIR /app
#COPY --from=planner /app/recipe.json recipe.json
#RUN rustup default stable
#RUN rustup target add aarch64-unknown-linux-gnu
#RUN cargo chef cook --release --recipe-path recipe.json
#COPY . .
#RUN cargo build --target=aarch64-unknown-linux-gnu --target-dir /usr/src/edge/target/release/edge
#
### ---- Builder Stage ----
### Use the official Rust image. The 'slim' variant is smaller and works for most cases.
##FROM rust:slim-bookworm AS builder
##
### Set the working directory.
##WORKDIR /usr/src/edge
##
### Install musl tools for static compilation.
##RUN apt-get update && apt-get install -y clang
##RUN rustup default stable
##RUN rustup target add aarch64-unknown-linux-gnu
##
##
### Copy your application's source code.
##COPY . .
##
##RUN cargo build --target=aarch64-unknown-linux-gnu --target-dir /usr/src/edge/target/release/edge
### RUN cargo build --release
### RUN cargo install --path .
#
#
## ---- Final Stage ----
## Use the 'scratch' image, which is completely empty.
#FROM scratch
#
#COPY --from=builder /usr/src/edge/target/release/edge /edge
#
## CMD cargo run
#CMD ["/edge"]
## Set the entrypoint for the container

# syntax=docker/dockerfile:1
FROM rust:1.85-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Builds dependencies - cached unless recipe.json changes
RUN cargo chef cook --release --recipe-path recipe.json
# Now copy actual source and build app
COPY . .
RUN cargo build --release --bin edge

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/edge /usr/local/bin/
CMD ["edge"]