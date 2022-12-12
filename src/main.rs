use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use color_eyre::Result;
use prometheus_client::encoding::text::encode;
use tracing::info;
use tracing_subscriber;

#[allow(dead_code)]
mod models;
mod prometheus;
mod speedtest;
use prometheus::PromMetrics;
use speedtest::run_speedtest;

// 1 minute between speed tests
const SPEEDTEST_INTERVAL: u64 = 60;
const METRIC_ENDPOINT: &'static str = "/metrics";
const LISTEN_ADDRESS: &'static str = "0.0.0.0:9090";

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let prom_metrics = PromMetrics::new();
    let registry = prom_metrics.setup_prometheus();

    info!("Starting speedtest...");
    info!("Running for the first time to accept license...");
    Command::new("speedtest")
        .arg("--accept-license")
        .output()
        .expect("Failed to run speedtest. Please make sure speedtest-cli is in your PATH.");

    thread::spawn(move || loop {
        let result = run_speedtest().unwrap();
        prom_metrics.upload_gauge.set(result.upload.bytes as u64);
        prom_metrics
            .download_gauge
            .set(result.download.bytes as u64);
        prom_metrics.ping_gauge.set(result.ping.latency);
        thread::sleep(Duration::from_secs(SPEEDTEST_INTERVAL));
    });

    let listener = TcpListener::bind(LISTEN_ADDRESS)?;
    // So the registry can be cloned
    let thread_registry = Arc::new(Mutex::new(registry));
    for client in listener.incoming() {
        let client = client?;
        let mut reader = BufReader::new(client.try_clone()?);
        let mut writer = BufWriter::new(client.try_clone()?);
        let registry = thread_registry.clone();
        thread::spawn(move || {
            let mut request = String::new();
            reader.read_line(&mut request).unwrap();
            match request.contains(&format!("GET {METRIC_ENDPOINT}")) {
                true => encode(&mut writer, &registry.lock().unwrap()).unwrap(),
                false => writeln!(&mut writer, "Invalid request").unwrap(),
            }
        });
    }

    Ok(())
}
