#[cfg(windows)]
use crate::api::transport::{TransportListener, TransportStream};
#[cfg(windows)]
use async_trait::async_trait;
#[cfg(windows)]
use tokio::net::windows::named_pipe::{ServerOptions, NamedPipeServer};
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
#[cfg(windows)]
use std::pin::Pin;
#[cfg(windows)]
use std::task::{Context, Poll};
#[cfg(windows)]
use std::sync::Arc;
#[cfg(windows)]
use tokio::sync::Mutex;

#[cfg(windows)]
pub struct WindowsTransportListener {
    pipe_name: OsString,
    // We need a mechanism to accept connections. 
    // Named pipes in Windows work differently; you create an instance, connect it, 
    // and when a client connects, you hand that off and create a NEW instance for the next client.
    // However, the trait assumes `accept` returns a connected stream.
    // So `accept` will create a new pipe instance (or use a pre-created one), wait for connection, and return it.
}

#[cfg(windows)]
impl WindowsTransportListener {
    pub fn bind(name: &str) -> anyhow::Result<Self> {
        let pipe_name = OsString::from(format!(r"\\.\pipe\{}", name));
        Ok(Self { pipe_name })
    }
}

#[cfg(windows)]
#[async_trait]
impl TransportListener for WindowsTransportListener {
    type Stream = NamedPipeServer;

    async fn accept(&mut self) -> anyhow::Result<Self::Stream> {
        // Create a new instance of the named pipe
        let server = ServerOptions::new()
            .first_pipe_instance(false)
            .create(&self.pipe_name)?;

        // Wait for a client to connect
        server.connect().await?;
        
        Ok(server)
    }
}
