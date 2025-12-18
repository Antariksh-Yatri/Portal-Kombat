use crate::api::protocol::{parse_request, write_response, Method, Request, Response};
use crate::api::transport::{TransportListener, TransportStream};
use log::{error, info};
use tokio::task;

pub async fn run_server<L>(mut listener: L) -> anyhow::Result<()>
where
    L: TransportListener + Send + Sync + 'static,
    L::Stream: Send,
{
    loop {
        match listener.accept().await {
            Ok(stream) => {
                task::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        error!("Connection error: {:?}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {:?}", e);
                // Optional: add backoff if accept fails repeatedly
            }
        }
    }
}

async fn handle_connection<S: TransportStream>(mut stream: S) -> anyhow::Result<()> {
    match parse_request(&mut stream).await {
        Ok(req) => {
            info!("Received request: {:?} {}", req.method, req.path);
            let response = handle_request(req);
            write_response(&mut stream, response).await?;
        }
        Err(e) => {
            error!("Failed to parse request: {:?}", e);
            let response = Response::error(400, "Invalid request");
            let _ = write_response(&mut stream, response).await;
        }
    }
    Ok(())
}

fn handle_request(req: Request) -> Response {
    if req.path == "/v1/status" && matches!(req.method, Method::GET) {
        return Response::ok(serde_json::json!({ "status": "running" }));
    }
    
    Response::error(404, "Not Found")
}
