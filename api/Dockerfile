FROM rust:1.85.1

COPY . /app
WORKDIR /app

RUN cargo build --release

ENTRYPOINT ["target/release/api"]
