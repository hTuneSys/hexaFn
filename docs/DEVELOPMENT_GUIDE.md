<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# DEVELOPMENT_GUIDE.md

This guide helps developers understand how to work with the hexaFn project. It explains the internal structure, tools, development workflow, and best practices.

---

## 🗂️ Project Structure

```
hexaFn/
├── src/                # Main source code
├── docs/               # Project documentation
├── .github/            # Issue templates, workflows, and configs
├── Cargo.toml          # Rust project definition
├── rust-toolchain.toml # Toolchain version pinning
├── .releaserc.yml      # Release automation config
├── .gitignore          # Git exclusion rules
└── README.md           # Project overview
```

---

## 🔧 Development Environment Setup

Ensure the following are installed:

- [Rust](https://www.rust-lang.org/tools/install)
- `cargo`, `clippy`, `rustfmt`
- Optional: `cargo-make`, `just`, `watchexec` for automation

Install dependencies:

```bash
cargo fetch
```

---

## 🚀 Building the Project

Build the full project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

Run with environment overrides:

```bash
HEXA_ENV=dev HEXA_DEBUG=true cargo run
```

---

## 🧪 Testing

Run the full test suite:

```bash
cargo test
```

Run a specific test:

```bash
cargo test function_name
```

---

## 🧼 Linting & Formatting

Ensure all code is clean and formatted before pushing:

```bash
cargo fmt       # Format code
cargo clippy    # Static analysis
```

All PRs must pass these checks.

---

## 🌲 Logging & Debugging

Use the `HEXA_DEBUG` environment variable to enable debug logs:

```bash
HEXA_DEBUG=true cargo run
```

For detailed tracing, integrate with `HexaWatch` logging module.

---

## 🔀 Branching & PRs

Follow the naming rules defined in [`BRANCH_STRATEGY.md`](BRANCH_STRATEGY.md) and [`PR_STRATEGY.md`](PR_STRATEGY.md). Allowed branch prefixes:

- `feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `ci/`, etc.

Create a feature branch:

```bash
git checkout -b feat/new-module
```

Open a PR with a valid title and follow checklist from [`PULL_REQUEST_TEMPLATE.md`](../.github/PULL_REQUEST_TEMPLATE.md).

---

## 🧱 Commit Conventions

Use the supported 12 types from [`COMMIT_STRATEGY.md`](COMMIT_STRATEGY.md). Example:

```bash
feat: add token parsing logic
```

Use semantic and meaningful commits. PRs with vague or mixed commits will be rejected.

---

## 🧪 CI/CD Pipeline

CI runs automatically via GitHub Actions:

- Linting (`clippy`)
- Formatting (`rustfmt`)
- Tests (`cargo test`)
- Branch and commit validation

---

## 🛡️ Branch Protection

Branches like `main`, `release`, and `develop` are protected:

- Require PR review
- Require status checks
- Require conventional commit title

---

## 📚 Documentation & Contributions

All contributors must follow the documentation style and structure:

- Use `///` for public Rust docs
- Update relevant `.md` files in `docs/` when modifying features
- Follow the guide in [`CONTRIBUTING.md`](CONTRIBUTING.md)

---

## 🧠 Helpful Resources

- [`GETTING_STARTED.md`](GETTING_STARTED.md) – Initial setup and quick commands
- [`ARCHITECTURE.md`](ARCHITECTURE.md) – System design and modules
- [`USE_CASES.md`](USE_CASES.md) – Functional capabilities
- [`CONTACT.md`](CONTACT.md), [`SUPPORT.md`](SUPPORT.md) – Communication channels
- [`LABELLING_STRATEGY.md`](LABELLING_STRATEGY.md) – Tag issues/PRs correctly
- [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md) – Directory structure
- [`BRANCH_STRATEGY.md`](BRANCH_STRATEGY.md) – Branch naming conventions
- [`COMMIT_STRATEGY.md`](COMMIT_STRATEGY.md) – Commit message conventions

---

Welcome to the hexaFn dev team! Let’s build it right from the start.