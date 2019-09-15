FROM debian:stretch-slim
FROM liuchong/rustup:stable

LABEL Name=sos-page Version=1.0.0

RUN apt update
RUN apt install -y curl
ENV PATH $PATH:~/.cargo/bin

WORKDIR /app
COPY . .

RUN rustup default nightly-2019-09-13
RUN cargo build --release

CMD ROCKET_PORT=$PORT ./target/release/sos-page
