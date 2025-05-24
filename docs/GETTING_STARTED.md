<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# GETTING_STARTED.md

Welcome to the hexaFn project! This guide will help you set up your environment, build the project, and understand the basics of how to start using or contributing to hexaFn.

---

## 🔧 Prerequisites

Ensure the following tools are installed:

- [Rust](https://www.rust-lang.org/tools/install)
- `cargo` (comes with Rust)
- `git` (version control)
- Optional: `just`, `make`, or `cargo-make` for task automation

---

## 📦 Clone the Repository

```bash
git clone https://github.com/hTuneSys/hexaFn.git
cd hexaFn
```

---

## 📁 Install Dependencies

hexaFn uses native Rust tooling.

```bash
cargo fetch
```

To install auxiliary tools (if configured):

```bash
cargo install cargo-make
```

---

## 🚀 Run the Project (Example)

```bash
cargo run
```

For custom environments:

```bash
HEXA_ENV=dev HEXA_PORT=4000 cargo run
```

---

## 🧪 Run Tests

```bash
cargo test
```

To run specific tests:

```bash
cargo test test_function_name
```

---

## ✅ Verify Formatting & Linting

Format code using:

```bash
cargo fmt
```

Check for lint errors:

```bash
cargo clippy
```

---

## 📚 Next Steps

- Check [`ARCHITECTURE.md`](ARCHITECTURE.md) to understand system design
- Use [`USE_CASES.md`](USE_CASES.md) to explore core functionalities
- Review [`CONTRIBUTING.md`](CONTRIBUTING.md) to contribute effectively

---

You're now ready to explore and build with hexaFn. Happy hacking!