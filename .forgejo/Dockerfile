FROM rust:1.94

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    git \
    curl \
    gpg \
    wget \
    tar \
    python3 \
    python3-pip \
    python3-venv \
    nodejs \
    npm \
    meson \
    pkg-config \
    libssl-dev \
    openssl

RUN rm -rf /var/lib/apt/lists/*

RUN rustup default stable \
    && rustup component add rustfmt clippy
