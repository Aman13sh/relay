# Codebase Reference

A file-by-file breakdown of every source file in the project.

---

## Workspace Root

### [`Cargo.toml`](../Cargo.toml)

```toml
[workspace]
members = ["crates/client", "crates/server", "crates/protocol"]
resolver = "3"
```

- **resolver = "3"** — uses Cargo's latest feature resolver (Rust 2024 edition default).
- Three workspace members: `client`, `server`, `protocol`.
- No shared `[workspace.dependencies]` yet.

---

## `crates/server`

The relay server binary.

### [`main.rs`](../crates/server/src/main.rs)

```rust
fn main() {
    server::run();
}
```

Pure entry point. Declares all modules (`config`, `connection`, `forwarder`, `server`) and delegates to `server::run()`.

---

### [`config.rs`](../crates/server/src/config.rs)

```rust
pub struct Config {
    pub listen_addr: String,
    pub destination_addr: String,
}
impl Config {
    pub fn new() -> Self { ... }
}
```

- Holds two addresses: where to listen, and where to forward.
- `Config::new()` returns hardcoded defaults.
- **TODO:** Replace with CLI arg parsing or env var reading.

---

### [`server.rs`](../crates/server/src/server.rs)

```rust
pub fn run() {
    let config = Config::new();
    let listener = TcpListener::bind(&config.listen_addr)?;
    for stream in listener.incoming() {
        thread::spawn(move || connection::handle(client, config));
    }
}
```

- Binds the listener.
- Loops over incoming connections.
- Spawns one OS thread per connection.
- Passes `config` into each connection handler (via move closure).

> **Note:** `config` is referenced as `&config` inside the loop but moved into the `thread::spawn` closure — this works because `Config` is `Copy`-able via the reference, but watch for lifetime issues if Config becomes more complex.

---

### [`connection.rs`](../crates/server/src/connection.rs)

```rust
pub fn handle(client: TcpStream, config: &Config) {
    let destination = TcpStream::connect(&config.destination_addr)?;

    let client_to_dst = client.try_clone()?;
    let dst_to_client = destination.try_clone()?;

    let t1 = thread::spawn(|| forwarder::forward(client_to_dst, destination));
    let t2 = thread::spawn(|| forwarder::forward(dst_to_client, client));

    t1.join()?;
    t2.join()?;
}
```

- Connects to the destination server the moment a client arrives.
- Clones streams to enable bidirectional forwarding on separate threads.
- `try_clone()` creates a second file descriptor pointing to the same socket — reads/writes on clones share the same underlying OS socket.
- Joins both threads before returning (blocks until the full connection session is done).

---

### [`forwarder.rs`](../crates/server/src/forwarder.rs)

```rust
pub fn forward(mut source: TcpStream, mut destination: TcpStream) {
    let mut buffer = [0u8; 4096];
    loop {
        match source.read(&mut buffer) {
            Ok(0)          => break,           // EOF
            Ok(n)          => destination.write_all(&buffer[..n]),
            Err(e)         => { eprintln!(...); break }
        }
    }
}
```

- The core forwarding primitive.
- Reads up to 4096 bytes at a time.
- Writes all read bytes to the destination.
- Breaks on EOF (`Ok(0)`) or any read error.

---

## `crates/client`

A minimal test client binary.

### [`main.rs`](../crates/client/src/main.rs)

```rust
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n")?;
    stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..n]));
}
```

- Connects directly to the relay at `127.0.0.1:9000`.
- Sends a raw HTTP/1.1 GET request (no library — raw bytes).
- Reads up to 1024 bytes of the response and prints it.
- Useful to manually verify the relay is working.

### Stub files (empty / not yet implemented)
| File | Intended Purpose |
|---|---|
| `client.rs` | Reusable client struct |
| `config.rs` | Client-side config (target relay addr, etc.) |
| `connection.rs` | Connection lifecycle management |

---

## `crates/protocol`

Shared library crate — **not yet used**.

### [`lib.rs`](../crates/protocol/src/lib.rs)

Contains only the default Cargo placeholder `add(left, right)` function. Will be replaced with actual protocol types.

### Stub files (empty / not yet implemented)
| File | Intended Purpose |
|---|---|
| `message.rs` | Message types: `Connect`, `Data`, `Disconnect`, etc. |
| `codec.rs` | Serialization: encode/decode messages to/from bytes |
