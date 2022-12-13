FROM rust:bullseye AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM bitnami/minideb:bullseye

RUN install_packages dumb-init curl gpg ca-certificates && \
    curl -sSL https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | bash && \
    install_packages speedtest

ENTRYPOINT ["dumb-init"]

COPY --from=build /app/target/release/speedmetrics /app/
EXPOSE 9090
CMD ["/app/speedmetrics"]
