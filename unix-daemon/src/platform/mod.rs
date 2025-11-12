use std::net::TcpStream;
use tokio::time::Duration;
pub trait NetworkManager: Send + Sync {
    // fn current_ssid(&self) -> anyhow::Result<Option<String>>;
    fn is_wifi_on(&self) -> anyhow::Result<bool>;
    fn internet_available(&self, time_out_seconds: u64) -> bool {
        let timeout = Duration::from_secs(time_out_seconds);
        TcpStream::connect_timeout(&"8.8.8.8:53".parse().unwrap(), timeout).is_ok()
    }
}

#[cfg(target_os = "macos")]
pub mod macos;
