FROM rust:bullseye AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc

# Using s6-overlay
ARG S6_OVERLAY_VERSION=3.1.2.1
ARG SPEEDTEST_CLI_VERSION=1.2.0
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
# Speedtest binary
ADD https://install.speedtest.net/app/cli/ookla-speedtest-${SPEEDTEST_CLI_VERSION}-linux-x86_64.tgz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && \
    tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz && \
    tar xzvf /tmp/ookla-speedtest-${SPEEDTEST_CLI_VERSION}-linux-x86_64.tgz && \
    cp /tmp/ookla-speedtest-${SPEEDTEST_CLI_VERSION}-linux-x86_64/speedtest /usr/local/bin

ENTRYPOINT ["/init"]

COPY --from=build /app/target/release/speedmetrics /app/
CMD ["/app/speedmetrics"]
