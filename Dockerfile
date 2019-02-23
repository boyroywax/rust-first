FROM yasuyuky/rust-ssl-static@sha256:3df2c8949e910452ee09a5bcb121fada9790251f4208c6fd97bb09d20542f188 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_ALLOW_CROSS=1

FROM scratch
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ADD target/x86_64-unknown-linux-musl/release/hello_world /
ADD .env /

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

CMD ["/hello_world"]