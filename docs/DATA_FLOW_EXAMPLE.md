<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📊 hexaFn Data Flow Example

This diagram illustrates the complete data flow through all hexaFn modules, organized according to the **6F Lifecycle Phases** and **Hexagonal Architecture layers**.

## 🔄 Complete Flow Diagram

```mermaid
flowchart TB
    %% Dış sistemler
    External[Dış Sistemler] --> BridgeInfra
    
    %% Bridge Modülü
    subgraph Bridge["hexafn-bridge (Feed Phase)"]
        BridgeInfra["Infrastructure Layer (Controller, Parser)"] --> BridgeApp
        BridgeApp["Application Layer (WebhookIngestionPort)"] --> BridgeDomain
        BridgeDomain["Domain Layer (WebhookEvent)"] --> BridgeOutInfra
        BridgeOutInfra["Infrastructure Layer (EventPublisher)"] --> TopicRouter
    end
    
    %% Trigger Modülü
    subgraph Trigger["hexafn-trigger (Filter Phase)"]
        TriggerInfra["Infrastructure Layer (EventSubscriber)"] --> TriggerApp
        TriggerApp["Application Layer (TriggerEvaluationPort)"] --> TriggerDomain
        TriggerDomain["Domain Layer (Trigger, Rules)"] --> TriggerOutInfra
        TriggerOutInfra["Infrastructure Layer (ExecutionRequestPublisher)"] --> ExecRequest
    end
    
    %% Run Modülü
    subgraph Run["hexafn-run (Format & Function Phases)"]
        RunInfra["Infrastructure Layer (ExecutionSubscriber)"] --> RunApp
        RunApp["Application Layer (FunctionExecutionPort)"] --> RunDomain
        RunDomain["Domain Layer (Function, Schema)"] --> RunOutInfra
        RunOutInfra["Infrastructure Layer (RuntimeImpls, ResultPublisher)"] --> Results
    end
    
    %% Forward Phase
    subgraph Forward["Forward Phase"]
        %% Store Modülü
        subgraph Store["hexafn-store"]
            StoreInfra["Infrastructure Layer (KVStoreController)"] --> StoreApp
            StoreApp["Application Layer (KVStorePort)"] --> StoreDomain
            StoreDomain["Domain Layer (KVPair, Key, Value)"] --> StoreOutInfra
            StoreOutInfra["Infrastructure Layer (StorageImpls)"] --> Storage
        end
        
        %% Cast Modülü
        subgraph Cast["hexafn-cast"]
            CastInfra["Infrastructure Layer (Controllers)"] --> CastApp
            CastApp["Application Layer (PublisherPort, SubscriberPort)"] --> CastDomain
            CastDomain["Domain Layer (Topic, Message, Subscriber)"] --> CastOutInfra
            CastOutInfra["Infrastructure Layer (MessageBus)"] --> Subscribers
        end
    end
    
    %% Watch Modülü
    subgraph Watch["hexafn-watch (Feedback Phase)"]
        WatchInfra["Infrastructure Layer (Endpoints, Hooks)"] --> WatchApp
        WatchApp["Application Layer (LoggingPort, TracingPort, MetricsPort)"] --> WatchDomain
        WatchDomain["Domain Layer (Trace, Span, LogEntry, MetricPoint)"] --> WatchOutInfra
        WatchOutInfra["Infrastructure Layer (Exporters)"] --> Monitoring
    end
    
    %% Core Modülü
    subgraph Core["hexafn-core (Shared Kernel)"]
        CoreDomain["Domain Layer (DomainEvent, ValueObjects)"]
        CoreApp["Application Layer (6F Lifecycle Traits)"]
        CoreInfra["Infrastructure Layer (EventBus, Config)"]
    end
    
    %% Bağlantılar
    TopicRouter --> TriggerInfra
    ExecRequest --> RunInfra
    Results --> StoreInfra
    Results --> CastInfra
    Storage --> WatchInfra
    Subscribers --> WatchInfra
    Monitoring --> External
    
    %% Core bağlantıları
    CoreDomain -.-> BridgeDomain & TriggerDomain & RunDomain & StoreDomain & CastDomain & WatchDomain
    CoreApp -.-> BridgeApp & TriggerApp & RunApp & StoreApp & CastApp & WatchApp
    CoreInfra -.-> BridgeInfra & TriggerInfra & RunInfra & StoreInfra & CastInfra & WatchInfra
```

## 📑 Flow Explanation

This diagram shows how data flows through the complete hexaFn system following the **6F Lifecycle Flow**:

1. **Feed** (hexafn-bridge): External systems send data to the Bridge module, which ingests it
2. **Filter** (hexafn-trigger): The Trigger module evaluates conditions and rules on the data
3. **Format & Function** (hexafn-run): The Run module formats data and executes the appropriate function
4. **Forward** (hexafn-store, hexafn-cast): Results are forwarded to storage or messaging systems
5. **Feedback** (hexafn-watch): The Watch module provides observability and monitoring

The **hexafn-core** module acts as a shared kernel, providing common functionality to all other modules.

Each module follows **Hexagonal Architecture** with clearly separated:
- Infrastructure Layer (adapters for external systems)
- Application Layer (use cases and ports)
- Domain Layer (business logic and entities)

This architecture ensures that business logic is decoupled from external concerns and can be tested independently.