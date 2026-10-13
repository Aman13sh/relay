// crate: client
// file:  connection.rs
//
// Opens the TCP connection to the relay server and sends
// the protocol handshake (ConnectRequest).
// Returns the open TcpStream ready for data forwarding.

use std::io;
use std::net::TcpStream;

use protocol::codec;
use protocol::message::ConnectRequest;
use protocol::types::SessionKind;

use crate::config::Config;

// Connects to the relay server and performs the protocol handshake.
// After this call the stream is ready for raw data forwarding.
pub fn open(config: &Config) -> io::Result<TcpStream> {
    let mut stream = connect_to_relay(&config.relay_addr)?;
    send_handshake(&mut stream, config)?;
    Ok(stream)
}

// Opens a raw TCP connection to the relay server.
fn connect_to_relay(addr: &str) -> io::Result<TcpStream> {
    TcpStream::connect(addr)
}

// Builds and writes the ConnectRequest so the relay knows where to forward.
fn send_handshake(stream: &mut TcpStream, config: &Config) -> io::Result<()> {
    let request = ConnectRequest::new(SessionKind::Git, &config.destination);
    codec::write_connect_request(stream, &request)
}
