use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::{error, info, warn};
use std::process::Command;

pub struct LinuxNetworkManager;

impl LinuxNetworkManager {
    pub fn new() -> Self {
        Self
    }

    fn _get_wifi_device(&self) -> Result<String> {
        info!("Attempting to find Wi-Fi device using 'nmcli device status'...");

        let output = Command::new("nmcli").args(["device", "status"]).output()?;

        if !output.status.success() {
            error!("Failed to execute 'nmcli device status' command.");
            return Err(anyhow!("nmcli failed"));
        }

        let stdout = String::from_utf8(output.stdout)?;

        info!("Output of 'nmcli device status':\n{}", stdout);

        for line in stdout.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() >= 2 && cols[1] == "wifi" {
                info!("Found Wi-Fi device: {}", cols[0]);
                return Ok(cols[0].to_string());
            }
        }

        error!("No Wi-Fi device found in 'nmcli device status' output.");
        Err(anyhow!("No Wi-Fi device found"))
    }
}

impl NetworkManager for LinuxNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        info!("Checking if Wi-Fi adapter is on...");

        match self._get_wifi_device() {
            Ok(dev) => {
                info!("Checking the radio status of Wi-Fi device: {}", dev);

                let output = Command::new("nmcli").args(["radio", "wifi"]).output()?;

                if !output.status.success() {
                    error!("Failed to execute 'nmcli radio wifi' for device: {}", dev);
                    return Ok(false);
                }

                let stdout = String::from_utf8(output.stdout)?;

                info!(
                    "Output of 'nmcli radio wifi' for device '{}':\n{}",
                    dev, stdout
                );

                if stdout.trim() == "enabled" {
                    info!("Wi-Fi adapter is ON.");
                    Ok(true)
                } else {
                    warn!("Wi-Fi adapter is OFF.");
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Error occurred while checking Wi-Fi device status: {}", e);
                Ok(false)
            }
        }
    }
}
