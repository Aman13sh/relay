// crate: server
// file:  forwarder.rs
//
// Reads data in chunks from source and writes to destination.
// Shuts down destination write half when source reaches EOF.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

// Forwards all bytes from source to destination until EOF.
// Closes the destination write half so the remote peer receives EOF.
pub fn forward(mut source: TcpStream, mut destination: TcpStream) {
    let mut buffer = [0u8; 4096];

    loop {
        match source.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                if destination.write_all(&buffer[..bytes_read]).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let _ = destination.shutdown(Shutdown::Write);
}