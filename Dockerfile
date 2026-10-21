# BUILDER 
FROM --platform=$BUILDPLATFORM rust:bookworm AS builder
WORKDIR /app

ARG BUILD_TARGET

RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly
RUN rustup override set nightly

COPY . .

RUN cargo build --profile $BUILD_TARGET --bin edge

# CONTAINER
FROM --platform=$BUILDPLATFORM debian:trixie AS runtime

ARG BUILD_TARGET
ARG BINARY_DIR

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/target/$BINARY_DIR/edge /edge
CMD ["/edge"]
