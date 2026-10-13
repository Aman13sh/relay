// crate: protocol
// file:  message.rs
//
// Defines the messages exchanged between relay client and server.
// These are pure data structs — no I/O lives here.
// Encoding and decoding is handled in codec.rs.

use crate::types::SessionKind;

// Sent by the client immediately after connecting to the relay.
// Tells the server which service to connect to on behalf of the client.
//
// Wire layout (written by codec::write_connect_request):
//   [1 byte]  session_kind
//   [4 bytes] destination length (u32, big-endian)
//   [N bytes] destination string (UTF-8)
pub struct ConnectRequest {
    // What kind of session this is (e.g. Git).
    pub kind: SessionKind,

    // Upstream address the relay should connect to.
    // Format: "host:port", e.g. "github.com:22"
    pub destination: String,
}

impl ConnectRequest {
    // Creates a new ConnectRequest with the given kind and destination.
    pub fn new(kind: SessionKind, destination: &str) -> ConnectRequest {
        ConnectRequest {
            kind,
            destination: destination.to_string(),
        }
    }
}
