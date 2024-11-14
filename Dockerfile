FROM rust:1.82.0-alpine3.20 AS builder

RUN apk add --no-cache musl-dev

COPY . .
RUN cargo build --release

FROM scratch

COPY --from=builder /target/release/dodona-containerfile-evaluator /bin/dodona-containerfile-evaluator
