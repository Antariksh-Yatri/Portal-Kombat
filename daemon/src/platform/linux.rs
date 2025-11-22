use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::{debug, error, info, trace, warn};
use std::process::Command;
use std::time::Instant;

pub struct LinuxNetworkManager;

impl LinuxNetworkManager {
    pub fn new() -> Self {
        Self
    }
    fn get_interface_details(&self) -> Result<(String, String, i32)> {
        let start = Instant::now();
        debug!("running: netsh wlan show interfaces");
        let output = Command::new("nmcli").args(&["device", "status"]).output()?;
        let duration = start.elapsed();
        let code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        debug!(
            "netsh finished: exit={} time_ms={} stdout_len={} stderr_len={}",
            code,
            duration.as_millis(),
            stdout.len(),
            stderr.len()
        );
        trace!("netsh stdout:\n{}", stdout);
        if !stderr.trim().is_empty() {
            trace!("netsh stderr:\n{}", stderr);
        }
        Ok((stdout, stderr, code))
    }

    fn parse_interfaces(&self, nmcli_out: &str) -> Option<Vec<(String, String)>> {
        let mut interface_states: Vec<(String, String)> = Vec::new();
        for line in nmcli_out.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if matches!(parts[1], "wifi" | "ethernet") {
                interface_states.push((parts[0].to_string(), parts[2].to_string()))
            }
        }
        debug!("no 'Name' field found in netsh output");
        if interface_states.is_empty() {
            None
        } else {
            Some(interface_states)
        }
    }
}

impl NetworkManager for LinuxNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        info!("checking Wi-Fi adapter state");
        let (stdout, stderr, exit_code) = match self.get_interface_details() {
            Ok(t) => t,
            Err(e) => {
                error!("failed to run netsh: {}", e);
                return Ok(false);
            }
        };
        if exit_code != 0 {
            warn!(
                "nmcli returned non-zero exit code: {}. stderr_len={}",
                exit_code,
                stderr.len()
            );
        }
        match self.parse_interfaces(&stdout) {
            Some(interfaces) => {
                for (name, state) in interfaces.iter() {
                    info!("found interface: '{}'", name);
                    if state.to_lowercase().contains("connected") {
                        info!("interface reports state: '{}', treating as ON", state);
                        return Ok(true);
                    } else {
                        info!("interface reports state: '{}', treating as OFF", state);
                    }
                }
                Ok(false)
            }
            None => {
                info!("no interface 'Name' parsed — continuing to parse state");
                warn!(
                    "couldn't parse interface 'State' from netsh output; dumping snippet for diagnosis"
                );
                let snippet: String = stdout.lines().take(20).collect::<Vec<&str>>().join("\n");
                debug!("netsh stdout snippet:\n{}", snippet);
                Ok(false)
            }
        }
    }
}
