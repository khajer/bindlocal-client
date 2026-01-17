
FROM rust:1.83-alpine AS builder
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src
COPY src ./src
RUN cargo build --release


FROM alpine:3.20
RUN apk add --no-cache ca-certificates
RUN addgroup -g 1000 connl && \
    adduser -D -u 1000 -G connl connl

WORKDIR /home/connl
COPY --from=builder /app/target/release/connl /usr/local/bin/connl

RUN chown -R connl:connl /home/connl
USER connl
ENTRYPOINT ["connl"]

CMD ["--help"]
