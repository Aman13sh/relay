// crate: protocol
// file:  types.rs
//
// Shared primitive types used across message and codec.
// SessionKind tells the relay server what kind of session
// the client wants to open (git, docker, exec, etc.).

// Identifies which upstream service this session targets.
// Encoded as a single byte on the wire.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionKind {
    // Forward raw TCP to a git remote (e.g. github.com:22)
    Git = 0x01,
}

impl SessionKind {
    // Converts a raw byte from the wire into a SessionKind.
    // Returns None if the byte is not a recognised session type.
    pub fn from_byte(byte: u8) -> Option<SessionKind> {
        match byte {
            0x01 => Some(SessionKind::Git),
            _    => None,
        }
    }

    // Converts a SessionKind into its wire byte representation.
    pub fn as_byte(self) -> u8 {
        self as u8
    }
}
