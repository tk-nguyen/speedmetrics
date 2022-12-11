# Speedtest with Prometheus Metrics

A simple rust application to measure your internet speed with [Ookla Speedtest CLI](https://www.speedtest.net/apps/cli).

This program expose prometheus metrics at `http://localhost:9090/metrics` when running.

Currently only have 3 metrics for now:

- `upload_speed`
- `download_speed`
- `ping_latency`

## License

- MIT License
