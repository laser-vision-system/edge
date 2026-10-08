FROM --platform=$BUILDPLATFORM rust:bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y clang

COPY . .

RUN cargo build --release --bin edge

FROM --platform=$BUILDPLATFORM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/scripts/bench-cnn-backends.sh /cnn_script.sh
RUN cnn_script.sh
COPY --from=builder /app/target/release/edge /edge
CMD ["/edge"]