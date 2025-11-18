use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::error;
use std::process::Command;

pub struct LinuxNetworkManager;

impl LinuxNetworkManager {
    pub fn new() -> Self {
        Self
    }

    fn _get_wifi_device(&self) -> Result<String> {
        let output = Command::new("nmcli").args(["device", "status"]).output()?;

        if !output.status.success() {
            return Err(anyhow!("nmcli failed"));
        }

        let stdout = String::from_utf8(output.stdout)?;
        for line in stdout.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() >= 2 && cols[1] == "wifi" {
                return Ok(cols[0].to_string());
            }
        }

        Err(anyhow!("No Wi-Fi device found"))
    }
}

impl NetworkManager for LinuxNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        match self._get_wifi_device() {
            Ok(dev) => {
                let output = Command::new("nmcli").args(["radio", "wifi"]).output()?;

                if !output.status.success() {
                    return Ok(false);
                }

                let stdout = String::from_utf8(output.stdout)?;
                Ok(stdout.trim() == "enabled")
            }
            Err(e) => {
                error!("error checking adapter power status: {}", e);
                Ok(false)
            }
        }
    }
}

