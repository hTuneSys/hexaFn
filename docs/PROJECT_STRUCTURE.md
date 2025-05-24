<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# 📁 Project Structure: `hexaFn`

This document defines the modular architecture for the `hexaFn` project using **Hexagonal Architecture (Ports & Adapters)** and **Domain-Driven Design (DDD)** principles.

Each module in this system aligns with one or more of the **6F Lifecycle Flow** stages:
`Feed → Filter → Format → Function → Forward → Feedback`

---

## 📦 Root Layout

```plaintext
.
├── config/             # Global configuration and environment loaders
├── modules/            # Business modules (each follows Hexagonal + DDD)
├── shared/             # Reusable types and utilities across modules
├── src/
│   ├── bootstrap.rs    # System-wide orchestrator (composition root)
│   └── main.rs         # Program entry point
└── tests/              # Integration and scenario-level tests
```

---

## 🧠 Hexagonal + DDD Explained

| Concept         | Role                                                                    |
| --------------- | ----------------------------------------------------------------------- |
| **Entity**      | Core domain object with identity and lifecycle (e.g. `Trigger`, `Task`) |
| **ValueObject** | Describes an attribute set without identity (e.g. `TimeRange`)          |
| **DTO**         | Data Transfer Object between boundaries (e.g. HTTP → use case)          |
| **Use Case**    | Application behavior entrypoint; often orchestrates domain logic        |
| **Port**        | Trait/Interface describing expected behavior                            |
| **Adapter**     | Concrete implementation of a port                                       |

* **Input Ports:** called by `presentation` (e.g. `run_task()`)
* **Output Ports:** consumed by `infrastructure` (e.g. `TaskRepo`, `WebhookSender`)

---

## 🔹 Modular Design: `modules/<mod>/src/`

Each module is self-contained and organized in `Hexagonal Architecture` layout:

```plaintext
modules/<mod>/src/
├── internal/
│   ├── domain/          # Core business rules (entities, VOs, domain services)
│   └── application/     # Use cases, application services, port definitions
│       ├── service/     # Application logic (uses ports)
│       ├── use_case/    # Explicitly named high-level actions
│       └── port/
│           ├── input/   # Called by external interfaces (e.g. controller)
│           └── output/  # Implemented by adapters (e.g. DB, HTTP clients)
├── external/
│   ├── infrastructure/  # Concrete implementations of output ports
│   │   └── adapter/      # e.g., DB, MQ, HTTP, File adapters
│   └── presentation/    # Interfaces (e.g. HTTP controllers, CLI handlers)
│       ├── controller/   # Entrypoints: accepts request and invokes use case
│       └── dto.rs       # Incoming/outgoing data structures
└── lib.rs               # Module entry (re-exports or registry)
```

---

## 🛋️ Module Overview (6F Mapping)

| Module     | Lifecycle Stage(s) | Responsibility                                          |
| ---------- | ------------------ | ------------------------------------------------------- |
| `trigger/` | Feed / Filter      | Detects & evaluates events                              |
| `run/`     | Function           | Executes logic (DSL, WASM, scripts)                     |
| `store/`   | Forward            | Persists outputs or states                              |
| `cast/`    | Forward            | Broadcasts events/messages (e.g., to pub/sub)           |
| `watch/`   | Feedback           | Collects logs, telemetry, audits                        |
| `bridge/`  | Integration        | Interfaces with external systems (e.g., webhooks, APIs) |

---

## 🔧 Shared Layer: `shared/`

```plaintext
shared/src/
├── dto.rs         # Common DTO types
├── error.rs       # Global error handling definitions
├── utils.rs       # Helper functions, reusable constants
└── lib.rs         # Exposes public shared items
```

---

## 🏋️ Configuration Layer: `config/`

```plaintext
config/
├── settings.rs    # Structs and logic to parse environment/config files
└── mod.rs         # Exports config API
```

Supports `*.env`, `*.toml`, `*.yaml` configuration loading for all environments.

---

## 🚀 Entry Layer

```plaintext
src/
├── bootstrap.rs    # Loads config, wires dependencies, registers modules
└── main.rs         # Starts the runtime and invokes bootstrap
```

This is the **composition root** of the system.

---

## 📚 Testing: `tests/`

```plaintext
tests/
├── integration/     # Black-box tests for module interoperability
└── README.md
```

Scenarios can be aligned with 6F stages (e.g. `run_success.rs`, `bridge_failure.rs`).

---

> ✅ This structure enforces separation of concerns, strict module boundaries, and high testability while allowing modules to evolve independently in a loosely coupled manner.
