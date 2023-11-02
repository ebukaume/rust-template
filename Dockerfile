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
    mv /app/target/release/api_implementation /app


FROM debian:bookworm-slim

COPY --from=builder /app/api_implementation /

ENTRYPOINT ["./api_implementation"]

EXPOSE 4242
