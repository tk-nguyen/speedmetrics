# Speedtest with Prometheus Metrics

[![Build image](https://github.com/tk-nguyen/speedmetrics/actions/workflows/image.yaml/badge.svg)](https://github.com/tk-nguyen/speedmetrics/actions/workflows/image.yaml) [![](https://ghcr-badge.deta.dev/tk-nguyen/speedmetrics/latest_tag?trim=major&label=latest)](https://github.com/tk-nguyen/speedmetrics/pkgs/container/speedmetrics)

A simple rust application to measure your internet speed with [Ookla Speedtest CLI](https://www.speedtest.net/apps/cli).

This program expose prometheus metrics at `http://localhost:9027/metrics` when running.

Currently only have 3 metrics for now:

- `upload_speed_bytes`
- `download_speed_bytes`
- `ping_latency_milliseconds`

## License

MIT License
