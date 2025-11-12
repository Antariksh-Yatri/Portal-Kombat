mod captive;
mod configs;
mod event;
mod fsm;
mod platform;

use anyhow::Result;
use configs::Config;
use fsm::Machine;
use log::{error, info, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    env_logger::init();
    info!("starting wifi-captive-daemon");

    let home_str = env::var("HOME").expect("HOME environment variable not set.");
    let mut config_path = PathBuf::from(home_str);
    config_path.push(".portal-kombat.toml");
    let config_string = fs::read_to_string(config_path).expect("Failed to read config file.");
    let config: Config = toml::from_str(&config_string).expect("Failed to parse config file.");

    let poll_interval = Duration::from_secs(config.refresh);
    let mut m = Machine::new(config);

    loop {
        m.reset();
        thread::sleep(poll_interval);
    }
}
