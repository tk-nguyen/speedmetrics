use std::process::Command;

use color_eyre::Result;
use tracing::info;
use tracing_subscriber;

#[allow(dead_code)]
mod models;
mod prometheus;
mod server;
mod speedtest;

use prometheus::PromMetrics;
use server::spawn_server;
use speedtest::run_speedtest;

fn main() -> Result<()> {
    let log_format = tracing_subscriber::fmt::format().with_target(false);
    tracing_subscriber::fmt().event_format(log_format).init();
    color_eyre::install()?;

    let prom_metrics = PromMetrics::new();
    let registry = prom_metrics.setup_prometheus();

    info!("Starting speedtest...");
    info!("Running for the first time to accept license...");
    Command::new("speedtest")
        .arg("--accept-license")
        .output()
        .expect("Failed to run speedtest. Please make sure speedtest-cli is in your PATH.");
    info!("Initial setup finished!");

    run_speedtest(prom_metrics)?;
    spawn_server(registry)
}
