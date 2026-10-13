# Project Rules — relay

## Code Style

### File Header
Every source file must begin with a comment block describing:
- What the file is responsible for
- Which crate it belongs to
- Any important notes

```rust
// crate: server
// file:  connection.rs
//
// Handles a single accepted TCP connection.
// Reads the ConnectRequest header from the client,
// opens the upstream connection, then splices both streams.
```

### Function Comments
Every function must have a comment directly above it explaining:
- What it does (one line)
- Key logic / steps if non-trivial

```rust
// Reads exactly n bytes from the stream into buf.
// Returns Err if the stream closes before n bytes are read.
fn read_exact(stream: &mut TcpStream, buf: &mut [u8]) -> io::Result<()> {
```

## Architecture Rules

### Procedural Style
- No traits, no generics, no OOP patterns unless absolutely necessary
- Logic lives in plain `fn` functions
- Structs are data-only (no `impl` blocks with business logic)
- `impl` blocks allowed only for `new()` constructors and `Default`

### File Size & Responsibility
- Every file has ONE job — if it does two things, split it
- No file should exceed ~80 lines of code (excluding comments)
- Prefer more files over longer files

### Logic Isolation
- Each function does ONE thing
- No function longer than ~20 lines
- Side effects (I/O, printing) only at the top level — helper functions stay pure where possible

### Module Structure
- One module = one concept
- Expose only what other crates need via `pub`
- Keep internal helpers private (`fn`, not `pub fn`)

## Naming

- Files: `snake_case.rs`
- Functions: `snake_case`
- Structs: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- No abbreviations unless extremely common (`buf`, `len`, `io`, `addr`)

## Error Handling
- Use `expect("descriptive message")` during early development
- Every `expect` message must say WHAT failed and WHERE
- No silent `unwrap()` — always prefer `expect` with context

## Dependencies
- Prefer `std` over external crates
- Add external crates only when the std solution is unreasonably verbose
- Document why a crate was added in `Cargo.toml` as a comment
