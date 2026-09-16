# ---- Builder Stage ----
# Use the official Rust image. The 'slim' variant is smaller and works for most cases.
FROM rust:slim-bookworm AS builder

# Set the working directory.
WORKDIR /usr/src/edge

# Install musl tools for static compilation.
RUN apt-get update && apt-get install -y clang && apt-get install -y cargo && apt-get -f -y install nasm
RUN rustup default stable
RUN rustup target add aarch64-unknown-linux-gnu

# Copy your application's source code.
COPY . .

RUN cargo install --path .


# ---- Final Stage ----
# Use the 'scratch' image, which is completely empty.
FROM ubuntu

RUN apt-get update && apt-get install -y cargo

COPY --from=builder /usr/src/edge/target/release/edge /edge

# CMD cargo run
CMD ["/edge"]
# Set the entrypoint for the container
