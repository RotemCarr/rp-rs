FROM rust:latest

WORKDIR /src
COPY . .

RUN apt-get update && apt-get install -y gcc-arm-none-eabi
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add thumbv8m.main-none-eabihf

CMD ["cargo", "build"]
