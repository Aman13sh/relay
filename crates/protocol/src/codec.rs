// crate: protocol
// file:  codec.rs
//
// Encodes and decodes protocol messages to/from a TcpStream.
// Each function handles exactly one message type, one direction.
// No business logic here — only serialization and I/O.

use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::message::ConnectRequest;
use crate::types::SessionKind;

// Writes a ConnectRequest onto the stream.
//
// Layout written:
//   [1 byte]  session_kind byte
//   [4 bytes] destination string length (u32, big-endian)
//   [N bytes] destination string (UTF-8)
pub fn write_connect_request(stream: &mut TcpStream, request: &ConnectRequest) -> io::Result<()> {
    let dest_bytes = request.destination.as_bytes();
    let dest_len   = dest_bytes.len() as u32;

    stream.write_all(&[request.kind.as_byte()])?;
    stream.write_all(&dest_len.to_be_bytes())?;
    stream.write_all(dest_bytes)?;

    Ok(())
}

// Reads a ConnectRequest from the stream.
//
// Reads the session kind byte, then the destination length,
// then the destination string. Returns Err on any I/O failure
// or if the session kind byte is not recognised.
pub fn read_connect_request(stream: &mut TcpStream) -> io::Result<ConnectRequest> {
    let kind        = read_session_kind(stream)?;
    let destination = read_destination(stream)?;

    Ok(ConnectRequest { kind, destination })
}

// Reads one byte and converts it to a SessionKind.
// Returns an InvalidData error if the byte is unknown.
fn read_session_kind(stream: &mut TcpStream) -> io::Result<SessionKind> {
    let mut buf = [0u8; 1];
    stream.read_exact(&mut buf)?;

    SessionKind::from_byte(buf[0]).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unknown session kind byte: 0x{:02x}", buf[0]),
        )
    })
}

// Reads a 4-byte big-endian length then that many bytes as a UTF-8 string.
// Returns an InvalidData error if the bytes are not valid UTF-8.
fn read_destination(stream: &mut TcpStream) -> io::Result<String> {
    let len  = read_u32_be(stream)? as usize;
    let mut  buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "destination is not valid UTF-8")
    })
}

// Reads exactly 4 bytes and interprets them as a big-endian u32.
fn read_u32_be(stream: &mut TcpStream) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}
