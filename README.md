<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# hexaFn

![hexaFn Logo](docs/assets/hexaFn-banner.png)

## Function Composition Framework for Rust

[![Crates.io](https://img.shields.io/crates/v/hexafn.svg)](https://crates.io/crates/hexafn)
[![Documentation](https://docs.rs/hexafn/badge.svg)](https://docs.rs/hexafn)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**From Feed to Feedback, fully programmable.**  
A modular, event-driven function pipeline powered by the 6F Lifecycle Flow. Built with documentation-first engineering, composable primitives, and developer joy at its core.

---

## 🚀 What is hexaFn?

**hexaFn** is a programmable, event-driven data engine designed to move data through a powerful architecture called the **6F Lifecycle Flow**:

> **Feed → Filter → Format → Function → Forward → Feedback**

hexaFn enables reactive pipelines that ingest, transform, route, and monitor data in real time—ideal for automation, serverless actions, messaging systems, or custom workflows.

Built by [hexaTune Team](https://hexafn.com) and maintained by [hTuneSys](https://github.com/hTuneSys), it combines **developer-first design** with production-ready modularity.

---

## 🔄 6F Lifecycle Flow

Each phase in the 6F architecture represents a focused stage of data handling:

| Step      | Purpose |
|-----------|---------|
| **Feed**  | Ingest from external sources (events, APIs, queues) |
| **Filter**| Pre-condition checks and gating |
| **Format**| Normalize, transform, validate |
| **Function**| Execute logic with user-defined behavior |
| **Forward**| Route results to KV stores, topics, services |
| **Feedback**| Log, trace, trigger, or audit |

  ![6F Lifecycle Diagram](docs/assets/diagram.png)

---

## 🧠 Why hexaFn?

- ✅ Modular, composable and testable
- ✅ Full documentation-first approach (`/docs`)
- ✅ GitHub-native structure: templates, workflows, linting
- ✅ MIT Licensed, SPDX-Compliant, REUSE-Spec Ready
- ✅ Created for plugin creators, pipeline architects, and system hackers

---

## 🧩 Core Modules

| Module         | Role |
|----------------|------|
| `HexaStore`    | Event-driven KV storage |
| `HexaCast`     | Pub-sub messaging engine |
| `HexaRun`      | Function runtime (WASM, JS, DSL) |
| `HexaTrigger`  | Trigger orchestration engine |
| `HexaWatch`    | Observability & audit tracing |
| `HexaBridge`   | Webhooks, SDK, external integration |

---

## 👤 Who is it for?

| Persona              | Use Case |
|----------------------|----------|
| **Backend Architects** | Event-driven systems, microservices |
| **Realtime Hackers**   | Chat, IoT, multiplayer games |
| **Automation Engineers** | Workflow orchestration |
| **Infra Builders**      | Lightweight programmable runtimes |
| **AI Developers**       | Live inference pipelines |
| **Plugin Authors**      | Writing reusable logic units |

---

## ❌ What hexaFn is NOT

| Myth         | Reality |
|--------------|---------|
| `Never-SQL`  | Not a relational DB |
| `No-Bloat`   | Not a monolith |
| `No-Wait`    | Not batch-oriented |
| `No-LockIn`  | Fully open & extensible |
| `No-Magic`   | Explicit, testable logic |
| `No-CMS`     | Not a frontend framework |

---

## 📚 Documentation

Everything is documented under the `/docs/` folder. Start with:

- [`GETTING_STARTED.md`](docs/GETTING_STARTED.md)
- [`HEXAGONAL_ARCHITECTURE_GUIDE.md`](docs/HEXAGONAL_ARCHITECTURE_GUIDE.md)
- [`DEVELOPMENT_GUIDE.md`](docs/DEVELOPMENT_GUIDE.md)
- [`USE_CASES.md`](docs/USE_CASES.md)
- [`COMMIT_STRATEGY.md`](docs/COMMIT_STRATEGY.md)
- [`PR_STRATEGY.md`](docs/PR_STRATEGY.md)
- [`LABELLING_STRATEGY.md`](docs/LABELLING_STRATEGY.md)
- [`ROADMAP.md`](docs/ROADMAP.md)

Interactive web view at: [https://hexafn.com](https://hexafn.com)

---

## 📦 Installation

Coming soon. CLI and runtime APIs under development.

> Follow progress by watching this repo or joining the discussion tab.

---

## 🔧 Dev Quickstart

```bash
git clone https://github.com/hTuneSys/hexaFn.git
cd hexaFn
cargo build
cargo run
```

Use `.env` or CLI args for environment config. See [`CONFIGURATION.md`](docs/CONFIGURATION.md).

---

## 🧪 Lint & Test

```bash
cargo fmt      # Code formatting
cargo clippy   # Linting
cargo test     # Run tests
```

CI pipelines validate formatting, commit types, and workflow rules.

---

## ✍️ Contributing

We welcome contributors! Please read:

- [`SUMMARY.md`](docs/SUMMARY.md)
- [`CONTRIBUTING.md`](docs/CONTRIBUTING.md)
- [`CODE_OF_CONDUCT.md`](docs/CODE_OF_CONDUCT.md)

Good first issues: [help wanted](https://github.com/hTuneSys/hexaFn/labels/help%20wanted)

---

## 👨‍💻 Author & Maintainers

Built and maintained by **Husamettin ARABACI** and the [hexaTune](https://github.com/hTuneSys) team.  
Project SPDX compliant. Fully REUSE Spec 3.3 compatible.

---

## 📜 License

Licensed under the [MIT License](./LICENSE)  
© 2025 hexaTune LLC. All rights reserved.

SPDX headers are present in all source files.  
This project is REUSE-compliant and safe for enterprise adoption
