FROM cffnpwr/rust-mold

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y pkg-config libssl-dev git && \
    mold -run rustup default nightly && \
    mold -run rustup update && \
    mold -run rustup component add clippy rustfmt && \
    mold -run cargo install cargo-watch cargo-edit

WORKDIR /app

COPY . .
