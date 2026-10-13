// crate: client
// file:  client.rs
//
// Core client logic: connects to the relay via protocol handshake,
// then pipes local stdin → relay and relay → local stdout.
// This is what makes the relay transparent to git and SSH.

use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;

use crate::config::Config;
use crate::connection;

// Runs the relay client session end-to-end.
// Connects to relay, sends handshake, and pipes stdio bidirectionally.
pub fn run(config: &Config) {
    let stream = connection::open(config)
        .expect("client.rs: failed to connect to relay");

    eprintln!("Connected to relay at {}", config.relay_addr);
    eprintln!("Relay will forward to: {}", config.destination);

    pipe_stdio(stream);
}

// Spawns two threads to splice stdin/stdout with the relay stream.
// Exits once the relay-to-stdout stream finishes.
fn pipe_stdio(stream: TcpStream) {
    let stream_for_write = stream.try_clone()
        .expect("client.rs: failed to clone stream for stdin pipe");

    thread::spawn(move || pipe_stdin_to_stream(stream_for_write));
    let t2 = thread::spawn(move || pipe_stream_to_stdout(stream));

    t2.join().expect("client.rs: relay→stdout thread panicked");
}

// Reads from stdin and writes every byte to the relay stream.
// Sends TCP FIN to relay when stdin closes.
fn pipe_stdin_to_stream(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let mut stdin = io::stdin();

    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if stream.write_all(&buf[..n]).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let _ = stream.shutdown(Shutdown::Write);
}

// Reads from the relay stream and writes every byte to stdout.
// Stops when the relay closes the connection.
fn pipe_stream_to_stdout(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let mut stdout = io::stdout();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if stdout.write_all(&buf[..n]).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
