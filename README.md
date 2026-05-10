<div align="center">

<h1>🌊 Sangam</h1>

<p><i>संगम — the confluence of systems into one collaborative flow.</i></p>

<p><b>A wireless peer-to-peer distributed compute mesh.</b><br/>
Turn nearby laptops into a secure compute cluster over WiFi. Pool unused power. Run workloads in parallel.</p>

<p>
  <a href="#-license"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-FFD43B.svg?style=for-the-badge"></a>
  <a href="#-status"><img alt="Status" src="https://img.shields.io/badge/status-early%20concept-FF6B6B?style=for-the-badge"></a>
  <a href="#-technology-stack"><img alt="Built with Rust" src="https://img.shields.io/badge/built%20with-Rust-DEA584?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a href="#-contributing"><img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-4CAF50?style=for-the-badge"></a>
</p>

<p>
  <a href="#-vision"><b>Vision</b></a> •
  <a href="#-key-features"><b>Features</b></a> •
  <a href="#-system-architecture"><b>Architecture</b></a> •
  <a href="#-how-sangam-works"><b>How it Works</b></a> •
  <a href="#-roadmap"><b>Roadmap</b></a> •
  <a href="#-contributing"><b>Contribute</b></a>
</p>

</div>

---

> **Think AirDrop, but for compute.**
> A local distributed cloud. Wireless compute pooling. Collaborative infrastructure for AI and development.

Sangam transforms nearby laptops and computers into a secure wireless compute cluster over WiFi, allowing teams to pool unused computational power and execute workloads in parallel — without cloud bills, without Kubernetes, without dedicated infrastructure.

---

## 📑 Table of Contents

<details>
<summary>Click to expand</summary>

