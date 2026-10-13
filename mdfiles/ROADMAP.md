# Roadmap & Next Steps

This document tracks what's been done, what's in progress, and what's planned.

---

## ✅ Done

- [x] Rust workspace setup with `resolver = "3"` (edition 2024)
- [x] `server` crate: TCP listener → thread-per-connection model
- [x] `connection` module: bidirectional forwarding with two threads
- [x] `forwarder` module: raw byte pipe with 4 KB buffer
- [x] `config` module: static `Config` struct
- [x] `client` crate: minimal test client with raw HTTP request
- [x] `protocol` crate: scaffolded with stub files

---

## 🚧 In Progress / Immediate Next Steps

### 1. Fill in `protocol` crate
- Define `message.rs`: message types for a custom relay protocol
  - `ConnectRequest { destination: String }`
  - `ConnectResponse { success: bool }`
  - `DataFrame { payload: Vec<u8> }`
- Define `codec.rs`: length-prefixed binary codec (e.g. `u32` BE length header + payload)

### 2. Wire `protocol` into `server` and `client`
- Add `protocol` as a dependency in `server/Cargo.toml` and `client/Cargo.toml`
- Use `ConnectRequest` to let clients tell the server *where* to connect (dynamic destination)

### 3. Dynamic Configuration
- Add CLI argument parsing with `clap` (or read from env vars)
  ```toml
  [dependencies]
  clap = { version = "4", features = ["derive"] }
  ```
- Allow `--listen` and `--destination` flags:
  ```bash
  cargo run -p server -- --listen 0.0.0.0:9000 --destination example.com:80
  ```

---

## 🔮 Future / Planned

### Async with Tokio
- Replace `std::thread` + `std::net` with `tokio::net::TcpStream` and `tokio::io::copy`
- Eliminates the 2-thread-per-connection overhead; handles thousands of concurrent connections
- Key change in `forwarder.rs`:
  ```rust
  // Before (sync)
  std::io::copy(&mut source, &mut destination);

  // After (async)
  tokio::io::copy(&mut source, &mut destination).await;
  ```

### Graceful Shutdown
- Replace `expect` panics with proper error propagation (`anyhow` or `thiserror`)
- Handle `SIGINT`/`SIGTERM` to close listener and drain active connections

### TLS Support
- Use `rustls` or `native-tls` to support HTTPS/TLS destinations
- Optionally wrap the listening side too (TLS termination)

### Connection Multiplexing
- Allow multiple logical streams over a single relay connection (like SOCKS5 or HTTP CONNECT)
- Requires the `protocol` crate's codec to be implemented first

### Observability
- Add structured logging with `tracing`
- Expose metrics (connections/sec, bytes forwarded, active connections) via a `/metrics` HTTP endpoint

---

## Known Issues / Limitations

| Issue | Impact | Fix |
|---|---|---|
| `expect` panics | Server crashes on any error | Replace with error propagation |
| Hardcoded config | Can't change destination without recompile | CLI args / env vars |
| Sync threads | Poor scalability (2 threads per connection) | Migrate to async tokio |
| No auth | Anyone can use the relay | Add connection auth to protocol |
| Buffer size fixed at 4 KB | Possible inefficiency for large transfers | Tune or make configurable |
