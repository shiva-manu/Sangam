# Sangam

> **AirDrop for Compute** — Turn nearby machines into a collaborative compute mesh.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](Cargo.toml)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB.svg)](apps/desktop/src-tauri/Cargo.toml)
[![React](https://img.shields.io/badge/React-19-61DAFB.svg)](apps/desktop/package.json)
[![Status](https://img.shields.io/badge/status-early--development-yellow.svg)]()

Sangam is a **local-first, peer-to-peer compute mesh** that transforms nearby machines on the same network into a temporary collaborative compute cluster. Built with Rust and Tauri 2, it enables teams, researchers, and developers to pool idle CPU, memory, and network capacity — without provisioning cloud infrastructure for every experiment.

---

## Table of Contents

- [Key Features](#-key-features)
- [Why Sangam Exists](#-why-sangam-exists)
- [Architecture](#-architecture)
- [Quick Start](#-quick-start)
- [Development Commands](#-development-commands)
- [Working With Multiple Nodes](#-working-with-multiple-nodes)
- [Repository Layout](#-repository-layout)
- [Technology Stack](#-technology-stack)
- [Current Status](#-current-status)
- [Roadmap](#-roadmap)
- [Use Cases](#-use-cases)
- [Contributing](#-contributing)
- [Security](#-security)
- [License](#-license)

---

## Key Features

| Feature | Description |
|---------|-------------|
| **LAN Peer Discovery** | Automatic node discovery via mDNS (`_sangam._udp.local.`) |
| **Desktop Control Plane** | Beautiful React + Tauri 2 UI with real-time metrics, logs, and task tracking |
| **Lightweight Runtime** | Rust core with Tokio async runtime, minimal overhead |
| **Transparent Operations** | Visualize peers, resources, task state, and runtime logs in real time |
| **Security-First Design** | Built with strong isolation and explicit trust boundaries in mind |
| **Multiple Task Types** | Sum, average, and shell command execution across the mesh |

---

## Why Sangam Exists

Modern laptops often sit near each other with unused CPU, memory, and network capacity. Sangam explores a lightweight way for teams, students, researchers, and hackathon builders to pool that local capacity without provisioning cloud infrastructure for every experiment.

The project is built around a few principles:

- **Local-first** — Nearby machines should be useful even without cloud services.
- **Lightweight** — The first successful cluster should not require Kubernetes.
- **Transparent** — Users should see peers, resources, logs, and task state.
- **Secure by design** — Arbitrary workloads must eventually run with strong isolation, explicit permissions, and clear trust boundaries.
- **Developer-friendly** — Rust core logic, a desktop control plane, and readable APIs should make the system easy to extend.

---

## Architecture

```text
                    Tauri Desktop App
           React UI + charts + runtime controls
                            |
                            v
                    Tauri command layer
                            |
                            v
                      sangam-core
  discovery | networking | peers | tasks | logs | metrics
                            |
                            v
           Nearby Sangam nodes on the local network
```

The runtime flow:

1. A node starts a TCP server on `0.0.0.0:8080`.
2. The node advertises itself through mDNS.
3. Other nodes resolve the service and add it to the peer registry.
4. The runtime sends a task to a discovered peer.
5. The receiving node executes the task and returns a result.
6. The task tracker, log bus, and desktop UI reflect the lifecycle.

---

## Quick Start

### Prerequisites

**Core runtime:**
- Rust stable toolchain (2024 edition)

**Desktop app:**
- Node.js and npm
- Rust stable toolchain
- Tauri system prerequisites for your OS

On Ubuntu/Debian:

```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

For other OSes, see the [official Tauri prerequisites](https://tauri.app/start/prerequisites/).

### Clone and Run

```bash
git clone https://github.com/shiva-manu/Sangam.git
cd Sangam
```

Run the headless runtime:

```bash
cargo run -p sangam-core
```

Run the desktop app:

```bash
npm run desktop:dev
```

Or manually:

```bash
cd apps/desktop
npm install
npm run tauri dev
```

---

## Development Commands

### Core Runtime

```bash
cargo fmt --all --check                                          # formatting
cargo clippy -p sangam-core --all-targets -- -D warnings         # lint
cargo test -p sangam-core                                        # tests
```

### Full Workspace (including Tauri crate)

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Desktop App

```bash
cd apps/desktop
npm install
npm run build                                                    # type-check + build
```

---

## Working With Multiple Nodes

To see discovery in action, run Sangam on two machines connected to the same local network. Each node should:

- Be able to send and receive local network traffic.
- Allow inbound TCP traffic on port `8080`.
- Allow mDNS/Bonjour traffic on the local network.

When the nodes discover each other, the runtime prints peer information and the desktop UI updates its peer, task, log, and metrics views.

---

## Repository Layout

```text
.
|-- apps/
|   `-- desktop/              # Tauri 2 + React + TypeScript desktop app
|       |-- src/              # Frontend UI (components, pages, hooks, lib)
|       `-- src-tauri/        # Tauri command layer and desktop shell
|-- crates/
|   `-- core/                 # Sangam runtime
|       |-- src/
|       |   |-- discovery/    # mDNS peer discovery
|       |   |-- networking/   # TCP client and server
|       |   |-- peers/        # Peer registry and status tracking
|       |   |-- tasks/        # Task definitions, execution, tracking
|       |   |-- metrics/      # System metrics collection and storage
|       |   |-- logging/      # Broadcast-based log bus
|       |   |-- models/       # Wire protocol message types
|       |   `-- utils/        # Banner, system info helpers
|       `-- tests/            # Integration tests
|-- Cargo.toml                # Rust workspace root
|-- package.json              # Convenience scripts for the desktop package
|-- CONTRIBUTING.md
|-- CODE_OF_CONDUCT.md
|-- SECURITY.md
|-- LICENSE
`-- README.md
```

---

## Technology Stack

### Core Runtime

| Crate | Purpose |
|-------|---------|
| Rust 2024 | Language and edition |
| Tokio | Async runtime |
| `mdns-sd` | mDNS peer discovery |
| `serde` / `serde_json` | Serialization |
| `sysinfo` | System metrics |
| `local-ip-address` | Local IP detection |

### Desktop App

| Dependency | Purpose |
|------------|---------|
| Tauri 2 | Desktop shell and IPC |
| React 19 | UI framework |
| TypeScript | Type-safe frontend |
| Vite | Build tooling |
| Tailwind CSS | Utility-first styling |
| Radix UI | Accessible primitives |
| Recharts | Charts and graphs |
| Lucide | Icons |

---

## Current Status

**Implemented:**

- LAN peer discovery with mDNS using the `_sangam._udp.local.` service type.
- Node advertisements that include node identity, CPU thread count, and RAM.
- Async TCP messaging on port `8080` using newline-delimited JSON.
- Task lifecycle for sum, average, and shell command tasks.
- In-memory peer registry, task tracker, runtime log bus, and metrics store.
- Tauri 2 desktop shell with React, TypeScript, Tailwind CSS, charts, runtime controls, peer views, task views, logs, and local metrics.

**Not yet production-ready:**

- Encrypted QUIC transport and stronger peer identity.
- Trust and pairing flows for private clusters.
- Container, WebAssembly, or microVM runtime isolation.
- Resource-aware scheduling, retries, result aggregation, and DAG execution.
- Packaged desktop releases and a stable developer-facing workload API.

---

## Roadmap

### Near Term

- Stabilize peer discovery and membership state.
- Add richer task types and examples.
- Improve runtime error reporting and desktop feedback.
- Expand integration tests around networking, discovery, and task tracking.
- Package desktop builds for supported platforms.

### Mid Term

- Replace prototype TCP transport with authenticated encrypted transport.
- Add trust, pairing, and cluster membership flows.
- Introduce a real scheduler with retries, placement decisions, and result aggregation.
- Add sandboxed workload execution with resource limits.
- Provide CLI and SDK surfaces for submitting workloads.

### Long Term

- Support DAG-based distributed workloads.
- Add GPU-aware scheduling where available.
- Support persistent team clusters.
- Explore relay, NAT traversal, and broader edge-compute use cases.

---

## Use Cases

Sangam is being designed for workloads that can be split into independent or mostly independent tasks:

- Batch image or video processing.
- Local AI preprocessing, embedding generation, or inference batches.
- Distributed test runs and build steps.
- Research data processing.
- Hackathon compute pooling across team laptops.

---

## Contributing

Contributions are welcome, especially in Rust networking, distributed systems, desktop UX, testing, documentation, and security design.

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request and follow the [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## Security

Sangam is not ready to run untrusted arbitrary workloads. Until strong isolation, trust management, and encrypted transport are implemented, only use the prototype on networks and machines you control.

Please do not report security vulnerabilities through public issues. Use GitHub private security advisories when available, or contact the repository maintainers through the repository owner profile. See [SECURITY.md](SECURITY.md) for details.

---

## License

Sangam is released under the MIT License. See [LICENSE](LICENSE) for details.
