#Ref - https://kerkour.com/rust-small-docker-image

####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

ENV USER=apprunner
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /app

COPY ./ .

RUN cargo build --release
RUN strip -s /app/target/release/api-implementation


####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/api-implementation ./

# Use an unprivileged user.
USER apprunner:apprunner

CMD ["/app/api-implementation"]
