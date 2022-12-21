use std::sync::atomic::AtomicU64;

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};

pub struct PromMetrics {
    pub upload_gauge: Gauge<u64, AtomicU64>,
    pub download_gauge: Gauge<u64, AtomicU64>,
    pub ping_gauge: Gauge<f64, AtomicU64>,
}

impl PromMetrics {
    pub fn new() -> Self {
        Self {
            upload_gauge: Gauge::<u64, AtomicU64>::default(),
            download_gauge: Gauge::<u64, AtomicU64>::default(),
            ping_gauge: Gauge::<f64, AtomicU64>::default(),
        }
    }
    pub fn setup_prometheus(&self) -> Registry {
        let mut registry = <Registry>::default();

        registry.register(
            "upload_speed_bytes",
            "Upload speed of the internet connection",
            Box::new(self.upload_gauge.clone()),
        );
        registry.register(
            "download_speed_bytes",
            "Download speed of the internet connection",
            Box::new(self.download_gauge.clone()),
        );
        registry.register(
            "ping_latency_milliseconds",
            "Latency of the connection",
            Box::new(self.ping_gauge.clone()),
        );
        registry
    }
}
