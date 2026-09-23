# ---- Builder Stage ----
# Use the official Rust image. The 'slim' variant is smaller and works for most cases.
FROM rust:slim-bookworm AS builder

# Set the working directory.
WORKDIR /usr/src/edge

# Install musl tools for static compilation.
RUN apt-get update && apt-get install -y clang
RUN rustup default stable
RUN rustup target add aarch64-unknown-linux-gnu


# Copy your application's source code.
COPY . .

RUN cargo build --target=aarch64-unknown-linux-gnu --target-dir /usr/src/edge/target/release/edge
# RUN cargo build --release
# RUN cargo install --path .


# ---- Final Stage ----
# Use the 'scratch' image, which is completely empty.
FROM scratch

COPY --from=builder /usr/src/edge/target/release/edge /edge

# CMD cargo run
CMD ["/edge"]
# Set the entrypoint for the container
