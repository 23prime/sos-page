FROM debian:stretch-slim
FROM liuchong/rustup:nightly

LABEL Name=sos-page Version=1.0.0

ENV PATH $PATH:~/.cargo/bin
# ENV CARGO_BUILD_TARGET_DIR=/tmp/target
# RUN rustup default nightly-2019-09-13

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/
RUN touch src/lib.rs
RUN cargo build --release
COPY . .
RUN rm -rf src/lib.rs
RUN cargo build --release

CMD ROCKET_PORT=$PORT ./target/release/sos-page
