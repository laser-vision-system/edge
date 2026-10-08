FROM --platform=$BUILDPLATFORM rust:bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly
RUN rustup override set nightly

COPY . .

RUN cargo build --profile $BUILD_TARGET --bin edge

FROM --platform=$BUILDPLATFORM debian:trixie AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/edge /edge
CMD ["/edge"]