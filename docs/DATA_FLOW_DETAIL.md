<!-- SPDX-FileCopyrightText: 2025 Husamettin ARABACI -->
<!-- SPDX-License-Identifier: MIT -->

# hexaFn Detailed Component Architecture Diagram

This document provides a detailed, module- and layer-level architecture diagram for the hexaFn project. It strictly follows the 6F Lifecycle Flow (Feed → Filter → Format → Function → Forward → Feedback) and Hexagonal Architecture, and reflects all requirements and data models as defined in DATA_MODEL_*.md and TODO_LIST.md.

---

## 📐 Detailed Module & Layer Architecture

```mermaid
flowchart TB
    %% === External World (External Systems) ===
    External["External Systems\n(Webhooks, APIs, Clients, Queues)"]
    ExternalOut["External Outputs\n(Monitoring, Webhooks, APIs)"]

    %% === BRIDGE (Feed) ===
    subgraph Bridge["hexafn-bridge (Feed)"]
        subgraph BridgeInfra["Infrastructure"]
            WebhookEndpoint["WebhookEndpoint\n(REST, SDK, API)"]
            WebhookParser["WebhookParser"]
            TokenValidator["TokenValidator"]
        end
        subgraph BridgeApp["Application"]
            IntegrationMgmtPort["IntegrationManagementPort"]
            WebhookReceiverPort["WebhookReceiverPort"]
            BridgeService["BridgeService"]
        end
        subgraph BridgeDomain["Domain"]
            Integration["Integration"]
            Webhook["Webhook"]
            EventTransformer["EventTransformer"]
            BridgeEvent["BridgeEvent"]
        end
    end

    %% === TRIGGER (Filter) ===
    subgraph Trigger["hexafn-trigger (Filter)"]
        subgraph TriggerInfra["Infrastructure"]
            EventSubscriber["EventSubscriber"]
            ConfigLoader["ConfigLoader"]
        end
        subgraph TriggerApp["Application"]
            TriggerRegistryPort["TriggerRegistryPort"]
            TriggerEvaluationPort["TriggerEvaluationPort"]
            TriggerService["TriggerService"]
        end
        subgraph TriggerDomain["Domain"]
            TriggerEntity["Trigger"]
            TriggerCondition["TriggerCondition"]
            CompoundType["CompoundType"]
            TriggerEvent["TriggerEvent"]
        end
    end

    %% === RUN (Format + Function) ===
    subgraph Run["hexafn-run (Format & Function)"]
        subgraph RunInfra["Infrastructure"]
            ExecutionSubscriber["ExecutionSubscriber"]
            FunctionRepository["FunctionRepository"]
        end
        subgraph RunApp["Application"]
            FunctionRegistryPort["FunctionRegistryPort"]
            FunctionExecutionPort["FunctionExecutionPort"]
            RunService["RunService"]
        end
        subgraph RunDomain["Domain"]
            FunctionRuntime["FunctionRuntime"]
            FunctionContext["FunctionContext"]
            ExecutionResult["ExecutionResult"]
        end
    end

    %% === STORE (Forward: KV) ===
    subgraph Store["hexafn-store (Forward: KV Store)"]
        subgraph StoreInfra["Infrastructure"]
            KVStoreController["KVStoreController"]
            StoreRepository["StoreRepository"]
        end
        subgraph StoreApp["Application"]
            KvStorePort["KvStorePort"]
            StoreService["StoreService"]
        end
        subgraph StoreDomain["Domain"]
            KvStore["KvStore"]
            KeyValueEntry["KeyValueEntry"]
            Namespace["Namespace"]
        end
    end

    %% === CAST (Forward: Pub/Sub) ===
    subgraph Cast["hexafn-cast (Forward: Pub/Sub)"]
        subgraph CastInfra["Infrastructure"]
            MessageBus["MessageBus"]
            CastRepository["CastRepository"]
        end
        subgraph CastApp["Application"]
            TopicManagementPort["TopicManagementPort"]
            PublisherPort["PublisherPort"]
            SubscriberPort["SubscriberPort"]
        end
        subgraph CastDomain["Domain"]
            Topic["Topic"]
            Message["Message"]
            Subscriber["Subscriber"]
        end
    end

    %% === WATCH (Feedback) ===
    subgraph Watch["hexafn-watch (Feedback)"]
        subgraph WatchInfra["Infrastructure"]
            LogExporter["LogExporter"]
            TraceExporter["TraceExporter"]
            MetricsExporter["MetricsExporter"]
        end
        subgraph WatchApp["Application"]
            LoggingPort["LoggingPort"]
            TracingPort["TracingPort"]
            MetricsPort["MetricsPort"]
            WatchService["WatchService"]
        end
        subgraph WatchDomain["Domain"]
            Trace["Trace"]
            Span["Span"]
            LogEntry["LogEntry"]
            MetricPoint["MetricPoint"]
        end
    end

    %% === CORE (Shared Kernel) ===
    subgraph Core["hexafn-core (Shared Kernel)"]
        subgraph CoreInfra["Infrastructure"]
            EventBus["EventBus"]
            Config["Config"]
            CLI["CLI"]
        end
        subgraph CoreApp["Application"]
            Orchestrator["Orchestrator"]
            PipelineBuilder["PipelineBuilder"]
        end
        subgraph CoreDomain["Domain"]
            DomainEvent["DomainEvent"]
            ValueObjects["ValueObjects"]
            Pipeline["Pipeline"]
        end
    end

    %% === Data Flow Connections (6F Lifecycle) ===
    External --> WebhookEndpoint
    WebhookEndpoint --> WebhookParser --> TokenValidator --> WebhookReceiverPort
    WebhookReceiverPort --> BridgeService --> EventTransformer
    EventTransformer --> BridgeEvent --> EventSubscriber
    EventSubscriber --> TriggerRegistryPort
    TriggerRegistryPort --> TriggerEvaluationPort --> TriggerService
    TriggerService --> TriggerEntity --> TriggerCondition
    TriggerService --> ExecutionSubscriber
    ExecutionSubscriber --> FunctionRegistryPort
    FunctionRegistryPort --> FunctionExecutionPort --> RunService
    RunService --> FunctionRuntime
    RunService --> FunctionContext
    FunctionRuntime --> ExecutionResult
    ExecutionResult --> KVStoreController
    ExecutionResult --> MessageBus
    KVStoreController --> KvStorePort --> StoreService --> KvStore
    MessageBus --> TopicManagementPort --> PublisherPort --> SubscriberPort
    PublisherPort --> Topic
    SubscriberPort --> Subscriber
    Topic --> Message
    Message --> LogExporter
    LogExporter --> LoggingPort --> WatchService --> Trace
    Trace --> Span
    WatchService --> MetricPoint
    MetricPoint --> MetricsExporter
    Trace --> TraceExporter
    LoggingPort --> LogEntry
    LogEntry --> LogExporter
    MetricsPort --> MetricPoint
    Orchestrator -.-> PipelineBuilder
    PipelineBuilder -.-> Pipeline
    EventBus -.-> EventSubscriber
    EventBus -.-> MessageBus
    EventBus -.-> LogExporter
    EventBus -.-> TraceExporter
    EventBus -.-> MetricsExporter
    CLI -.-> Orchestrator
    Config -.-> Orchestrator
    Pipeline -.-> DomainEvent
    ValueObjects -.-> Pipeline
    
    %% === Feedback and External Output ===
    LogExporter --> ExternalOut
    TraceExporter --> ExternalOut
    MetricsExporter --> ExternalOut
```

---

## 📝 Explanation

- **Each module** (Bridge, Trigger, Run, Store, Cast, Watch, Core) is shown with its Infrastructure, Application, and Domain layers, as required by Hexagonal Architecture.
- **6F Lifecycle Flow** is strictly followed: Feed (Bridge) → Filter (Trigger) → Format/Function (Run) → Forward (Store/Cast) → Feedback (Watch).
- **Data flow** is unidirectional and modular, with clear boundaries and port/adaptor interfaces between layers.
- **Core module** provides shared types, orchestration, event bus, and pipeline builder, supporting all modules.
- **External systems** interact via Bridge (input) and Watch (output/feedback).
- **All major data structures, ports, and flows** are represented, matching the comprehensive data models and TODO_LIST.md requirements.
- **This diagram and explanation are designed to be exhaustive and production-ready, suitable for onboarding, architecture review, and implementation reference.**