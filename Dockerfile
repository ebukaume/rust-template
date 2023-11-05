#Ref - https://kerkour.com/rust-small-docker-image

FROM rust:1.73.0 AS builder

RUN update-ca-certificates

WORKDIR /app

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target \
    cargo build --release && \
    cargo strip && \
    mv /app/target/release/server /app/server


FROM debian:bookworm-slim

COPY --from=builder /app/server .

ENTRYPOINT ["./server"]

EXPOSE 4242
