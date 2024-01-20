FROM rust:alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev pkgconfig && cargo build --release

FROM scratch
COPY --from=builder /usr/src/app/target/release/epic-authentik /
ENTRYPOINT ["/epic-authentik"]
