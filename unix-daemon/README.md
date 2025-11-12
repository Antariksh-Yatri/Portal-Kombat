# WiFi Captive Portal Daemon

A cross-platform daemon that automatically detects and authenticates with WiFi captive portals. Built with Rust for reliability and performance.

## Features

- 🔍 **Automatic Detection**: Probes for captive portals using standard connectivity checks
- 🔐 **Auto-Authentication**: Automatically logs in using configured credentials
- 🔄 **State Machine**: Robust state management for handling various portal states
- 🖥️ **Cross-Platform**: Supports macOS (with Linux support planned)
- 📡 **Network Monitoring**: Continuously monitors WiFi status and internet connectivity
- 🛡️ **Error Handling**: Handles various portal responses (max concurrent, wrong credentials, etc.)

## Requirements

- Rust 1.70+ (2024 edition)
- macOS 10.13+ (for macOS support)
- Network access to configure WiFi settings

## Installation

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd unix-daemon

# Build the project
cargo build --release

# The binary will be at target/release/wifi-captive-daemon
```

## Configuration

Create a configuration file at `~/.portal-kombat.toml` with the following structure:

```toml
refresh = 1          # Refresh interval in seconds
timeouts = 5         # Request timeout in seconds

[profile]
rollno = "your_roll_number"
password = "your_password"
```

### Configuration Options

- `refresh`: How often (in seconds) the daemon checks for captive portals
- `timeouts`: HTTP request timeout in seconds
- `profile.rollno`: Your username/roll number for portal authentication
- `profile.password`: Your password for portal authentication

## Usage

### Running as a Standalone Process

```bash
# Run directly
./target/release/wifi-captive-daemon

# Or with cargo
cargo run --release
```

### Running as a Daemon


#### macOS (LaunchDaemon)

>  [!IMPORTANT]  
> It is adviced to disable the default mac captive handler using the command below ```sudo defaults write /Library/Preferences/SystemConfiguration/com.apple.captive.control Active -boolean false```

1. Copy the plist file to LaunchDaemons:
   ```bash
   sudo cp resources/com.example.wifidaemon.plist /Library/LaunchDaemons/
   ```

2. Update the plist file with the correct path to your binary

3. Load the daemon:
   ```bash
   sudo launchctl load /Library/LaunchDaemons/com.example.wifidaemon.plist
   ```

4. Check status:
   ```bash
   sudo launchctl list | grep wifidaemon
   ```

#### Linux (systemd)

1. Copy the service file:
   ```bash
   sudo cp resources/wifi-captive-daemon.service /etc/systemd/system/
   ```

2. Update the service file with the correct path to your binary

3. Enable and start the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable wifi-captive-daemon.service
   sudo systemctl start wifi-captive-daemon.service
   ```

4. Check status:
   ```bash
   sudo systemctl status wifi-captive-daemon.service
   ```

## How It Works

The daemon uses a state machine to manage the authentication flow [refer more details]():

### Portal Detection

The daemon uses Google's connectivity check (`http://connectivitycheck.gstatic.com/generate_204`) to detect captive portals. If a redirect is detected or a portal URL is found in the response, it proceeds with authentication.

## Project Structure

```
.
├── src/
│   ├── main.rs          # Entry point and main loop
│   ├── captive.rs       # Captive portal detection and authentication
│   ├── configs.rs       # Configuration structures
│   ├── event.rs         # Event definitions
│   ├── fsm.rs           # Finite state machine implementation
│   └── platform/
│       ├── mod.rs       # Platform abstraction trait
│       └── macos.rs     # macOS-specific network management
├── resources/
│   ├── com.example.wifidaemon.plist  # macOS LaunchDaemon plist
│   └── wifi-captive-daemon.service   # Linux systemd service file
└── Cargo.toml           # Rust project configuration
```

## Dependencies

- **tokio**: Async runtime (currently using blocking mode)
- **reqwest**: HTTP client for portal communication
- **scraper**: HTML parsing for form extraction
- **regex**: Pattern matching for portal responses
- **serde/toml**: Configuration file parsing
- **anyhow/thiserror**: Error handling
- **log/env_logger**: Logging

## Logging

The daemon uses the `log` crate with `env_logger`. Set the log level using the `RUST_LOG` environment variable:

```bash
# Debug logging
RUST_LOG=debug cargo run

# Info logging (default)
RUST_LOG=info cargo run

# Error logging only
RUST_LOG=error cargo run
```

## Troubleshooting

### Daemon not starting

- Check that the configuration file exists at `~/.portal-kombat.toml`
- Verify the configuration file format is valid TOML
- Check system logs for errors:
  - macOS: `log show --predicate 'process == "wifi-captive-daemon"' --last 5m`
  - Linux: `journalctl -u wifi-captive-daemon.service -n 50`

### Authentication failing

- Verify credentials in the configuration file
- Check network connectivity
- Review logs for specific error messages (max concurrent, wrong credentials, etc.)

### WiFi status not detected

- Ensure proper permissions for network status checks
- On macOS, verify `networksetup` command is available
- Check that the network interface name is correct (default: `en0`)

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```