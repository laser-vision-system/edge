# BUILDER 
FROM --platform=$BUILDPLATFORM rust:bookworm AS builder
WORKDIR /ap

ARG BUILD_TARGET

RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly
RUN rustup override set nightly

COPY . .
RUN . ./docker_binary.sh $BUILD_TARGET

RUN cargo build --profile $BUILD_TARGET --bin edge --target-dir ./target/docker

# CONTAINER
FROM --platform=$BUILDPLATFORM debian:trixie AS runtime

ARG BUILD_TARGET

COPY docker_binary.sh /docker_binary.sh 
RUN . ./docker_binary.sh $BUILD_TARGET

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/target/docker/$BINARY_DIR/edge /edge
CMD ["/edge"]
