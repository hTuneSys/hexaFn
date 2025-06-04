<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📊 hexaFn Data Flow Overview

This diagram provides a high-level architectural overview of the hexaFn project, showing the relationships and data flows between all main modules and their interaction with external systems. It is designed to align with the 6F Lifecycle Flow (**Feed → Filter → Format → Function → Forward → Feedback**) and the Hexagonal Architecture principles. For detailed component-level diagrams, see `DATA_FLOW_DETAIL.md`.

---

## 🔄 High-Level Architecture Diagram

```mermaid
flowchart TB
    %% External Systems
    External["External Systems<br/>(Webhooks, APIs, Clients, Queues)"]

    %% Feed Phase: Bridge Module
    subgraph Bridge["hexafn-bridge (Feed)"]
        BridgeInfra["Infrastructure Layer<br/>(Webhook Endpoints, SDKs, API Clients)"]
        BridgeApp["Application Layer<br/>(IntegrationManagementPort, WebhookReceiverPort)"]
        BridgeDomain["Domain Layer<br/>(Integration, Webhook, EventTransformer)"]
    end

    %% Filter Phase: Trigger Module
    subgraph Trigger["hexafn-trigger (Filter)"]
        TriggerInfra["Infrastructure Layer<br/>(EventSubscriber, ConfigLoader)"]
        TriggerApp["Application Layer<br/>(TriggerRegistryPort, TriggerEvaluationPort)"]
        TriggerDomain["Domain Layer<br/>(Trigger, TriggerCondition, CompoundType)"]
    end

    %% Format & Function Phase: Run Module
    subgraph Run["hexafn-run (Format & Function)"]
        RunInfra["Infrastructure Layer<br/>(ExecutionSubscriber, FunctionRepository)"]
        RunApp["Application Layer<br/>(FunctionRegistryPort, FunctionExecutionPort)"]
        RunDomain["Domain Layer<br/>(FunctionRuntime, FunctionContext, ExecutionResult)"]
    end

    %% Forward Phase: Store & Cast Modules
    subgraph Store["hexafn-store (Forward: KV Store)"]
        StoreInfra["Infrastructure Layer<br/>(KVStoreController, StoreRepository)"]
        StoreApp["Application Layer<br/>(KvStorePort, StoreService)"]
        StoreDomain["Domain Layer<br/>(KvStore, KeyValueEntry, Namespace)"]
    end

    subgraph Cast["hexafn-cast (Forward: Pub/Sub)"]
        CastInfra["Infrastructure Layer<br/>(Controllers, MessageBus)"]
        CastApp["Application Layer<br/>(PublisherPort, SubscriberPort)"]
        CastDomain["Domain Layer<br/>(Topic, Message, Subscriber)"]
    end

    %% Feedback Phase: Watch Module
    subgraph Watch["hexafn-watch (Feedback)"]
        WatchInfra["Infrastructure Layer<br/>(Endpoints, Exporters)"]
        WatchApp["Application Layer<br/>(LoggingPort, TracingPort, MetricsPort)"]
        WatchDomain["Domain Layer<br/>(Trace, Span, LogEntry, MetricPoint)"]
    end

    %% Core Module (Shared Kernel)
    subgraph Core["hexafn-core (Shared Kernel)"]
        CoreDomain["Domain Layer<br/>(DomainEvent, ValueObjects, Pipeline)"]
        CoreApp["Application Layer<br/>(6F Lifecycle Traits, Orchestrator)"]
        CoreInfra["Infrastructure Layer<br/>(EventBus, Config, CLI)"]
    end

    %% Data Flow Connections (6F Lifecycle)
    External --> BridgeInfra
    BridgeInfra --> BridgeApp --> BridgeDomain
    BridgeDomain --> TriggerInfra
    TriggerInfra --> TriggerApp --> TriggerDomain
    TriggerDomain --> RunInfra
    RunInfra --> RunApp --> RunDomain
    RunDomain --> StoreInfra
    RunDomain --> CastInfra
    StoreInfra --> StoreApp --> StoreDomain
    CastInfra --> CastApp --> CastDomain
    StoreDomain --> WatchInfra
    CastDomain --> WatchInfra
    WatchInfra --> WatchApp --> WatchDomain
    WatchDomain --> CoreInfra

    %% Feedback/Observability
    WatchDomain --> External

    %% Core Module Relationships
    CoreDomain -.-> BridgeDomain & TriggerDomain & RunDomain & StoreDomain & CastDomain & WatchDomain
    CoreApp -.-> BridgeApp & TriggerApp & RunApp & StoreApp & CastApp & WatchApp
    CoreInfra -.-> BridgeInfra & TriggerInfra & RunInfra & StoreInfra & CastInfra & WatchInfra

    %% Bidirectional/Orchestration
    CoreApp -.-> CoreInfra
    CoreInfra -.-> CoreApp
```

---

## 📑 Flow Explanation

- **External Systems** send events or requests to the system via the **Bridge** module (Feed phase).
- **Bridge** normalizes and ingests events, passing them to the **Trigger** module (Filter phase).
- **Trigger** evaluates conditions and rules, forwarding valid events to the **Run** module (Format & Function phases).
- **Run** executes user-defined logic/functions, producing results that are sent to **Store** (for persistence) and/or **Cast** (for pub/sub/event broadcasting) in the Forward phase.
- **Store** and **Cast** modules can both emit events or state changes to the **Watch** module (Feedback phase), which handles logging, tracing, and metrics.
- **Watch** provides observability and can export feedback to external monitoring systems.
- The **Core** module provides shared types, orchestration, and event bus infrastructure, supporting all other modules and ensuring architectural consistency.

This high-level diagram ensures clear separation of concerns, strict module boundaries, and a unidirectional data flow that follows the 6F Lifecycle Flow. For detailed component and interface relationships, see `DATA_FLOW_DETAIL.md`.