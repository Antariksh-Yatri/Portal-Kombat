use crate::platform::NetworkManager;
use anyhow::{Context, Result};
use std::process::Command;
use std::str;

pub struct MacOSNetworkManager;

impl MacOSNetworkManager {
    pub fn new() -> Self { Self }
}

impl NetworkManager for MacOSNetworkManager {
    fn current_ssid(&self) -> Result<Option<String>> {
        // Pragmatic approach: call the airport command-line tool.
        // Path may vary; this is the usual private tool location.
        // This is brittle — replace with CoreWLAN bindings later.
        println!("getting current ssid");
        return Ok(Some("IIITKottayam".to_string()))
        // let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        //     .arg("-I")
        //     .output()
        //     .context("failed to run airport -I")?;

        // if !output.status.success() {
        //     // Not connected to Wi-Fi or airport absent
        //     return Ok(None);
        // }

        // let stdout = str::from_utf8(&output.stdout).context("airport output not utf8")?;
        // // Parse for "SSID: <name>"
        // for line in stdout.lines() {
        //     let line = line.trim();
        //     if let Some(rem) = line.strip_prefix("SSID: ") {
        //         return Ok(Some(rem.to_string()));
        //     }
        // }
        // Ok(None)
    }

    fn open_url(&self, url: &str) -> Result<()> {
        // Use `open` to open URL in default browser
        Ok(())
        // let status = Command::new("open")
        //     .arg(url)
        //     .status()
        //     .context("failed to run open command")?;
        // if status.success() {
        //     Ok(())
        // } else {
        //     Err(anyhow::anyhow!("open returned non-zero status"))
        // }
    }
}
