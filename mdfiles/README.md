# Relay — Transparent TCP Tunnel & Remote Execution Middleman

A lightweight TCP relay server and client that lets a **local machine** interact with remote services (like Git repositories or Docker daemons) **using credentials and network access of a host machine**, without transferring secret keys or code to the host.

---

## 1. Zero-Rust Setup (Ship as Package)

You do **NOT** need Rust or Cargo installed on either machine to run this.

### Build the Package Once (On Build Machine)
```bash
make package
```
This generates a self-contained tarball: `relay-0.1.0-<os>-<arch>.tar.gz` containing only the compiled binaries and `install.sh`.

### Install on Any Machine (No Rust Required)
```bash
tar -xzf relay-0.1.0-*.tar.gz
cd relay-0.1.0
./install.sh
```
Installs:
- `relay-server` → `/usr/local/bin` (or `~/.local/bin`)
- `relay-client` → `/usr/local/bin` (or `~/.local/bin`)

---

## 2. Two-Machine Setup (Network Test)

### Machine A: Host Machine (Has SSH Keys / Privileges)
1. Run the relay server (it binds to `0.0.0.0:9000` by default):
   ```bash
   relay-server
   ```
2. Find Machine A's local IP address:
   ```bash
   # On macOS:
   ipconfig getifaddr en0
   # On Linux:
   hostname -I | awk '{print $1}'
   ```
   *Example: `192.168.1.50`*

### Machine B: Local Machine (Where You Work)
Point `RELAY_ADDR` to Machine A's IP address:

```bash
export RELAY_ADDR="192.168.1.50:9000"
```

#### Test SSH Connection:
```bash
ssh -o "ProxyCommand=relay-client" -T git@github.com
```

#### Test Git Clone / Push via Relay:
```bash
GIT_SSH_COMMAND="ssh -o ProxyCommand=relay-client" git clone git@github.com:your-user/your-repo.git
```

---

## 3. Configuration via Environment Variables

No code editing or recompilation needed:

| Variable | Target | Default | Description |
|---|---|---|---|
| `RELAY_LISTEN` | `relay-server` | `0.0.0.0:9000` | Address to bind to on host machine |
| `RELAY_ADDR` | `relay-client` | `127.0.0.1:9000` | Relay server IP/port to connect to |
| `RELAY_DEST` | `relay-client` | `github.com:22` | Upstream destination requested |

---

## 4. Architecture

```
[Local Machine B]                              [Host Machine A]                             [Remote]
 (Your workspace)                               (Has SSH credentials)
┌─────────────────┐   Protocol Handshake       ┌─────────────────┐    Raw SSH Stream       ┌────────────┐
│  git / ssh      │ ─────────────────────────► │  relay-server   │ ──────────────────────► │ GitHub:22  │
│  (via client)   │ ◄───────────────────────── │  (0.0.0.0:9000) │ ◄────────────────────── │            │
└─────────────────┘      TCP Port 9000         └─────────────────┘                         └────────────┘
```
