FROM alpine:3.21

RUN apk add --no-cache \
    bash \
    git \
    openssh \
    curl \
    wget \
    tar \
    python3 \
    py3-pip \
    nodejs \
    npm \
    rust \
    rustup \
    cargo \
    meson \
    pkgconfig \
    gnupg \
    openssl

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH="/usr/local/cargo/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path \
    && rustup default stable \
    && rustup component add rustfmt clippy
