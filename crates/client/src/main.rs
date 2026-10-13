// crate: client
// file:  main.rs
//
// Entry point for the relay client binary.
// Loads config and delegates everything to client::run.

mod client;
mod config;
mod connection;

fn main() {
    let config = config::Config::new();
    client::run(&config);
}