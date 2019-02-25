# rust-first
rustup

## Installing racer on MacOSX
https://github.com/racer-rust/racer

## Serenity Discord Library
https://github.com/serenity-rs/serenity

## dotenv
* https://crates.io/crates/dotenv
* https://github.com/barosl/rust-dotenv

## Rust Docker Image
https://docs.docker.com/samples/library/rust/

### Rust Musl - compile Rust binaries with no dependencies
Static Compiling: Ideal for docker containers - https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/
```bash
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```
* https://github.com/emk/rust-musl-builder

Got containers working from this example:
```text
FROM yasuyuky/rust-ssl-static@sha256:3df2c8949e910452ee09a5bcb121fada9790251f4208c6fd97bb09d20542f188 as build

COPY ./ ./

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/totto /build-out/

RUN ls /build-out/

FROM scratch

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build /build-out/totto /

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

CMD ["/totto"]
```
* https://www.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes

## Docker Build --no-cache
```bash
docker build --no-cache --rm -f "Dockerfile" -t rust-first:latest .
```