// crate: client
// file:  config.rs
//
// Client-side configuration.
// Holds where the relay server is and what upstream to request.
// Reads values from environment variables with fallbacks.
//
// Environment variables:
//   RELAY_ADDR — address of the relay server (default: 127.0.0.1:9000)
//   RELAY_DEST — upstream destination target (default: github.com:22)

use std::env;

// All addresses and settings the client needs to connect.
pub struct Config {
    // Address of the relay server running on the host machine.
    pub relay_addr: String,

    // Upstream destination the relay should connect to on our behalf.
    pub destination: String,
}

impl Config {
    // Reads config from env vars, falling back to default values.
    pub fn new() -> Config {
        Config {
            relay_addr: read_env("RELAY_ADDR", "127.0.0.1:9000"),
            destination: read_env("RELAY_DEST", "github.com:22"),
        }
    }
}

// Returns the value of an env var, or the given default if not set.
fn read_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
