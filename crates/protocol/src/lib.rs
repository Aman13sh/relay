// crate: protocol
// file:  lib.rs
//
// Public interface of the protocol crate.
// Re-exports message types and codec functions
// so callers only need `use protocol::...`.

pub mod codec;
pub mod message;
pub mod types;
