# Architecture Overview

## Vision

Build a **remote execution relay** that lets your local machine **borrow the identity and environment of a remote host machine** — without copying code to the remote.

The core idea:

> *"My code lives on my laptop. But when I do `git push`, it looks like it came from the server — using the server's SSH keys, the server's git identity, the server's IP address."*

This is **not** a simple TCP pipe. It is an **identity relay** — the host machine acts on behalf of the local machine, using its own credentials, keys, and environment.

---

## The Problem Being Solved

| Situation | Without Relay | With Relay |
|---|---|---|
| `git push` to GitHub | Must have SSH key on local, or copy key | Host's SSH key is used, zero config on local |
| Build a Docker image | Must have Docker installed locally | Host builds it, host's daemon is used |
| Run a container | Needs local Docker/k8s | Container runs on host, serves from host IP |
| Deploy | Must scp/rsync code to host first | Code never leaves local — host executes directly |

---

## System Overview

```
┌──────────────────────────────────────────────────────────────────────┐
│                         LOCAL MACHINE                                │
│                                                                      │
│   Developer writes code                                              │
│   runs: git push → points to Relay (e.g., 127.0.0.1:9000)           │
│                                                                      │
│   ┌─────────────────────────────┐                                    │
│   │   Client (relay client)     │                                    │
│   │   - Sends git protocol      │                                    │
│   │   - Sends commands          │                                    │
│   └──────────────┬──────────────┘                                    │
└──────────────────┼───────────────────────────────────────────────────┘
                   │  TCP connection (persistent tunnel)
                   │  raw bytes / framed protocol
                   ▼
┌──────────────────────────────────────────────────────────────────────┐
│                         HOST / REMOTE MACHINE                        │
│                                                                      │
│   ┌─────────────────────────────────────────────────────────┐        │
│   │                  Relay Server                           │        │
│   │                                                         │        │
│   │  Receives: git pack protocol / commands from local      │        │
│   │  Executes: using HOST's own credentials & environment   │        │
│   │                                                         │        │
│   │  ┌───────────────┐    ┌───────────────┐                 │        │
│   │  │ Git Forwarder │    │ Docker Runner │  ← future       │        │
│   │  │ (Milestone 1) │    │ (Milestone 2) │                 │        │
│   │  └───────┬───────┘    └───────────────┘                 │        │
│   └──────────┼────────────────────────────────────────────--┘        │
│              │                                                        │
│    uses HOST's SSH keys (~/.ssh/id_rsa)                              │
│    uses HOST's git config (~/.gitconfig)                             │
│    uses HOST's docker daemon (/var/run/docker.sock)                  │
└──────────────┼───────────────────────────────────────────────────────┘
               │  outbound connection from HOST (using HOST's identity)
               ▼
┌──────────────────────────────────────────────────────────────────────┐
│                     REMOTE SERVICES                                  │
│                                                                      │
│   GitHub (git@github.com)    — sees push from HOST's SSH key         │
│   Docker Hub                 — sees push from HOST's account         │
│   Any HTTP/SSH endpoint      — sees requests from HOST's IP          │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Milestone 1 — Git Relay

**Goal:** Local developer does `git push` → GitHub receives it as if pushed from the host machine.

### How it works

```
Local: git push relay-origin main
        │
        │  (git smart HTTP / SSH pack protocol over TCP)
        ▼
Relay Server (on host, port 9000)
        │
        │  receives git pack data from local
        │  opens connection to git@github.com using HOST's SSH key
        │  forwards pack protocol bytes to GitHub
        │
        ▼
GitHub ← receives the push authenticated as the HOST machine
```

**Key properties:**
- Code bytes travel: `local → relay → github` (relay is a passthrough, not a store)
- Authentication: **host machine's `~/.ssh/id_rsa`** is used, never the local key
- Local git config: just needs `git remote add origin tcp://host-ip:9000/repo-name`
- No code is persisted on the host — it's a live byte stream

### What the relay does for git

