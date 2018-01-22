FROM liuchong/rustup:nightly

RUN mkdir /build

WORKDIR /build

COPY . /build

RUN cargo test
