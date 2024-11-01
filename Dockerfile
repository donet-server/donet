FROM ubuntu:24.04 as build
WORKDIR /usr/local/app

RUN set -ex; \
    apt-get update; \
    apt-get install -y git meson pkg-config cmake g++ gcc curl libssl-dev openssl;

# Set up the Rust toolchain
RUN apt-get install -y rustup; \
    rustup default stable; \
    rustup component add clippy rustfmt;

ENV PATH="/root/.cargo/bin:${PATH}"

# Expects daemon.toml to be present in the source root dir
COPY . .

RUN meson setup build -Dprofile=release;
RUN meson compile -C build;

ENTRYPOINT ["./build/donetd"]
