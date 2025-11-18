use crate::platform::NetworkManager;
use anyhow::{Result, anyhow};
use log::{debug, error, info, trace, warn};
use std::process::Command;
use std::time::Instant;

pub struct WindowsNetworkManager;

impl WindowsNetworkManager {
    pub fn new() -> Self {
        Self
    }

    fn run_netsh_show_interfaces(&self) -> Result<(String, String, i32)> {
        let start = Instant::now();
        debug!("running: netsh wlan show interfaces");
        let output = Command::new("netsh")
            .args(&["wlan", "show", "interfaces"])
            .output()?;
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

    fn parse_interface_name(&self, netsh_out: &str) -> Option<String> {
        for line in netsh_out.lines() {
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim();
                let val = line[(idx + 1)..].trim();
                if key.eq_ignore_ascii_case("name") && !val.is_empty() {
                    debug!("parsed interface name: '{}'", val);
                    return Some(val.to_string());
                }
            }
        }
        debug!("no 'Name' field found in netsh output");
        None
    }

    fn parse_interface_state(&self, netsh_out: &str) -> Option<String> {
        for line in netsh_out.lines() {
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim();
                let val = line[(idx + 1)..].trim();
                if key.eq_ignore_ascii_case("state") {
                    debug!("parsed interface state: '{}'", val);
                    return Some(val.to_lowercase());
                }
            }
        }
        debug!("no 'State' field found in netsh output");
        None
    }
}

impl NetworkManager for WindowsNetworkManager {
    fn is_adapater_on(&self) -> Result<bool> {
        info!("checking Windows Wi-Fi adapter state");

        let (stdout, stderr, exit_code) = match self.run_netsh_show_interfaces() {
            Ok(t) => t,
            Err(e) => {
                error!("failed to run netsh: {}", e);
                return Ok(false);
            }
        };

        if exit_code != 0 {
            warn!(
                "netsh returned non-zero exit code: {}. stderr_len={}",
                exit_code,
                stderr.len()
            );
        }

        match self.parse_interface_name(&stdout) {
            Some(name) => info!("found Wi-Fi interface name: '{}'", name),
            None => info!("no interface 'Name' parsed — continuing to parse state"),
        }

        match self.parse_interface_state(&stdout) {
            Some(state) => {
                if state.contains("connected") {
                    info!("Wi-Fi interface reports state: '{}', treating as ON", state);
                    Ok(true)
                } else {
                    info!(
                        "Wi-Fi interface reports state: '{}', treating as OFF",
                        state
                    );
                    Ok(false)
                }
            }
            None => {
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
