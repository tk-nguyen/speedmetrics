FROM rust:bullseye AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM bitnami/minideb:bullseye

# Using s6-overlay
ARG S6_OVERLAY_VERSION=3.1.2.1
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
RUN install_packages curl gpg xz-utils ca-certificates && \
    tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && \
    tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz && \
    curl -sSL https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | bash && \
    install_packages speedtest

ENTRYPOINT ["/init"]

COPY --from=build /app/target/release/speedmetrics /app/
EXPOSE 9090
CMD ["/app/speedmetrics"]
