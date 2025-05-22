<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# Project Structure: hexaFn

This document outlines the proposed directory structure for the `hexaFn` project based on Hexagonal Architecture and Domain-Driven Design (DDD). Each module aligns with a step in the **6F Lifecycle Flow**:  
**Feed → Filter → Format → Function → Forward → Feedback**

```plaintext
src/
├── core/              # Core pipeline engine coordinating the 6F lifecycle
│   ├── domain/        # Core domain models and port interfaces (shared event definitions)
│   ├── application/   # Pipeline orchestration logic (manages Feed→Filter→Format transitions)
│   └── infrastructure/ # Infrastructure for core setup (e.g., configuration loading, bootstrapping)
├── modules/           # Independent modules corresponding to 6F steps
│   ├── hexastore/     # Forward stage: Event-driven key-value store
│   │   ├── domain/        # Storage models and event hooks
│   │   ├── application/   # Service layer managing store operations and event triggering
│   │   └── infrastructure/ # Storage backends (in-memory, disk, etc.)
│   ├── hexacast/      # Forward stage: Pub/Sub messaging system
│   │   ├── domain/        # Topic and subscription definitions
│   │   ├── application/   # Message publishing and subscription management
│   │   └── infrastructure/ # Messaging backends (internal queues, sockets, etc.)
│   ├── hexarun/       # Function stage: Function runtime execution
│   │   ├── domain/        # Function definitions and sandbox policies
│   │   ├── application/   # Loading, executing, and managing function lifecycles
│   │   └── infrastructure/ # Runtime engines (e.g., WASM, JS interpreters)
│   ├── hexatrigger/   # Feed & Filter stages: Conditional trigger engine
│   │   ├── domain/        # Event and condition models for triggers
│   │   ├── application/   # Evaluates triggers and schedules functions
│   │   └── infrastructure/ # External sources and schedulers (e.g., webhooks, cron jobs)
│   ├── hexawatch/     # Feedback stage: Observability and logging
│   │   ├── domain/        # Log structures and audit/metric events
│   │   ├── application/   # Event collection and notification logic
│   │   └── infrastructure/ # Output adaptors (central logging, metric sinks)
│   └── hexabridge/    # Integration module: Bridges to external systems
│       ├── domain/        # Definitions for third-party services and event exchange
│       ├── application/   # Manages data transfer between hexaFn and outside world
│       └── infrastructure/ # Adaptors: webhooks, APIs, SDKs
├── common/            # Shared utilities across modules
│   ├── config/        # Configuration loader and environment setup
│   ├── events/        # Shared event types and internal event bus
│   ├── logger/        # Centralized logging utility
│   └── utils/         # General helpers and common types (error handling, constants)
├── cli/               # Command-line interface tools and subcommands
└── main.rs            # Entry point: initializes configuration and triggers the 6F pipeline
```

> This structure enables independent development, testing, and deployment of each module while keeping the system highly maintainable and extensible.
