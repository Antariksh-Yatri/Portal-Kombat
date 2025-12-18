#[cfg(unix)]
use crate::api::transport::TransportListener;
#[cfg(unix)]
use async_trait::async_trait;
#[cfg(unix)]
use log::{info, warn};
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

#[cfg(unix)]
pub struct UnixTransportListener {
    listener: UnixListener,
    path: PathBuf,
}

#[cfg(unix)]
impl UnixTransportListener {
    pub fn bind(path: PathBuf) -> anyhow::Result<Self> {
        if path.exists() {
            warn!("Removing existing socket file: {:?}", path);
            std::fs::remove_file(&path)?;
        }

        let listener = UnixListener::bind(&path)?;
        info!("Listening on unix socket: {:?}", path);
        
        // Set permissions to 700 so only owner can access
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_mode(0o700);
        std::fs::set_permissions(&path, perms)?;

        Ok(Self { listener, path })
    }
}

#[cfg(unix)]
#[async_trait]
impl TransportListener for UnixTransportListener {
    type Stream = UnixStream;

    async fn accept(&mut self) -> anyhow::Result<Self::Stream> {
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }
}

#[cfg(unix)]
impl Drop for UnixTransportListener {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
