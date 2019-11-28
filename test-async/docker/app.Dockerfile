# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

RUN rustup component add rustfmt

WORKDIR /usr/src/test-async

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUST_BACKTRACE=1 RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/test-async*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 test-async

RUN adduser -D -s /bin/sh -u 1000 -G test-async test-async

WORKDIR /home/test-async/bin/

COPY --from=cargo-build /usr/src/test-async/target/x86_64-unknown-linux-musl/release/test-async .

RUN chown test-async:test-async test-async

USER test-async

CMD ["./test-async"]
