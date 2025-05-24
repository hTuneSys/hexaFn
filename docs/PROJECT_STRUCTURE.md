<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
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
├── scripts/            # Automation & utility scripts (dev, deploy, test)
├── crates/             # Cargo workspace - Business domains (Bounded Contexts)
├── docs/               # Architecture documentation and guides
├── examples/           # Usage examples and tutorials
├── integration-tests/  # Cross-domain integration tests
├── benchmarks/         # Performance benchmarks
├── .github/            # GitHub workflows and templates
├── LICENSES/           # License files
├── LICENSE             # Main license file
├── Cargo.toml          # Workspace configuration
├── Cargo.lock          # Dependency lock file
├── package.json        # Semantic-release configuration
├── REUSE.toml          # REUSE compliance configuration
├── README.md           # Project overview
├── .releaserc.yml      # Semantic release configuration
├── .gitignore          # Git ignore rules
└── CHANGELOG.md        # Version history
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

## 🔹 Modular Design: `crates/<domain>/src/`

Each domain crate is self-contained and organized in **Hexagonal Architecture** layout:

```plaintext
crates/<domain>/src/
├── domain/              # Core business rules (entities, value objects, domain services)
│   ├── entities/        # Aggregates and core business objects with identity
│   ├── value_objects/   # Immutable value types without identity
│   ├── events/          # Domain events for cross-domain communication
│   ├── services/        # Domain services containing business logic
│   └── contracts/       # Domain contracts and interfaces (core only)
├── application/         # Use cases, application services, port definitions
│   ├── commands/        # State-changing operations (CQRS Commands)
│   ├── queries/         # Read operations (CQRS Queries)
│   ├── ports/           # Interface definitions (dependency inversion)
│   └── services/        # Application orchestration services
├── infrastructure/      # Concrete implementations of output ports
│   ├── persistence/     # Database adapters, repositories
│   ├── messaging/       # Event publishing, message queue adapters
│   └── external/        # External API clients, third-party integrations
└── lib.rs               # Crate entry point (re-exports and public API)
```

---

## 🛋️ Module Overview (6F Mapping)

| Module                | Lifecycle Stage(s) | Responsibility                                          |
| --------------------- | ------------------ | ------------------------------------------------------- |
| `hexafn-core/`        | Shared Kernel      | Domain contracts, shared types, 6F lifecycle traits    |
| `hexafn-trigger/`     | Feed / Filter      | Detects & evaluates events                              |
| `hexafn-run/`         | Function           | Executes logic (DSL, WASM, scripts)                     |
| `hexafn-store/`       | Forward            | Persists outputs or states                              |
| `hexafn-cast/`        | Forward            | Broadcasts events/messages (e.g., to pub/sub)           |
| `hexafn-watch/`       | Feedback           | Collects logs, telemetry, audits                        |
| `hexafn-bridge/`      | Integration        | Interfaces with external systems (e.g., webhooks, APIs) |

---

## 🏗️ Cargo Workspace Configuration

```toml
# Root Cargo.toml
[workspace]
members = [
    "crates/hexafn-core",
    "crates/hexafn-trigger", 
    "crates/hexafn-run",
    "crates/hexafn-store",
    "crates/hexafn-cast", 
    "crates/hexafn-watch",
    "crates/hexafn-bridge",
]
resolver = "2"

[workspace.dependencies]
# Shared dependencies across all crates
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
# ... other shared dependencies

# Internal dependencies
hexafn-core = { path = "crates/hexafn-core" }
```

---

## 🧬 Domain Dependencies

```plaintext
┌─────────────────┐
│  hexafn-bridge  │
└─────────┬───────┘
          │
┌─────────┼───────┐
│  hexafn-watch   │
└─────────┬───────┘
          │
┌─────────┼───────┐ ┌─────────────────┐ ┌─────────────────┐
│  hexafn-cast    │ │  hexafn-store   │ │  hexafn-run     │
└─────────┬───────┘ └─────────┬───────┘ └─────────┬───────┘
          │                   │                   │
          └─────────┬─────────┴─────────┬─────────┘
                    │                   │
          ┌─────────┼───────┐ ┌─────────┼───────┐
          │ hexafn-trigger  │ │   hexafn-core   │
          └─────────────────┘ └─────────────────┘
                                       │
                              (Shared Domain Kernel)
```

**Dependency Rules:**

* ✅ Business domains → `hexafn-core` (only)
* ✅ `hexafn-core` → External libraries (only)
* ❌ No circular dependencies
* ❌ No direct inter-domain dependencies

---

## 📚 Testing Strategy

```plaintext
integration-tests/     # Cross-domain integration tests
├── pipeline_flow/     # End-to-end 6F lifecycle tests
├── domain_events/     # Inter-domain communication tests
└── README.md

benchmarks/           # Performance benchmarks
├── pipeline_throughput/
├── domain_latency/
└── README.md

examples/            # Usage examples and tutorials
├── basic_pipeline/  # Simple 6F pipeline example
├── event_driven/    # Event-driven architecture example
└── README.md
```

**Testing Levels:**

* **Unit Tests**: In each crate's `src/` directory
* **Integration Tests**: In `integration-tests/` directory
* **Performance Tests**: In `benchmarks/` directory
* **Examples**: In `examples/` directory

---

## 🎯 Development Workflow

### **Per-Domain Development:**

```bash
# Work on a specific domain
cd crates/hexafn-trigger
cargo test
cargo check

# Test specific domain integration
cargo test --workspace --test trigger_integration
```

### **Workspace-wide Operations:**

```bash
# Build entire workspace
cargo build --workspace

# Test all domains
cargo test --workspace

# Check all dependencies
cargo check --workspace
```

---

> ✅ This structure enforces separation of concerns, strict domain boundaries, and high testability while allowing domains to evolve independently. The Cargo workspace approach provides strong dependency management and enables parallel development across teams while maintaining architectural integrity through the shared domain kernel (`hexafn-core`).
