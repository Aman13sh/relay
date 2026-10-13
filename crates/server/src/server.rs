use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::config::Config;
use crate::connection;

pub fn run() {
    let config = Arc::new(Config::new());

    let listener = TcpListener::bind(&config.listen_addr)
        .expect("Failed to bind Relay server");

    println!(
        "Relay server listening on {}",
        config.listen_addr
    );

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                let config = Arc::clone(&config);

                thread::spawn(move || {
                    connection::handle(client, &config);
                });
            }

            Err(error) => {
                eprintln!("Connection failed: {error}");
            }
        }
    }
}