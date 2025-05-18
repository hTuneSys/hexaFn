<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# 🚀 Getting Started with hexaFn

Welcome, contributor! This guide will help you get up and running with the **hexaFn** codebase and contribution flow.

---

## 🧱 Project Overview

**hexaFn** is a modular, programmable event engine built around the `6F Lifecycle Flow`:

> **Feed → Filter → Format → Function → Forward → Feedback**

It consists of six core modules:

- `HexaStore` – Typed, event-driven key-value store  
- `HexaCast` – Real-time pub-sub and broadcast engine  
- `HexaRun` – Dynamic function runtime (WASM, JS, DSL)  
- `HexaTrigger` – Conditional triggers and flow orchestration  
- `HexaWatch` – Logging, audit, and observability system  
- `HexaBridge` – External integrations via webhooks, SDKs, APIs

---

## 💻 Local Setup

### 1. Clone the repository

```bash
git clone https://github.com/hTuneSys/hexaFn.git
cd hexaFn
```

### 2. Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
rustup component add clippy rustfmt
```

### 3. Run tests

```bash
cargo test
```

---

## 🔧 Contribution Flow

1. **Create your branch**  
   ```bash
   git checkout -b feat/run/add-custom-wasm
   ```

2. **Make your changes**, then commit with a conventional message  
   ```bash
   git commit -m "feat(run): support custom WASM execution"
   ```

3. **Push to your fork** and **open a pull request** against the `develop` branch.

4. **Ensure** the following before submitting:
   - ✅ Branch name matches pattern (`feat/*`, `fix/*`, etc.)
   - ✅ PR title follows semantic format
   - ✅ Passes CI (`cargo fmt`, `clippy`, `test`)
   - ✅ Checks off modules in PR template

---

## 📦 Tools & Conventions

| Tool                | Purpose                                |
|---------------------|----------------------------------------|
| `cargo fmt`         | Code formatting                        |
| `cargo clippy`      | Code linting and static analysis       |
| `cargo tarpaulin`   | Code coverage (optional)               |
| `commitlint`        | Commit message validation              |
| `semantic-release`  | Automated versioning and changelogs    |
| `labeler.yml`       | Auto-labeling based on file paths      |

---

## 🥪 Run Specific Tests

To run tests from a specific module (if modularized into crates or directories):

```bash
cargo test -p hexastore
```

Or using path filtering:
```bash
cargo test --test test_name
```

---

## 💡 Tips for Contributors

- Keep your changes focused and atomic (1 feature/fix per PR)
- Prefer pluggable, isolated logic in modules or functions
- Ask questions in Discussions if uncertain
- Review [CONTRIBUTING.md](../.github/CONTRIBUTING.md) for standards

---

## 📬 Contact

- Email: [info@hexafn.com](mailto:info@hexafn.com)  
- Discussions: [GitHub Discussions](https://github.com/hTuneSys/hexaFn/discussions)

---

Thanks for contributing to **hexaFn** –  
**Together, we turn events into logic.**
