mod captive;
mod configs;
mod event;
mod fsm;
mod platform;

use configs::Config;
use fsm::Machine;
use log::info;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::platform::NetworkManager;
use crate::platform::linux::LinuxNetworkManager;

fn main() {
    env_logger::init();
    info!("starting wifi-captive-daemon");
    let nm: Box<dyn NetworkManager> = Box::new(LinuxNetworkManager::new());
    nm.is_adapater_on();
    // #[cfg(any(target_os = "linux", target_os = "macos"))]
    // let home_str = std::env::var("HOME").expect("HOME env variable not set");
    //
    // #[cfg(target_os = "windows")]
    // let home_str = std::env::var("USERPROFILE").expect("USERPROFILE env variable not set");
    // let mut config_path = PathBuf::from(home_str);
    //
    // config_path.push(".portalkombatd.toml");
    // let config_string = fs::read_to_string(config_path).expect("Failed to read config file.");
    // let config: Config = toml::from_str(&config_string).expect("Failed to parse config file.");
    // let poll_interval = Duration::from_secs(config.refresh);
    // let mut m = Machine::new(config);
    //
    // loop {
    //     m.reset();
    //     thread::sleep(poll_interval);
    // }
}
