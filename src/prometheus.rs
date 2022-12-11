use std::sync::atomic::AtomicU64;

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};

pub struct PromMetrics {
    pub upload_gauge: Gauge,
    pub download_gauge: Gauge,
    pub ping_gauge: Gauge<f64, AtomicU64>,
}

impl PromMetrics {
    pub fn new() -> Self {
        Self {
            upload_gauge: Gauge::default(),
            download_gauge: Gauge::default(),
            ping_gauge: Gauge::<f64, AtomicU64>::default(),
        }
    }
    pub fn setup_prometheus(&self) -> Registry {
        let mut registry = <Registry>::default();

        registry.register(
            "upload_speed",
            "Upload speed of the internet connection in bytes",
            Box::new(self.upload_gauge.clone()),
        );
        registry.register(
            "download_speed",
            "Download speed of the internet connection in bytes",
            Box::new(self.download_gauge.clone()),
        );
        registry.register(
            "ping_latency",
            "Latency of the connection in milliseconds",
            Box::new(self.ping_gauge.clone()),
        );
        registry
    }
}