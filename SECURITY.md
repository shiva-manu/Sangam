# Security Policy

## ⚠️ Current Security Status

Sangam is in **early development** and is **not yet suitable for running
untrusted workloads or operating on untrusted networks**. The current prototype
lacks production-grade isolation, encrypted transport, and formal trust
management.

**Use Sangam only on networks and machines you control.**

## Supported Versions

Sangam has not reached a stable release yet. Security fixes will be applied to
the latest development branch only.

| Version       | Supported          |
| ------------- | ------------------ |
| `main` branch | ✅ Active development |
| Older commits | ❌ No backports     |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, use one of the following channels:

1. **GitHub Private Security Advisories** (preferred):
   Navigate to the [Security Advisories](https://github.com/shiva-manu/Sangam/security/advisories)
   tab and create a new private advisory.

2. **Direct contact**: Reach out to the repository maintainer through their
   [GitHub profile](https://github.com/shiva-manu).

### What to Include

- Description of the vulnerability and its potential impact.
- Steps to reproduce or a proof of concept.
- Affected components (discovery, networking, task execution, desktop app).
- Suggested fix, if you have one.

### Response Timeline

- **Acknowledgment**: Within 72 hours of report.
- **Initial assessment**: Within 1 week.
- **Fix or mitigation**: Depends on severity; critical issues are prioritized.

We will credit reporters in release notes unless anonymity is requested.

## Security Architecture & Known Limitations

### Current State (Prototype)

| Area | Status | Risk |
|------|--------|------|
| Transport | Unencrypted TCP on port 8080 | 🔴 High — messages are plaintext |
| Peer identity | No authentication | 🔴 High — any node can join |
| Task execution | No sandboxing | 🔴 High — tasks run with full process privileges |
| Discovery | mDNS on local network | 🟡 Medium — any LAN device can discover/join |
| Desktop app | Local-only Tauri commands | 🟢 Low — no remote attack surface |

### Planned Security Improvements

These are on the roadmap but **not yet implemented**:

- **Encrypted transport**: Replace TCP with QUIC + TLS for all node-to-node
  communication.
- **Peer authentication**: Cryptographic node identity with key-pair-based
  mutual authentication.
- **Trust and pairing**: Explicit cluster membership with invite/accept flows.
- **Workload isolation**: Container, WebAssembly, or microVM sandboxing for
  task execution with resource limits.
- **Permission model**: Fine-grained controls over what workloads can access.
- **Signed messages**: Integrity verification for all inter-node messages.

## Security Best Practices for Users

While Sangam is in development:

1. **Run only on trusted private networks** — do not expose Sangam to the
   public internet.
2. **Use a firewall** — restrict port 8080 to known local IPs.
3. **Do not run sensitive workloads** — the prototype has no isolation.
4. **Keep dependencies updated** — run `cargo update` and `npm audit fix`
   regularly.
5. **Review peer connections** — use the desktop UI to verify that only
   expected nodes are in the mesh.

## Dependency Security

Sangam uses:

- **Rust crates** managed via `Cargo.toml` — auditable with
  [`cargo audit`](https://github.com/rustsec/rustsec).
- **npm packages** for the desktop frontend — auditable with `npm audit`.

We recommend contributors run these checks before submitting code:

```bash
cargo audit
cd apps/desktop && npm audit
```

## Scope

The following are **in scope** for security reports:

- Vulnerabilities in Sangam's Rust runtime (`crates/core/`).
- Vulnerabilities in the Tauri command layer (`apps/desktop/src-tauri/`).
- Vulnerabilities in the desktop frontend that could lead to code execution.
- Dependency vulnerabilities that are exploitable in Sangam's context.

The following are **out of scope**:

- Issues that require physical access to the machine.
- Denial of service on local networks (inherent to LAN-based systems).
- Vulnerabilities in upstream dependencies with no path to exploitation in
  Sangam.
- Social engineering attacks.

## License

This security policy is part of the Sangam project, released under the
[MIT License](LICENSE).
