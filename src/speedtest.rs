use std::{
    io::Read,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use super::{models::SpeedtestResult, prometheus::PromMetrics};
use color_eyre::Result;
use tracing::{error, info};

// 1 minute between speed tests
const SPEEDTEST_INTERVAL: u64 = 60;

pub fn run_speedtest(prom_metrics: PromMetrics) -> Result<()> {
    thread::spawn(move || loop {
        info!("Measuring internet speed...");
        let mut child = Command::new("speedtest")
            .args(&["-f", "json"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run speedtest. Please make sure speedtest-cli is in your PATH.");
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    match status.success() {
                        true => info!("Measuring complete!"),
                        false => {
                            let mut error_output = String::new();
                            child
                                .stderr
                                .expect("Cannot get the stderr of speedtest-cli!")
                                .read_to_string(&mut error_output)
                                .expect("Cannot read the stderr of speedtest-cli!");
                            error!("Measuring did not succeed. Error: {error_output}");
                        }
                    }
                    break;
                }
                Ok(None) => (),
                Err(e) => {
                    error!("Problem waiting for the output of the speedtest: {e}");
                }
            }
        }

        let result: SpeedtestResult = serde_json::from_reader(
            child
                .stdout
                .expect("Cannot get the stdout of speedtest-cli!"),
        )
        .expect("Invalid JSON output from speedtest-cli");
        prom_metrics.upload_gauge.set(result.upload.bytes as u64);
        prom_metrics
            .download_gauge
            .set(result.download.bytes as u64);
        prom_metrics.ping_gauge.set(result.ping.latency);
        thread::sleep(Duration::from_secs(SPEEDTEST_INTERVAL));
    });
    Ok(())
}
