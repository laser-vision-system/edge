# ---- Builder Stage ----
# Use the official Rust image. The 'slim' variant is smaller and works for most cases.
FROM rust:slim-bookworm AS builder

# Set the working directory.
WORKDIR /usr/src/app

ARG LLVM_VERSION=12

# Make sure that all packages are up to date then
# install the base Ubuntu packages that we need for
# building the kernel

RUN apt-get update -qq && \
    DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
        bc \
        binutils \
        binutils-aarch64-linux-gnu \
        binutils-arm-linux-gnueabi \
        binutils-mips-linux-gnu \
        binutils-mipsel-linux-gnu \
        binutils-powerpc-linux-gnu \
        binutils-powerpc64-linux-gnu \
        binutils-powerpc64le-linux-gnu \
        binutils-riscv64-linux-gnu \
        binutils-s390x-linux-gnu \
        bison \
        ca-certificates \
        ccache \
        cpio \
        curl \
        expect \
        flex \
        git \
        gnupg \
        libelf-dev \
        libssl-dev \
        lz4 \
        make \
        opensbi \
        openssl \
        ovmf \
        qemu-efi-aarch64 \
        qemu-system-arm \
        qemu-system-mips \
        qemu-system-misc \
        qemu-system-ppc \
        qemu-system-x86 \
        u-boot-tools \
        xz-utils \
        zstd && \
    rm -rf /var/lib/apt/lists/*

# Install the latest nightly Clang/lld packages from apt.llvm.org
# Delete all the apt list files since they're big and get stale quickly
RUN curl https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    echo "deb http://apt.llvm.org/focal/ llvm-toolchain-focal$(test ${LLVM_VERSION} -ne 12 && echo "-${LLVM_VERSION}") main" | tee -a /etc/apt/sources.list && \
    apt-get update -qq && \
    apt-get install --no-install-recommends -y \
        clang-${LLVM_VERSION} \
        lld-${LLVM_VERSION} \
        llvm-${LLVM_VERSION} && \
    chmod -f +x /usr/lib/llvm-${LLVM_VERSION}/bin/* && \
    rm -rf /var/lib/apt/lists/*


# Install musl tools for static compilation.
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

# Copy your application's source code.
COPY . .

# Build a statically-linked, release-optimized binary.
RUN cargo build --target x86_64-unknown-linux-musl --release

# ---- Final Stage ----
# Use the 'scratch' image, which is completely empty.
FROM scratch

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/edge /edge

# Set the entrypoint for the container
CMD ["/edge"]