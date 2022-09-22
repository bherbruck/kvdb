FROM rust:alpine as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine as runtime

WORKDIR /app

COPY --from=builder /app/target/release/kvdb .

RUN mkdir /data

CMD ["./kvdb", "/data/kvdb.db"]