1. Accept TCP connection from local `git` client
2. Parse the target repo from the initial handshake (custom protocol header)
3. Open SSH/HTTPS connection to `github.com` using host credentials
4. Splice the two streams: local ↔ relay ↔ github (bidirectional byte forwarding)
5. When git session ends, close both sides

---

## Milestone 2 — Docker Relay *(future)*

**Goal:** Build and run Docker containers on the host without `scp`-ing code.

```
Local: relay-docker build .
        │
        │  streams Dockerfile + build context (tar) over TCP to relay
        ▼
Relay Server (on host)
        │
        │  receives build context
        │  calls host's Docker daemon (unix socket /var/run/docker.sock)
        │  streams build logs back to local terminal
        │
        ▼
Host Docker: image built using host's daemon, host's layer cache, host's registry auth
        │
        │  relay-docker run <image>
        ▼
Container runs on host, exposed on host's network
```

---

## Milestone 3 — General Remote Terminal *(future)*

**Goal:** Execute arbitrary shell commands on host from local, with output streamed back.

```
Local: relay exec -- git status
Local: relay exec -- npm run build
Local: relay exec -- systemctl restart nginx
        │
        ▼
Relay Server: runs command in host shell, streams stdout/stderr back in real time
```

This is essentially a **controlled remote shell** — not a full PTY/SSH replacement, but command-oriented execution with structured I/O.

---

## Protocol Design (protocol crate)

The `protocol` crate defines the **framing layer** between local client and relay server. Since the relay handles multiple use cases (git, docker, exec), it needs a header to distinguish session types.

### Message Types

```
┌────────────────────────────────────────────────────┐
│  Frame Header (fixed size, e.g., 8 bytes)          │
│  ┌──────────┬──────────┬────────────────────────┐  │
│  │  Version │  MsgType │  Payload Length (u32)  │  │
│  │  (u8)    │  (u8)    │                        │  │
│  └──────────┴──────────┴────────────────────────┘  │
│  Payload (variable, up to Payload Length bytes)    │
└────────────────────────────────────────────────────┘
```

**Message types (planned):**

| Type | ID | Direction | Purpose |
|---|---|---|---|
| `Hello` | 0x01 | Client→Server | Identify session type (git/docker/exec) |
| `ConnectTarget` | 0x02 | Client→Server | Tell relay where to connect (GitHub URL, etc.) |
| `Data` | 0x03 | Both | Raw payload forwarding |
| `ExecRequest` | 0x04 | Client→Server | Run a command on host |
| `ExecOutput` | 0x05 | Server→Client | stdout/stderr stream |
| `Close` | 0xFF | Both | Gracefully close session |

---

## Internal Module Architecture (server crate)

```
server/
├── main.rs          — entry point
├── config.rs        — listen addr, feature flags, credential paths
├── server.rs        — TCP listener + connection dispatch loop
├── connection.rs    — session lifecycle: parse Hello → route → forward
├── forwarder.rs     — raw bidirectional byte splicing (already working)
└── (future)
    ├── git.rs       — git session handler (connect to github with host keys)
    ├── docker.rs    — docker session handler (talk to /var/run/docker.sock)
    └── exec.rs      — exec session handler (spawn subprocess, stream output)
```

---

## Threading Model

Currently: **2 OS threads per connection** (one per direction of the TCP pipe).

```
Main thread
└── TcpListener::incoming() loop
    └── per client: thread::spawn
        └── connection::handle()
            ├── thread_1: local → upstream   (forwarder)
            └── thread_2: upstream → local   (forwarder)
```

**Future:** Migrate to **tokio async** for scalability — `tokio::io::copy_bidirectional` replaces both threads with a single async task.

---

## Key Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Transport | Raw TCP | Simple, works for any protocol (git, docker, ssh) |
| Framing | Custom length-prefixed binary | Low overhead, easy to extend |
| Auth (local→relay) | TBD (token / mTLS) | Relay must not be open to anyone |
| Auth (relay→github) | Host's `~/.ssh` keys | Core value prop — host identity is used |
| Concurrency | OS threads → tokio | Start simple, migrate when needed |
| Code storage on host | None | Code is streamed, never written to host disk |
