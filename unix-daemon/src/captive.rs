use anyhow::{Context, Result};
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

pub struct CaptiveDetector {
    client: Client,
    pub probe_url: String,
    timeout: Duration,
}

impl CaptiveDetector {
    pub fn new(timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("reqwest client");
        Self {
            client,
            probe_url: "http://connectivitycheck.gstatic.com/generate_204".to_string(),
            timeout: Duration::from_secs(timeout),
        }
    }

    /// Returns (is_captive, optional_redirect_url)
    pub async fn probe(&self) -> Result<(bool, Option<String>)> {
        let resp = self
            .client
            .get(&self.probe_url)
            .send()
            .await
            .context("probe request failed")?;

        if resp.status().is_redirection() {
            if let Some(loc) = resp.headers().get(reqwest::header::LOCATION) {
                println!("{:?}", loc);
                return Ok((true, Some(loc.to_str()?.to_string())));
            } else {
                return Ok((true, None));
            }
        }
        let status = resp.status();
        if status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            let re = Regex::new(r"(?P<url>http?://(?P<domain>[^/]+))").unwrap();
            match re.captures(&body) {
                Some(caps) => Ok((true, Some(caps["url"].to_string()))),
                None => Ok((false, None)),
            }
        } else {
            Ok((false, None))
        }
    }
}
