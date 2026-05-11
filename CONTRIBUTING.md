# Contributing to Sangam

Thank you for helping build Sangam. The project is an early-stage wireless
peer-to-peer compute mesh built with Rust, Tauri, React, and TypeScript.

This guide explains how to set up the project, make focused changes, run
checks, and open a pull request that is easy to review.

## Community Expectations

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
Be direct, respectful, and constructive. Sangam touches distributed systems,
security, networking, and UI, so careful disagreement is welcome when it helps
the project get better.

## Good First Contributions

Helpful areas include:

- Reproducing and documenting networking or discovery issues.
- Improving tests for `sangam-core`.
- Making desktop UI states clearer and more resilient.
- Improving README, architecture notes, and setup instructions.
- Adding examples that demonstrate realistic distributed workloads.
- Reviewing security boundaries and identifying unsafe assumptions.

For larger design changes, open an issue first so maintainers and contributors
can align before implementation.

## Development Environment

### Required

- Rust stable toolchain.
- Node.js and npm.
- Git.

### Desktop App Requirements

The desktop app uses Tauri 2, so your machine also needs the platform-specific
Tauri prerequisites.

Ubuntu/Debian:

```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Other operating systems:

```text
https://tauri.app/start/prerequisites/
```

## Repository Map

```text
apps/desktop/              Tauri desktop app and React frontend
apps/desktop/src-tauri/    Tauri command layer that delegates to sangam-core
crates/core/               Runtime, discovery, networking, peers, tasks, metrics
crates/core/tests/         Integration tests for core behavior
```

## Local Setup

Clone the repository:

```bash
git clone https://github.com/shiva-manu/Sangam.git
cd Sangam
```

Run the core runtime:

```bash
cargo run -p sangam-core
```

Run the desktop app:

```bash
cd apps/desktop
npm install
npm run tauri dev
```

From the repository root, you can also use:

```bash
npm run desktop:dev
npm run desktop:build
npm run desktop:tauri -- dev
```

## Development Workflow

1. Create a focused branch:

   ```bash
   git checkout -b fix/descriptive-name
   ```

2. Keep each change scoped to one idea.
3. Add or update tests when behavior changes.
4. Update documentation when commands, architecture, or user-facing behavior
   changes.
5. Run the relevant checks before opening a pull request.
6. Clearly describe what changed, why it changed, and how you verified it.

## Checks

Core runtime:

```bash
cargo fmt --all --check
cargo clippy -p sangam-core --all-targets -- -D warnings
cargo test -p sangam-core
```

Full Rust workspace, including the Tauri crate:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Desktop app:

```bash
cd apps/desktop
npm install
npm run build
```

If a check cannot run on your machine because of missing platform packages,
network restrictions, or operating-system limitations, mention that in the pull
request.

## Code Style

Rust:

- Prefer clear module boundaries and small public APIs.
- Keep async code explicit about ownership, cancellation, and errors.
- Use typed data structures instead of ad hoc string parsing where practical.
- Keep Tauri command logic thin; shared runtime behavior belongs in
  `sangam-core`.

TypeScript and React:

- Keep UI components focused and reusable.
- Handle loading, empty, and error states.
- Keep desktop UI behavior aligned with the runtime state rather than mocked
  assumptions.
- Avoid introducing new styling systems unless there is a strong reason.

Documentation:

- Separate implemented behavior from planned behavior.
- Prefer commands that can be copied and run from a known directory.
- Mention operating-system assumptions when they matter.

## Testing Guidance

Add tests when you change:

- Peer discovery parsing or address selection.
- Networking message formats.
- Task execution or task lifecycle tracking.
- Metrics, logging, or registry behavior.
- Tauri command behavior that affects runtime state.

For networked behavior, prefer tests around small deterministic functions where
possible, then add integration coverage for the runtime path.

## Commit Messages

Use concise imperative messages:

```text
Add peer registry stale-node cleanup
Fix task tracker failure state
Document Tauri setup requirements
```

## Pull Request Checklist

Before opening a pull request, confirm:

- The change is focused and described clearly.
- Relevant tests were added or updated.
- Relevant documentation was updated.
- Formatting and lint checks were run where practical.
- Any skipped checks are explained.
- Screenshots or short recordings are included for visible UI changes.

## Reporting Bugs

When opening an issue, include:

- What you expected to happen.
- What actually happened.
- Steps to reproduce.
- Operating system and network context.
- Relevant logs or screenshots.
- Whether the issue affects the core runtime, desktop app, or both.

## Reporting Security Issues

Please do not open public issues for security vulnerabilities. Use GitHub
private security advisories when available, or contact the repository
maintainers through the repository owner profile.

Sangam is not yet safe for untrusted arbitrary workloads. Treat the current
runtime as a prototype and test it only on machines and networks you control.
