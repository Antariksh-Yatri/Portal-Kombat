mod captive;
mod configs;
mod platform;

use anyhow::Result;
use captive::CaptiveDetector;
use configs::Config;
use log::{error, info, warn};
use platform::NetworkManager;
use std::env;
use std::fs;
use std::path::PathBuf;
use tokio::time::{Duration, sleep};

#[cfg(target_os = "macos")]
use platform::macos::MacOSNetworkManager;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("starting wifi-captive-daemon");

    let home_str = env::var("HOME").expect("HOME environment variable not set.");
    let mut config_path = PathBuf::from(home_str);
    config_path.push(".portal-kombat.toml");
    let config_string = fs::read_to_string(config_path).expect("Failed to read config file.");
    let config: Config = toml::from_str(&config_string).expect("Failed to parse config file.");

    #[cfg(target_os = "macos")]
    let nm: Box<dyn NetworkManager> = Box::new(MacOSNetworkManager::new());
    #[cfg(not(target_os = "macos"))]
    compile_error!("No network manager implemented for this platform yet");

    let detector = CaptiveDetector::new(10);
    let poll_interval = Duration::from_secs(1);

    loop {
        if let Err(e) = check_once(&*nm, &detector).await {
            error!("check failed: {:?}", e);
        }
        sleep(poll_interval).await;
    }
}

async fn check_once(nm: &dyn NetworkManager, detector: &CaptiveDetector) -> Result<()> {
    match nm.current_ssid() {
        Ok(Some(ssid)) => {
            info!("connected SSID: {}", ssid);
            let (is_captive, redirect) = detector.probe().await?;
            if is_captive {
                info!("captive portal detected");
                if let Some(url) = redirect {
                    println!("opening redirect URL: {}", url);
                    if let Err(e) = nm.open_url(&url) {
                        warn!("failed to open url {}: {:?}", url, e);
                    }
                } else {
                    println!("no redirect captured; opening generic captive portal URL");
                    // Some captive portals don't redirect; open probe URL to trigger captive UI
                    if let Err(e) = nm.open_url(&detector.probe_url) {
                        warn!("failed to open probe url: {:?}", e);
                    }
                }
            } else {
                println!("no captive portal");
            }
        }
        Ok(None) => {
            println!("not connected to Wi-Fi");
        }
        Err(e) => {
            println!("failed to get SSID: {:?}", e);
        }
    }
    Ok(())
}
