// crate: server
// file:  connection.rs
//
// Handles a single accepted client TCP connection.
// Reads the ConnectRequest header from the client to learn
// the upstream destination, then splices the two streams
// bidirectionally until either side closes.

use std::net::TcpStream;
use std::thread;

use protocol::codec;

use crate::forwarder;

// Entry point for a new client connection.
// Reads the protocol handshake, opens the upstream, then forwards.
pub fn handle(mut client: TcpStream, _config: &crate::config::Config) {
    let destination_addr = read_handshake(&mut client);
    let upstream         = open_upstream(&destination_addr);

    println!("Relaying: client → {destination_addr}");

    splice(client, upstream);

    println!("Connection closed: {destination_addr}");
}

// Reads the ConnectRequest from the client and returns the destination address.
// Panics with a descriptive message if the handshake fails or is malformed.
fn read_handshake(stream: &mut TcpStream) -> String {
    let request = codec::read_connect_request(stream)
        .expect("connection.rs: failed to read ConnectRequest from client");

    request.destination
}

// Opens a TCP connection to the upstream destination.
// Panics if the upstream is unreachable.
fn open_upstream(addr: &str) -> TcpStream {
    TcpStream::connect(addr)
        .expect(&format!("connection.rs: failed to connect to upstream '{addr}'"))
}

// Clones both streams and spawns two threads — one per direction.
// Blocks until both directions have finished (i.e. connection is fully closed).
fn splice(client: TcpStream, upstream: TcpStream) {
    let client_reader   = client.try_clone().expect("connection.rs: failed to clone client stream");
    let upstream_reader = upstream.try_clone().expect("connection.rs: failed to clone upstream stream");

    let t1 = thread::spawn(move || forwarder::forward(client_reader, upstream));
    let t2 = thread::spawn(move || forwarder::forward(upstream_reader, client));

    t1.join().expect("connection.rs: client→upstream thread panicked");
    t2.join().expect("connection.rs: upstream→client thread panicked");
}