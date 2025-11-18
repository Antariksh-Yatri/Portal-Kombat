use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::{error, info, warn};
use std::process::Command;

pub struct MacOSNetworkManager;

impl MacOSNetworkManager {
    pub fn new() -> Self {
        Self
    }

    fn _get_adapter_status(&self) -> Result<String> {
        info!("Checking for Wi-Fi adapter status...");

        let output = Command::new("networksetup")
            .arg("-listallhardwareports")
            .output()?;

        if !output.status.success() {
            error!("Failed to execute 'networksetup -listallhardwareports'");
            return Err(anyhow!("networksetup failed"));
        }

        let stdout = String::from_utf8(output.stdout)?;
        let mut lines = stdout.lines();

        info!(
            "Output of 'networksetup -listallhardwareports':\n{}",
            stdout
        );

        while let Some(line) = lines.next() {
            if line.contains("Hardware Port: Wi-Fi") || line.contains("Hardware Port: AirPort") {
                info!("Found potential Wi-Fi port: {}", line);
                for _ in 0..3 {
                    if let Some(l) = lines.next() {
                        if let Some(idx) = l.find("Device:") {
                            let dev = l[(idx + 7)..].trim().to_string();
                            info!("Found Wi-Fi device: {}", dev); // Log the found device
                            return Ok(dev);
                        }
                    }
                }
            }
        }

        error!("No Wi-Fi device found in the 'networksetup' output.");
        Err(anyhow!("No Wi-Fi device found"))
    }
}

impl NetworkManager for MacOSNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        info!("Checking if Wi-Fi adapter is on...");
        match self._get_adapter_status() {
            Ok(dev) => {
                info!("Checking the power status of the Wi-Fi device: {}", dev);
                let output = Command::new("networksetup")
                    .arg("-getairportpower")
                    .arg(&dev)
                    .output()?;

                if !output.status.success() {
                    error!("Failed to get airport power status for device: {}", dev);
                    return Ok(false);
                }

                let stdout = String::from_utf8(output.stdout)?;
                info!("Wi-Fi device power status: {}", stdout);

                if stdout.contains("On") {
                    info!("Wi-Fi adapter is ON.");
                    Ok(true)
                } else {
                    warn!("Wi-Fi adapter is OFF.");
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Error checking adapter power status: {}", e);
                Ok(false)
            }
        }
    }
}
