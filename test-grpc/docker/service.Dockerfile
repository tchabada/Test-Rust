# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

# FROM rust:1.43 as cargo-build

# RUN apt-get update

# RUN apt-get install musl-tools -y

# RUN rustup target add x86_64-unknown-linux-musl

# RUN rustup component add rustfmt

# WORKDIR /usr/src/test-grpc

# COPY Cargo.toml Cargo.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUST_BACKTRACE=1 RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/test-grpc*

# COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

# FROM alpine:latest

# RUN addgroup -g 1000 test-grpc

# RUN adduser -D -s /bin/sh -u 1000 -G test-grpc test-grpc

# WORKDIR /home/test-grpc/bin/

# COPY --from=cargo-build /usr/src/test-grpc/target/x86_64-unknown-linux-musl/release/test-grpc .

# RUN chown test-grpc:test-grpc test-grpc

# USER test-grpc

# CMD ["./test-grpc"]

FROM rust:1.43

RUN rustup component add rustfmt

WORKDIR /usr/src/test-grpc

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/test-grpc*

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/test-grpc"]
