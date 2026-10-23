# BUILDER 
FROM --platform=$BUILDPLATFORM rust:bookworm AS builder

# home directory structure while building
WORKDIR /app

# Dev or release 
ARG BUILD_TARGET

# install dependancies including nightly for cargo
RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly
RUN rustup override set nightly

# copy git repo for building the project 
COPY . .

# build the binary using cargo
RUN cargo build --profile $BUILD_TARGET --bin edge

## CONTAINER
FROM --platform=$BUILDPLATFORM debian:trixie AS runtime

# Binary directory tells the docker file which directory the rust binary has been built to. passed during run
ARG BINARY_DIR

# install required certs for container 
RUN apt-get update && apt-get install -y ca-certificates

# coppies the compiled binary from our builder 
COPY --from=builder /app/target/$BINARY_DIR/edge /edge
CMD ["/edge"]
