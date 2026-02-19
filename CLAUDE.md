# flock-agent (flockd)

Rust on-device agent for the Flock fleet management platform. Single static binary, no runtime dependencies. Compiled with musl libc.

## Build & Run

```bash
cargo build --release
./target/release/flockd
```

## Test

```bash
cargo test
```

## Cross-compile Targets

- `aarch64-unknown-linux-musl`
- `armv7-unknown-linux-musleabihf`
- `arm-unknown-linux-musleabihf` (armv6)
- `x86_64-unknown-linux-musl`

## Architecture

Runs as a systemd service. No Node.js, no Python, no container runtime of its own.

### Responsibilities

- MQTT connection to the ingester
- Reconcile actual container state toward desired state (every 5s)
- Ship telemetry and logs
- Manage local WireGuard interface
- Self-update

### Container Runtime Abstraction

Trait-based interface with implementations for:
- **containerd** (gRPC)
- **Podman** (REST)

Configured per device. No Docker daemon required.

### Offline Resilience

Backend unreachable → device keeps running last known desired state. State reports buffer locally and flush on reconnect.

### Registration

First boot: reads provisioning key from config → calls API → receives credentials + WireGuard config → persists locally. Subsequent boots skip registration.

## Conventions

- Tests are part of every task, not a follow-up
- No stubs or scaffolding unless explicitly asked
- Prefer explicit and readable over clever
- Work in small, verifiable increments
