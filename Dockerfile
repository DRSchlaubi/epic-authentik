FROM rust:alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev pkgconfig libressl-dev && cargo build --release

FROM alpine
RUN apk --no-cache add ca-certificates
COPY --from=builder /usr/src/app/target/release/epic-authentik /
ENTRYPOINT ["/epic-authentik"]
