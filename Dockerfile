FROM ubuntu:24.04 AS base
WORKDIR /

RUN set -ex; \
    apt-get update; \
    apt-get install -y git meson pkg-config cmake g++ gcc curl libssl-dev openssl;

# Set up the Rust toolchain
RUN apt-get install -y rustup; \
    rustup default stable; \
    rustup component add clippy rustfmt;

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .
