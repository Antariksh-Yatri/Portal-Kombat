use crate::platform::NetworkManager;
use anyhow::{Context, Result};
use std::process::Command;
use std::str;

pub struct MacOSNetworkManager;

impl MacOSNetworkManager {
    pub fn new() -> Self {
        Self
    }
}

impl NetworkManager for MacOSNetworkManager {
    // fn current_ssid(&self) -> Result<Option<String>> {
    //     println!("getting current ssid");
    //     return Ok(Some("IIITKottayam".to_string()));
    // }
    fn is_wifi_on(&self) -> Result<bool> {
        let output = Command::new("networksetup")
            .arg("-getairportpower")
            .arg("en0")
            .output()
            .expect("failed to check wifi power status");
        if !output.status.success() {
            return Ok(false);
        }
        let stdout = str::from_utf8(&output.stdout).expect("networksetup output decoding failed");
        if stdout.contains("On") {
            return Ok(true);
        }
        Ok(false)
    }
}
