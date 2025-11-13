use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::error;
use std::process::Command;

pub struct MacOSNetworkManager;

impl MacOSNetworkManager {
    pub fn new() -> Self {
        Self
    }
    fn _get_wifi_device(&self) -> Result<String> {
        let output = Command::new("networksetup")
            .arg("-listallhardwareports")
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("networksetup failed"));
        }
        let stdout = String::from_utf8(output.stdout)?;
        let mut lines = stdout.lines();
        while let Some(line) = lines.next() {
            if line.contains("Hardware Port: Wi-Fi") || line.contains("Hardware Port: AirPort") {
                for _ in 0..3 {
                    if let Some(l) = lines.next() {
                        if let Some(idx) = l.find("Device:") {
                            let dev = l[(idx + 7)..].trim().to_string();
                            return Ok(dev);
                        }
                    }
                }
            }
        }
        Err(anyhow!("No Wi-Fi device found"))
    }
}

impl NetworkManager for MacOSNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        match self._get_wifi_device() {
            Ok(dev) => {
                let output = Command::new("networksetup")
                    .arg("-getairportpower")
                    .arg(&dev)
                    .output()?;

                if !output.status.success() {
                    return Ok(false);
                }

                let stdout = String::from_utf8(output.stdout)?;
                Ok(stdout.contains("On"))
            }
            Err(e) => {
                error!("error checking adapter power status: {}", e);
                Ok(false)
            }
        }
    }
}
