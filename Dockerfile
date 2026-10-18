FROM --platform=$BUILDPLATFORM rust:bookworm AS builder
WORKDIR /ap

ARG BUILD_TARGET
ARG BINARY_DIR

RUN . ./docker_binary.sh $BINARY_DIR

RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly
RUN rustup override set nightly

COPY . .

RUN cargo build --profile $BUILD_TARGET --bin edge

FROM --platform=$BUILDPLATFORM debian:trixie AS runtime
ARG BUILD_TARGET
ARG BINARY_DIR
COPY docker_binary.sh /docker_binary.sh 
RUN . ./docker_binary.sh $BUILD_TARGET
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/docker/$BINARY_DIR/edge /edge
CMD ["/edge"]