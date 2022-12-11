use std::{
    io::Read,
    process::{Command, Stdio},
};

use crate::models::SpeedtestResult;
use color_eyre::{eyre::eyre, Result};
use tracing::{error, info};

pub fn run_speedtest() -> Result<SpeedtestResult> {
    info!("Measuring internet speed...");
    let mut child = Command::new("speedtest")
        .args(["-f", "json"])
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
                            .ok_or_else(|| eyre!("Cannot get the stderr!"))?
                            .read_to_string(&mut error_output)?;
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

    let result = serde_json::from_reader(child.stdout.unwrap())?;

    Ok(result)
}
