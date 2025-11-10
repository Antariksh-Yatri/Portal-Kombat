pub trait NetworkManager: Send + Sync {
    /// Get the currently connected wifi SSID (if any).
    fn current_ssid(&self) -> anyhow::Result<Option<String>>;

    /// Platform-specific helper to open a URL in a browser if required.
    fn open_url(&self, url: &str) -> anyhow::Result<()>;
}

#[cfg(target_os = "macos")]
pub mod macos;
