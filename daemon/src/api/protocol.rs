use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub body: Option<serde_json::Value>,
}

impl Response {
    pub fn new(status: u16, body: Option<serde_json::Value>) -> Self {
        Self { status, body }
    }

    pub fn ok(body: serde_json::Value) -> Self {
        Self::new(200, Some(body))
    }

    pub fn error(status: u16, message: &str) -> Self {
        Self::new(
            status,
            Some(serde_json::json!({ "error": message })),
        )
    }
}

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use crate::api::transport::TransportStream;
use anyhow::{anyhow, Context};

pub async fn parse_request<S: TransportStream>(stream: &mut S) -> anyhow::Result<Request> {
    // Wrap stream in BufReader for line-based reading
    let mut reader = BufReader::new(stream);
    
    // 1. Read Request Line: METHOD PATH V1
    let mut line = String::new();
    reader.read_line(&mut line).await.context("Failed to read request line")?;
    
    if line.is_empty() {
        return Err(anyhow!("Empty request"));
    }

    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid request line: {:?}", line));
    }

    let method = match parts[0].to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        _ => return Err(anyhow!("Unsupported method: {}", parts[0])),
    };

    let path = parts[1].to_string();

    // 2. Read Headers (we only care about Content-Length for now)
    let mut content_length = 0;
    loop {
        line.clear();
        reader.read_line(&mut line).await?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break; // End of headers
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            if key.trim().eq_ignore_ascii_case("Content-Length") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
    }

    // 3. Read Body if Content-Length > 0
    let body = if content_length > 0 {
        let mut buffer = vec![0u8; content_length];
        reader.read_exact(&mut buffer).await?;
        let json_val: serde_json::Value = serde_json::from_slice(&buffer)?;
        Some(json_val)
    } else {
        None
    };

    Ok(Request { method, path, body })
}

pub async fn write_response<S: TransportStream>(stream: &mut S, response: Response) -> anyhow::Result<()> {
    let body_str = if let Some(body) = &response.body {
        serde_json::to_string(body)?
    } else {
        String::new()
    };

    let status_line = format!("V1 {}\r\n", response.status);
    let len_header = format!("Content-Length: {}\r\n", body_str.len());
    let type_header = "Content-Type: application/json\r\n";
    
    let mut data = Vec::new();
    data.extend_from_slice(status_line.as_bytes());
    data.extend_from_slice(len_header.as_bytes());
    data.extend_from_slice(type_header.as_bytes());
    data.extend_from_slice(b"\r\n"); // End of headers
    data.extend_from_slice(body_str.as_bytes());

    stream.write_all(&data).await?;
    stream.flush().await?;
    Ok(())
}
