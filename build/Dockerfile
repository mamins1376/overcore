FROM liuchong/rustup:nightly

RUN mkdir -p /build/src

WORKDIR /build

RUN touch src/lib.rs

ADD Cargo.toml .

RUN cargo build

RUN rm -rf src/

RUN find target/ -name '*overlib*' -type f -delete
