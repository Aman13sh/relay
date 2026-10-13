// crate: protocol  (integration tests)
// file:  tests/codec_test.rs
//
// Tests for protocol::codec encode/decode logic.
// Uses real loopback TCP sockets — no mocks needed.
// Run with: cargo test -p protocol

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

use protocol::codec;
use protocol::message::ConnectRequest;
use protocol::types::SessionKind;

// Helper: binds a listener on a random free port and returns (listener, addr).
// Port 0 tells the OS to pick any free port — tests never conflict with each other.
fn bind_free_port() -> (TcpListener, std::net::SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("test helper: failed to bind listener");
    let addr = listener.local_addr()
        .expect("test helper: failed to get local addr");
    (listener, addr)
}

// Verifies that writing then reading a ConnectRequest over a real socket
// produces the exact same kind and destination — full encode/decode roundtrip.
#[test]
fn roundtrip_connect_request() {
    let (listener, addr) = bind_free_port();

    // Server thread: accept one connection, decode the request, return it.
    let server = thread::spawn(move || {
        let (mut conn, _) = listener.accept()
            .expect("roundtrip: accept failed");
        codec::read_connect_request(&mut conn)
            .expect("roundtrip: read_connect_request failed")
    });

    // Client side: connect, encode and send the request.
    let mut client = TcpStream::connect(addr)
        .expect("roundtrip: failed to connect to listener");
    let request = ConnectRequest::new(SessionKind::Git, "github.com:22");
    codec::write_connect_request(&mut client, &request)
        .expect("roundtrip: write_connect_request failed");

    // Drop so server thread sees EOF and returns cleanly.
    drop(client);

    let received = server.join().expect("roundtrip: server thread panicked");

    assert_eq!(received.kind,        SessionKind::Git);
    assert_eq!(received.destination, "github.com:22");
}

// Verifies that a completely unknown session kind byte produces an Err,
// not a panic or a silently wrong SessionKind value.
#[test]
fn rejects_unknown_session_kind() {
    let (listener, addr) = bind_free_port();

    let server = thread::spawn(move || {
        let (mut conn, _) = listener.accept()
            .expect("unknown_kind: accept failed");
        codec::read_connect_request(&mut conn)
    });

    let mut client = TcpStream::connect(addr)
        .expect("unknown_kind: failed to connect");

    // Send 0xFF — not a valid SessionKind byte.
    client.write_all(&[0xFF])
        .expect("unknown_kind: write failed");
    drop(client);

    let result = server.join().expect("unknown_kind: server thread panicked");
    assert!(result.is_err(), "expected Err for unrecognised session kind 0xFF");
}
