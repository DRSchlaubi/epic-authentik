FROM rust as build
WORKDIR /usr/src/epic-authentik
COPY . .

RUN cargo build --release
FROM alpine:latest
COPY --from=build /usr/src/epic-authentik/target/release/epic-authentik .
CMD ["epic-authentik"]
