// crate: server
// file:  config.rs
//
// Server-side configuration.
// Values are read from environment variables with hardcoded fallbacks.
// The forwarding destination is determined per-connection
// by the protocol handshake (ConnectRequest), not by config.
//
// Environment variables:
//   RELAY_LISTEN  — address to bind on (default: 0.0.0.0:9000)

use std::env;

// Static server configuration loaded once at startup.
pub struct Config {
    // TCP address the relay server binds to.
    pub listen_addr: String,
}

impl Config {
    // Reads config from env vars, falling back to 0.0.0.0:9000.
    pub fn new() -> Config {
        Config {
            listen_addr: read_env("RELAY_LISTEN", "0.0.0.0:9000"),
        }
    }
}

// Returns the value of an env var, or the given default if not set.
fn read_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}