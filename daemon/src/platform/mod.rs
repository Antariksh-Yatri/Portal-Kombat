use log::{error, info};
use std::net::TcpStream;
use tokio::time::Duration;

pub trait NetworkManager: Send + Sync {
    // fn current_ssid(&self) -> anyhow::Result<Option<String>>;
    fn is_adapater_on(&self) -> anyhow::Result<bool>;
    fn internet_available(&self, time_out_seconds: u64) -> bool {
        let timeout = Duration::from_secs(time_out_seconds);
        let result = TcpStream::connect_timeout(&"8.8.8.8:53".parse().unwrap(), timeout);
        match result {
            Ok(_) => {
                info!("Internet is available, successfully connected to 8.8.8.8:53");
                true
            }
            Err(e) => {
                error!("Internet availability check failed: {}", e);
                false
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub mod macos;

// #[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;
