FROM rust:1.72 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm -f target/release/deps/be-low-level*

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/be-low-level /usr/local/bin/be-low-level

EXPOSE 4000

CMD ["be-low-level"]
