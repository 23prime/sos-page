FROM debian:stretch-slim
FROM liuchong/rustup:stable

LABEL Name=sos-page Version=1.0.0

RUN apt update
RUN apt install -y curl
# RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.39.0 -y
ENV PATH $PATH:~/.cargo/bin

WORKDIR /app
COPY . .
# COPY Cargo.toml Cargo.lock Rocket.toml rust-toolchain ./

# RUN cargo check
RUN rustup default nightly-2019-09-13
RUN cargo build --release

# COPY . .

CMD ROCKET_PORT=$PORT cargo run --release