- [🎯 Vision](#-vision)
- [💡 Why Sangam Exists](#-why-sangam-exists)
- [🧩 Core Concept](#-core-concept)
- [✨ Key Features](#-key-features)
- [🏗️ System Architecture](#-system-architecture)
- [🛠️ Technology Stack](#-technology-stack)
- [📁 Project Structure](#-project-structure)
- [⚙️ How Sangam Works](#-how-sangam-works)
- [🔀 Distributed Execution Model](#-distributed-execution-model)
- [🔐 Security Architecture](#-security-architecture)
- [🧠 Scheduler Design](#-scheduler-design)
- [🌐 Networking Layer](#-networking-layer)
- [📦 Runtime Isolation](#-runtime-isolation)
- [🎨 Example Use Cases](#-example-use-cases)
- [🗺️ Roadmap](#-roadmap)
- [🌟 Future Goals](#-future-goals)
- [🚀 Installation](#-installation)
- [🤝 Contributing](#-contributing)
- [📜 License](#-license)
- [📍 Status](#-status)

</details>

---

## 🎯 Vision

Modern laptops and personal computers contain **enormous unused compute power**. Every day:

- 💤 CPUs sit idle while users browse the web
- 🎮 GPUs remain underutilized between gaming sessions
- 💸 Teams lack affordable compute access
- 🎓 Students cannot always afford cloud infrastructure
- ⏱️ Hackathon projects hit hardware limitations at the worst moment

> Sangam creates a **local-first compute layer** where nearby systems collaborate dynamically to share workloads securely and efficiently.

The long-term vision is a decentralized compute fabric where students, developers, researchers, and teams can turn nearby devices into **temporary compute clusters** — without needing dedicated infrastructure.

---

## 💡 Why Sangam Exists

Distributed computing today is broken for everyday developers:

| ❌ Today's Reality | ✅ Sangam's Approach |
| :--- | :--- |
| Cloud-dependent | Local-first |
| Infrastructure heavy | Lightweight |
| Overly complex | Developer-friendly |
| Expensive to operate | Free between peers |
| Requires Kubernetes & servers | Wireless & ad-hoc |
| Inaccessible to students | Built for collaboration |

Most systems require Kubernetes, dedicated servers, static infrastructure, or complex orchestration before users can run even simple distributed workloads. Sangam aims to make distributed computing **wireless, local-first, lightweight, collaborative, secure, and developer-friendly.**

---

## 🧩 Core Concept

Sangam does **not** try to merge multiple laptops into one giant virtual computer. Instead, it **distributes independent tasks** across multiple worker nodes.

```text
                    📦 1000 images to process
                              │
        ┌──────────┬──────────┼──────────┬──────────┐
        ▼          ▼          ▼          ▼          ▼
   ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
   │Laptop A│ │Laptop B│ │Laptop C│ │Laptop D│
   │ 1-250  │ │251-500 │ │501-750 │ │751-1000│
   └────────┘ └────────┘ └────────┘ └────────┘
```

Each device executes its assigned tasks independently and returns results to the scheduler.

This architecture is **scalable**, **fault tolerant**, **network efficient**, **wireless-friendly**, and **practical on consumer hardware**.

---

## ✨ Key Features

<table>
<tr>
<td width="50%" valign="top">

### 📡 Wireless Peer Discovery
Nearby devices automatically discover each other over WiFi using **mDNS**, **Zeroconf**, and **peer-to-peer** protocols. No manual IP configuration required.

</td>
<td width="50%" valign="top">

### 🔄 Dynamic Compute Clusters
Create temporary compute clusters instantly. Devices can **join**, **leave**, **reconnect**, **pause contribution**, or **scale dynamically**.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### ⚡ Distributed Task Execution
Large workloads are split into smaller tasks and distributed across multiple systems for true **parallel execution**.

</td>
<td width="50%" valign="top">

### 🛡️ Secure Sandboxed Runtime
All workloads run inside **isolated containers** or **microVMs**. Contributor devices stay protected from malicious code, filesystem access, credentials, and unauthorized networking.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### 🎛️ Resource Contribution Controls
Configure **CPU limits**, **RAM limits**, **battery-aware participation**, **trusted-device permissions**, and **idle-only mode**.

</td>
<td width="50%" valign="top">

### ♻️ Fault Tolerance
If a worker disconnects, unfinished tasks are **reassigned**, **checkpoints restored**, and workloads **continue automatically**.

</td>
</tr>
<tr>
<td colspan="2" valign="top">

### 📊 Real-Time Monitoring
A local dashboard displays **active nodes**, **CPU/RAM usage**, **task progress**, **cluster health**, and **execution logs** — all in real time.

</td>
</tr>
</table>

---

## 🏗️ System Architecture

```text
                         ┌──────────────────────┐
                         │     🧠 Scheduler     │
                         │  Task Orchestrator   │
                         └──────────┬───────────┘
                                    │
               ┌────────────────────┼────────────────────┐
               │                    │                    │
               ▼                    ▼                    ▼
        ┌────────────┐      ┌────────────┐      ┌────────────┐
        │ 💻 Laptop A│      │ 💻 Laptop B│      │ 💻 Laptop C│
        │   Worker   │      │   Worker   │      │   Worker   │
        └─────┬──────┘      └─────┬──────┘      └─────┬──────┘
              │                   │                   │
              └───────────────────┼───────────────────┘
                                  │
        ┌─────────────────────────▼──────────────────────────┐
        │           🌐 QUIC + P2P Networking Layer           │
        └────────────────────────────────────────────────────┘
```

| Component | Responsibility |
| :--- | :--- |
| **Scheduler** | Task orchestration, worker coordination, retries, result aggregation, cluster health |
| **Workers** | Execute isolated workloads, stream logs/results, report resource availability |
| **Network Layer** | Encrypted P2P transport, peer discovery, NAT-aware routing |

---

## 🛠️ Technology Stack

<table>
<tr>
<td valign="top" width="50%">

#### 🦀 Core Runtime
![Rust](https://img.shields.io/badge/Rust-000?style=flat&logo=rust)
![Tokio](https://img.shields.io/badge/Tokio-async-FF6B35?style=flat)

- **Rust** — systems language for safety + speed
- **Tokio** — async runtime
- **Async Rust ecosystem**

</td>
<td valign="top" width="50%">

#### 🌐 Networking
![QUIC](https://img.shields.io/badge/QUIC-quinn-9146FF?style=flat)
![gRPC](https://img.shields.io/badge/gRPC-tonic-1AAAFF?style=flat)

- **QUIC** (`quinn`) — low-latency transport
- **gRPC** + **libp2p** — peer protocols
- **mDNS / Zeroconf** — discovery
- **TLS** — encryption everywhere

</td>
</tr>
<tr>
<td valign="top" width="50%">

#### 📦 Runtime Isolation
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white)
![Wasm](https://img.shields.io/badge/Wasm-654FF0?style=flat&logo=webassembly&logoColor=white)

- **Docker** — Phase 1 sandbox
- **Firecracker** — microVM isolation
- **WebAssembly** — secure runtime

</td>
<td valign="top" width="50%">

#### 🖥️ Desktop Interface
![Tauri](https://img.shields.io/badge/Tauri-24C8DB?style=flat&logo=tauri&logoColor=white)
![React](https://img.shields.io/badge/React-61DAFB?style=flat&logo=react&logoColor=black)
![Tailwind](https://img.shields.io/badge/Tailwind-06B6D4?style=flat&logo=tailwindcss&logoColor=white)

- **Tauri** — lightweight cross-platform shell
- **React** + **Tailwind CSS** — UI

</td>
</tr>
<tr>
<td valign="top" width="50%">

#### 💾 State & Storage
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat&logo=sqlite&logoColor=white)
![Redis](https://img.shields.io/badge/Redis-DC382D?style=flat&logo=redis&logoColor=white)
![Postgres](https://img.shields.io/badge/PostgreSQL-336791?style=flat&logo=postgresql&logoColor=white)

- **SQLite** — local node state
- **Redis** — task queues
- **PostgreSQL** — persistent clusters

</td>
<td valign="top" width="50%">

#### 🔭 Observability & Auth
![OTel](https://img.shields.io/badge/OpenTelemetry-425CC7?style=flat&logo=opentelemetry&logoColor=white)

- **`tracing`** + **OpenTelemetry**
- **Protocol Buffers** + `tonic` gRPC
- **ed25519** identity keypairs
- **Node identity certificates**

</td>
</tr>
</table>

---

## 📁 Project Structure

<details>
<summary><b>Click to view planned repository layout</b></summary>

```text
sangam/
│
├── 📱 apps/
│   ├── desktop/                # Tauri desktop application
│   └── dashboard/              # Web dashboard
│
├── 📦 crates/
│   ├── scheduler/              # Task scheduler engine
│   ├── runtime/                # Task execution runtime
│   ├── networking/             # QUIC + P2P networking
│   ├── discovery/              # mDNS peer discovery
│   ├── security/               # Authentication & encryption
│   ├── worker/                 # Worker node runtime
│   ├── orchestrator/           # Distributed task orchestration
│   ├── checkpointing/          # Fault tolerance system
│   ├── telemetry/              # Metrics & tracing
│   └── common/                 # Shared utilities and types
│
├── 🔌 proto/                   # gRPC protobuf definitions
│
├── 🐳 containers/
│   ├── python/
│   ├── node/
│   └── rust/
│
├── 🎯 examples/
│   ├── rendering/
│   ├── image-processing/
│   ├── ai-inference/
│   └── distributed-builds/
│
├── 🛠️ scripts/
│
├── 📚 docs/
│   ├── architecture/
│   ├── networking/
│   ├── scheduler/
│   └── security/
│
├── 🧪 tests/
│
├── Cargo.toml
├── README.md
└── LICENSE
```

</details>

---

## ⚙️ How Sangam Works

```text
   1️⃣ Discover  →  2️⃣ Cluster  →  3️⃣ Submit  →  4️⃣ Schedule  →  5️⃣ Execute  →  6️⃣ Aggregate
```

### 1️⃣ Device Discovery

Nearby devices broadcast availability over local WiFi using **mDNS**, **Zeroconf**, and **QUIC peer discovery**.

```text
🔍 Mani Laptop discovered
   ├─ 8 CPU threads
   ├─ 16 GB RAM
   └─ ✅ Available for contribution
```

### 2️⃣ Cluster Formation

Users create or join clusters. Resources from each device pool together.

```text
Create cluster  →  Invite team  →  Pool resources  →  Ready to execute
```

### 3️⃣ Task Submission

Users submit workloads such as **Python jobs**, **rendering tasks**, **AI workloads**, **test suites**, and **distributed builds**.

### 4️⃣ Task Scheduling

The scheduler splits workloads, assigns tasks, tracks progress, retries failures, balances load dynamically, and monitors worker health.

### 5️⃣ Execution

Worker nodes receive isolated workloads, execute them inside **containers** or **microVMs**, stream logs and results, and return outputs securely.

### 6️⃣ Result Aggregation

The scheduler merges **outputs**, **checkpoints**, **logs**, and **execution results** into the final deliverable.

---

## 🔀 Distributed Execution Model

Sangam uses a **task-graph execution model**:

```text
        ┌───────► Task B ───────┐
Task A ─┤                       ├──► Task D (aggregate)
        └───────► Task C ───────┘
```

Independent tasks execute in parallel across available workers. The model supports:

- ✅ **Scalability** — fan out across N workers
- 🔁 **Retries** — automatic failure recovery
- 🧬 **Dependency tracking** — DAG-aware scheduling
- 📍 **Checkpoint recovery** — resume from last good state
- 🧮 **Partial aggregation** — stream results as they arrive

---

## 🔐 Security Architecture

> **Security is a first-class priority.** Sangam is designed around **zero-trust execution.**

Contributor devices should **never** expose local files, OS access, private credentials, or unrestricted network permissions. All workloads execute inside isolated runtime environments.

### 🧱 Isolation Layers

| Phase | Isolation Technology | Trust Level |
| :---: | :--- | :--- |
| **1** | 🐳 Docker containers | Good |
| **2** | 🔥 Firecracker microVMs | Strong |
| **3** | 🕸️ WebAssembly runtime | Strongest |

### 🔒 Encryption

All communication is **encrypted**, **authenticated**, and **peer-verified** using:

- **TLS** for transport security
- **ed25519** identity keys
- **Node identity certificates**

---

## 🧠 Scheduler Design

The scheduler is the brain of every cluster. It handles:

<table>
<tr>
<td valign="top" width="50%">

**Today**
- 🗂️ Node management
- 🎯 Workload distribution
- 🔁 Retries
- 📍 Checkpoint recovery
- 💓 Health monitoring
- 🧬 Dependency execution
- 🧮 Result aggregation
- ⚖️ Resource-aware scheduling

</td>
<td valign="top" width="50%">

**Future**
- 🎮 GPU-aware scheduling
- 📊 Resource scoring
- 🧠 Intelligent load balancing
- 🛰️ Latency-aware routing
- 🤖 ML-driven placement
- 🌍 Cross-cluster federation

</td>
</tr>
</table>

---

## 🌐 Networking Layer

Sangam uses **QUIC-based communication** for:

- ⚡ low latency
- 🔀 multiplexed streams
- 🔒 encrypted transport
- 📶 unstable WiFi resilience
- 🔄 fast reconnects

**Future capabilities** include internet-wide mesh routing, NAT traversal, decentralized peer relay systems, and persistent team clusters.

---

## 📦 Runtime Isolation

Tasks execute inside isolated environments providing **security**, **reproducibility**, **cross-platform compatibility**, and **predictable resource limits**.

**Supported runtime targets:**

![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=node.js&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust&logoColor=white)
![Custom](https://img.shields.io/badge/Custom%20Containers-2496ED?style=for-the-badge&logo=docker&logoColor=white)

---

## 🎨 Example Use Cases

<table>
<tr>
<td width="50%" valign="top">

### 🏆 Hackathon Compute Pooling
Temporarily combine team laptops into a local compute cluster — no cloud setup, no credit cards, just plug in and pool.

</td>
<td width="50%" valign="top">

### 🤖 AI Workloads
Distribute embedding generation, preprocessing, inference batches, vector indexing, and multi-agent workflows across nearby devices.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### 🎬 Rendering Farms
Parallel rendering for **Blender**, video processing pipelines, and animation workflows. Cut render times by N workers.

</td>
<td width="50%" valign="top">

### 🏗️ Distributed Builds
Accelerate **Android builds**, **Rust compilation**, **test suites**, and **CI/CD pipelines** across team hardware.

</td>
</tr>
<tr>
<td colspan="2" valign="top">

### 🔬 Research Computing
Enable low-cost distributed computation for **students**, **researchers**, **universities**, and **labs** — collaborative experiments, data processing, and reproducible science.

</td>
</tr>
</table>



## 🌟 Future Goals

Sangam aims to become:

- 🌍 **Decentralized compute infrastructure**
- 🤖 **Local-first AI orchestration**
- 📡 **Wireless distributed computing platform**
- 🔗 **Collaborative edge-compute network**

> A future where nearby devices become temporary clusters, students train models together, developers borrow compute instantly, teams spin up AI clusters on demand, and unused hardware becomes shared infrastructure.

---

## 🚀 Installation

> ⚠️ **Sangam is in early-stage development.** The pieces below already work end-to-end — peer discovery, TCP messaging, task execution — but the public release is not packaged yet.

### Repository layout (current)

```text
sangam/
├── apps/
│   └── desktop/                # Tauri 2 + React + TypeScript GUI
│       └── src-tauri/          # → depends on sangam-core via path
├── crates/
│   └── core/                   # Sangam runtime (discovery, networking, tasks)
│       ├── src/
│       └── tests/
├── .github/workflows/ci.yml    # fmt · clippy · test · build (parallel: core / desktop)
└── Cargo.toml                  # workspace root
```

### Run the headless runtime

```bash
git clone https://github.com/shiva-manu/Sangam.git
cd Sangam
cargo run -p sangam-core --release
```

You'll see the banner, the node ID, and any peers discovered on your LAN.
Press `Ctrl-C` for a graceful shutdown.

### Run the desktop control plane

Linux prerequisites (Ubuntu/Debian — see [Tauri prereqs](https://tauri.app/start/prerequisites/) for other distros):

```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Then:

```bash
cd apps/desktop
npm install
npm run tauri dev
```

Want to be notified about the first packaged release? **⭐ Star this repo** to follow progress.

---

## 🤝 Contributing

Contributions are warmly welcomed. Sangam is at its earliest stage — your input shapes the foundation.

**Areas of interest:**

| Domain | Looking for |
| :--- | :--- |
| 🌐 Distributed systems | Architects, researchers, contributors |
| 🦀 Rust networking | QUIC, libp2p, async Rust experts |
| 🧠 Scheduler design | Resource-aware scheduling, DAG execution |
| 🔌 P2P systems | NAT traversal, peer discovery, relays |
| 🛡️ Secure runtime isolation | Containers, microVMs, Wasm |
| 🎨 UI/UX | Tauri, React, dashboard design |
| 🤖 AI infrastructure | Distributed inference, multi-agent systems |

**How to contribute:**

1. 🍴 Fork the repository
2. 🌿 Create a feature branch (`git checkout -b feature/amazing-feature`)
3. 💍 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🔁 Open a Pull Request

---

## 📜 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

```
Copyright (c) Sangam Contributors
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
```

---

## 📍 Status

![Status](https://img.shields.io/badge/stage-concept%20%2B%20research-FF6B6B?style=for-the-badge)

**Sangam is currently an early-stage concept, architecture, and research exploration.**

We're laying the groundwork for what wireless distributed compute could look like in 2026 and beyond. Code, prototypes, and a working preview are coming soon.

---

<div align="center">

### 🌊 Built with curiosity, for the next generation of compute.

If Sangam resonates with you, **⭐ star the repo**, **share with your team**, and **join the conversation**.

<sub>Made with ❤️ by people who believe compute should be shared, not rented.</sub>

</div>
