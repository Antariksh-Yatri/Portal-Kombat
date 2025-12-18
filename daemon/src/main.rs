mod api;
mod captive;
mod configs;
mod event;
mod fsm;
mod platform;

use configs::Config;
use fsm::Machine;
use log::info;

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use crate::api::server::run_server;

#[cfg(unix)]
use crate::api::platform::unix::UnixTransportListener;
#[cfg(windows)]
use crate::api::platform::windows::WindowsTransportListener;

fn main() {
    env_logger::init();
    info!("starting wifi-captive-daemon");
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let home_str = std::env::var("HOME").expect("HOME env variable not set");

    #[cfg(target_os = "windows")]
    let home_str = std::env::var("USERPROFILE").expect("USERPROFILE env variable not set");
    let mut config_path = PathBuf::from(home_str);

    config_path.push(".portalkombatd.toml");
    let config_string = fs::read_to_string(config_path).expect("Failed to read config file.");
    let config: Config = toml::from_str(&config_string).expect("Failed to parse config file.");
    let poll_interval = Duration::from_secs(config.refresh);
    
    // Start API Server in a separate thread with its own Runtime
    // This avoids conflict between reqwest::blocking (used in Machine) and tokio runtime.
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime");
        
        rt.block_on(async {
            #[cfg(unix)]
            {
                let socket_path = PathBuf::from("/tmp/portalkombat.sock");
                match UnixTransportListener::bind(socket_path) {
                    Ok(listener) => {
                        if let Err(e) = run_server(listener).await {
                            log::error!("API server error: {:?}", e);
                        }
                    }
                    Err(e) => log::error!("Failed to bind socket: {:?}", e),
                }
            }

            #[cfg(windows)]
            {
                match WindowsTransportListener::bind("portalkombat") {
                    Ok(listener) => {
                        if let Err(e) = run_server(listener).await {
                            log::error!("API server error: {:?}", e);
                        }
                    }
                    Err(e) => log::error!("Failed to bind pipe: {:?}", e),
                }
            }
        });
    });

    let mut m = Machine::new(config);

    loop {
        m.reset();
        std::thread::sleep(poll_interval);
    }
}
