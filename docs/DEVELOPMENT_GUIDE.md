<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# DEVELOPMENT_GUIDE.md

This guide helps developers understand how to work with the hexaFn project. It explains the internal structure, tools, development workflow, and best practices.

---

## 🗂️ Project Structure

- [PROJECT_STRUCTURE](PROJECT_STRUCTURE.md) – Directory structure

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

Follow the naming rules defined in [BRANCH_STRATEGY](BRANCH_STRATEGY.md) and [PR_STRATEGY](PR_STRATEGY.md). Allowed branch prefixes:

- `feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `ci/`, etc.

Create a feature branch:

```bash
git checkout -b feat/new-module
```

Open a PR with a valid title and follow checklist from [PULL_REQUEST_TEMPLATE](../.github/PULL_REQUEST_TEMPLATE.md).

---

## 🧱 Commit Conventions

Use the supported 12 types from [COMMIT_STRATEGY](COMMIT_STRATEGY.md). Example:

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

## 🏛️ Architecture Documentation

Understanding hexaFn's architecture is essential for effective development. Review these key documents:

- [HEXAGONAL_ARCHITECTURE_GUIDE](HEXAGONAL_ARCHITECTURE_GUIDE.md) - Fundamental hexagonal architecture principles
- [RUST_PORTS_ADAPTERS_EXAMPLE](RUST_PORTS_ADAPTERS_EXAMPLE.md) - Comprehensive component catalog organized by architectural layers
- [DATA_FLOW](DATA_FLOW.md) - High-level data flow diagram across all modules
- [DATA_FLOW_SAMPLE](DATA_FLOW_SAMPLE.md) - Sample data flow diagram illustrating the 6F Lifecycle Flow
- [DATA_FLOW_DETAIL](DATA_FLOW_DETAIL.md) - Detailed component interactions with interfaces and methods
- [DATA_MODEL_CORE](DATA_MODEL_CORE.puml) - Core data model definitions
- [DATA_MODEL_BRIDGE](DATA_MODEL_BRIDGE.puml) - Bridge data model definitions
- [DATA_MODEL_TRIGGER](DATA_MODEL_TRIGGER.puml) - Trigger data model definitions
- [DATA_MODEL_RUN](DATA_MODEL_RUN.puml) - Run data model definitions
- [DATA_MODEL_STORE](DATA_MODEL_STORE.puml) - Store data model definitions
- [DATA_MODEL_CAST](DATA_MODEL_CAST.puml) - Cast data model definitions
- [DATA_MODEL_WATCH](DATA_MODEL_WATCH.puml) - Watch data model definitions

These documents explain how components interact across the 6F Lifecycle Flow (Feed → Filter → Format → Function → Forward → Feedback) following clean hexagonal architecture patterns.

When developing a new feature:
1. Identify which module and layer your code belongs to using `RUST_PORTS_ADAPTERS.md`
2. Understand how data flows through the system with `DATA_FLOW.md`
3. Review the sample data flow in `DATA_FLOW_SAMPLE.md`
4. Reference the detailed interfaces and methods in `DATA_FLOW_DETAIL.md`
5. Follow the architectural principles outlined in `HEXAGONAL_ARCHITECTURE_GUIDE.md`
6. Ensure your code adheres to the data model definitions in `DATA_MODEL_*.puml`, etc.

Maintaining architectural integrity is critical - all PRs will be reviewed for compliance with these patterns.

---

## 📚 Documentation & Contributions

All contributors must follow the documentation style and structure:

- Use `///` for public Rust docs
- Update relevant `.md` files in `docs/` when modifying features
- Follow the guide in [CONTRIBUTING](CONTRIBUTING.md)

---

## 🧠 Helpful Resources

- [GETTING_STARTED](GETTING_STARTED.md) – Initial setup and quick commands
- [HEXAGONAL_ARCHITECTURE_GUIDE](HEXAGONAL_ARCHITECTURE_GUIDE.md) - Protocol-agnostic implementation patterns and data flow
- [ARCHITECTURE](ARCHITECTURE.md) – System design and modules
- [USE_CASES](USE_CASES.md) – Functional capabilities
- [CONTACT](CONTACT.md), [SUPPORT](SUPPORT.md) – Communication channels
- [LABELLING_STRATEGY](LABELLING_STRATEGY.md) – Tag issues/PRs correctly
- [PROJECT_STRUCTURE](PROJECT_STRUCTURE.md) – Directory structure
- [BRANCH_STRATEGY](BRANCH_STRATEGY.md) – Branch naming conventions
- [COMMIT_STRATEGY](COMMIT_STRATEGY.md) – Commit message conventions

---

Welcome to the hexaFn dev team! Let’s build it right from the start.
