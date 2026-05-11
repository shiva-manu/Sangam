# Contributing to Sangam

Thank you for your interest in contributing to Sangam. This project is an early-stage wireless peer-to-peer distributed compute mesh built with Rust, Tauri, React, and TypeScript.

## Ways to contribute

- Report bugs and reproducible issues
- Suggest improvements to the runtime, networking, scheduler, or desktop UI
- Improve documentation and examples
- Add tests for existing behavior
- Help validate the project on different operating systems and networks

## Development setup

### Prerequisites

- Rust stable toolchain
- Node.js and npm
- Tauri system prerequisites for your operating system

On Ubuntu/Debian, the desktop app requires:

```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### Run the core runtime

```bash
cargo run -p sangam-core
```

### Run the desktop app

```bash
npm install
npm run desktop:tauri -- dev
```

You can also run commands from the desktop package directly:

```bash
cd apps/desktop
npm install
npm run tauri dev
```

## Checks before opening a pull request

Run the relevant checks before submitting changes:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run desktop:build
```

If the desktop build requires system packages that are unavailable on your machine, mention that in the pull request.

## Pull request process

1. Fork the repository.
2. Create a focused branch for your change.
3. Keep changes small and easy to review.
4. Add or update tests when changing behavior.
5. Update documentation when changing commands, architecture, or user-facing behavior.
6. Open a pull request using the template.

## Commit style

Use clear, concise commit messages that describe the change, for example:

```text
Add peer metrics collector
Fix desktop build script
Document Tauri setup
```

## Reporting security issues

Please do not open public issues for security vulnerabilities. Use GitHub private security advisories when available, or contact the repository maintainers through the repository owner profile.
