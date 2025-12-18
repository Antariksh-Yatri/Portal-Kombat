use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

// Marker trait for the underlying transport stream.
// Must be AsyncRead + AsyncWrite + Send + Unpin to be usable by tokio helpers.
pub trait TransportStream: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

// Blanket implementation for any type that satisfies the bounds.
impl<T> TransportStream for T where T: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

#[async_trait]
pub trait TransportListener {
    type Stream: TransportStream;

    // Accept a new connection.
    async fn accept(&mut self) -> anyhow::Result<Self::Stream>;
}
