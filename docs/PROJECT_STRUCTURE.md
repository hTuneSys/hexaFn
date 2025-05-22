<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# 📁 Project Structure: `hexaFn`

This document defines the modular folder architecture for the `hexaFn` project, following **Hexagonal Architecture** and **Domain-Driven Design (DDD)** principles.  
Each module corresponds to one or more steps in the **6F Lifecycle Flow**:  
`Feed → Filter → Format → Function → Forward → Feedback`

---

## 📦 Root Layout

```plaintext
src/
├── core/
├── modules/
├── common/
├── cli/
└── main.rs
```

---

## 🧠 Core Layer (`src/core/`)

| Folder            | Purpose                                                                 |
|-------------------|-------------------------------------------------------------------------|
| `domain/`         | Shared domain models & interfaces (e.g. events, ports)                 |
| `application/`    | Orchestration logic (Feed → Filter → Format coordination)              |
| `infrastructure/` | Global config loading, initialization routines                         |

---

## 🧩 Functional Modules (`src/modules/`)

Each module maps to a stage (or stages) of the 6F pipeline and follows the same internal structure:  
`domain/`, `application/`, and `infrastructure/`

| Module        | Lifecycle Stage(s) | Description                                            |
|---------------|--------------------|--------------------------------------------------------|
| `trigger/`| 🟢 Feed / Filter    | Trigger engine: evaluates incoming events & conditions |
| `run/`    | 🟡 Function         | Function execution: DSL / WASM / JS / Lua runtimes     |
| `store/`  | 🔵 Forward          | Key-Value store: stores execution outcomes              |
| `cast/`   | 🔵 Forward          | Pub/Sub messaging: broadcasts internal events          |
| `watch/`  | 🟣 Feedback         | Observability: logging, metrics, auditing              |
| `bridge/` | 🔁 Integration      | Bridges: connects to external systems (e.g. webhooks)  |

Example structure for each module:

```plaintext
run/
├── domain/
├── application/
└── infrastructure/
```

---

## 🔧 Shared Utilities (`src/common/`)

| Folder         | Purpose                                                    |
|----------------|------------------------------------------------------------|
| `config/`      | Environment and settings loader                            |
| `events/`      | Shared event types and internal event bus definitions      |
| `logger/`      | Centralized logging infrastructure                         |
| `utils/`       | Helper functions, constants, error handling, etc.          |

---

## 💻 CLI Interface (`src/cli/`)

Command-line interface definitions, CLI subcommands for operations like:

- Reloading triggers
- Running dry DSL functions
- Publishing test events
- Inspecting logs or config

---

## 🚀 Entry Point

```plaintext
main.rs
```

Responsible for:

- Initializing global config
- Bootstrapping modules
- Launching the 6F pipeline

---

> ✅ This structure promotes modularity, testability, and maintainability. Each module can evolve independently while conforming to a unified architectural standard.
