FROM rust:latest

ARG TARGETARCH

WORKDIR /src
COPY . .

RUN apt-get update && apt-get install -y gcc-arm-none-eabi
RUN if [ "$TARGETARCH" = "amd64" ]; then \
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
    rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu; \
    else \
        echo "Unsupported architecture"; \
        echo ${TARGETARCH}; \
        exit 1; \
    fi;

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup component add clippy
RUN rustup target add thumbv8m.main-none-eabihf
