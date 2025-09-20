use daemonize::Daemonize;
use log::{error, info};
use std::fs::File;
use std::io::Write;
use std::{thread, time::Duration};

enum LoginState {
    CONNECTED,
    LOGGEDIN,
    LOGGEDOUT,
    MAXCONCURRENT,
    AUTHFAILED,
    UNKNOWN,
    CREDUNAVAILABLE,
    WIFINOTCONNECTED,
    AVAILABLE,
}
fn main() {
    env_logger::init();

    // Daemonize the process
    let daemonize = Daemonize::new()
        .pid_file("/tmp/rust_daemon.pid") // File to store the daemon's PID
        .chown_pid_file(true)
        .umask(0o027) // Set file permissions
        .working_directory("/tmp")
        .exit_action(|| info!("Daemon exited."))
        .privileged_action(|| info!("Daemon started."));

    match daemonize.start() {
        Ok(_) => {
            info!("Daemon started successfully!");

            // Your daemon's main loop
            loop {
                // Write some activity to a log file
                if let Err(e) = write_log("Daemon is running...") {
                    error!("Error writing log: {}", e);
                }

                // Sleep for a while before repeating
                thread::sleep(Duration::from_secs(60));
            }
        }
        Err(e) => {
            eprintln!("Failed to start daemon: {}", e);
            std::process::exit(1);
        }
    }
}

fn write_log(message: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("/tmp/daemon.log")?;
    writeln!(file, "{}", message)?;
    Ok(())
}
