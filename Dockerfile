# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/rust-actix-web-stub

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rust-actix-web-stub*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 rust-actix-web-stub

RUN adduser -D -s /bin/sh -u 1000 -G rust-actix-web-stub rust-actix-web-stub

WORKDIR /home/rust-actix-web-stub/bin/

COPY --from=cargo-build /usr/src/rust-actix-web-stub/target/x86_64-unknown-linux-musl/release/rust-actix-web-stub .
COPY --from=cargo-build /usr/src/rust-actix-web-stub/config/default.toml ./config.toml

RUN chown rust-actix-web-stub:rust-actix-web-stub rust-actix-web-stub

USER rust-actix-web-stub

CMD ["/rust-actix-web-stub", "--input", "a", "--output", "b", "--config", "/config.toml"]
