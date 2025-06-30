<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📦 hexaFn Data Lifecycle (Vertical ASCII Flow)

**Scenario:**
A value is published to an MQTT topic. The Bridge module receives the message via MQTT, converts it to an event, and dispatches it. The Trigger module checks if the value is a number. If it is, the Run module calculates its square. The Store module saves the result as a score in the KV store. The Cast module publishes the score to users. The Watch module logs the entire operation.

This diagram below shows the exact port, adapter, and service names for this scenario:

```
┌───────┐   ┌────────────────────────────┐◀─────────────────────────────────────────────────────┌───────┐
│   C   │   │ Bridge Infra: Listener     │                                                      │   E   │
│   o   │   │   MqttMessageListener      │                                                      │   x   │
│   r   │   └─────────┬──────────────────┘                                                      │   t   │
│   e   │             │                                                                         │   e   │
│       │             ▼                                                                         │   r   │
│   │   │   ┌────────────────────────────┐                                                      │   n   │
│   │   │   │ Bridge Infra: P.Adapter    │                                                      │   a   │
│   ▼   │   │   MqttToEventAdapter       │                                                      │   l   │
│       │   └─────────┬──────────────────┘                                                      │       │
│   E   │             │                                                                         │   S   │
│   v   │             ▼                                                                         │   y   │
│   e   │   ┌────────────────────────────┐                                                      │   s   │
│   n   │   │ Bridge App: P.Port         │                                                      │   t   │
│   t   │   │   IncomingEventPort        │                                                      │   e   │
│       │   └─────────┬──────────────────┘                                                      │   m   │
│   M   │             │                                                                         │       │
│   a   │             ▼                                                                         │       │
│   n   │   ┌────────────────────────────┐      ┌────────────────────────────┐                  │       │
│   a   │   │ Bridge App:                │◀────▶│ Bridge Domain:             │                  │       │
│   g   │   │   EventIngestService       │      │   RawEvent                 │                  │       │
│   e   │   └─────────┬──────────────────┘      └────────────────────────────┘                  │       │
│   r   │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Bridge App: S.Port         │                                                      │       │
│       │   │   EventDispatchPort        │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Bridge Infra: S.Adapter    │                                                      │       │
│       │   │   EventBusPublisherAdapter │                                                      │       │
│       │◀──└────────────────────────────┘                                                      │       │
│       │                                                                                       │       │
│       │                                                                                       │       │
│       │──▶┌────────────────────────────┐                                                      │       │
│       │   │ Trigger Infra: Listener    │                                                      │       │
│       │   │   EventBusSubscriber       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Trigger Infra: P.Adapter   │                                                      │       │
│       │   │   EventToRuleCheckAdapter  │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│   C   │   │ Trigger App: P.Port        │                                                      │   E   │
│   o   │   │   RuleCheckPort            │                                                      │   x   │
│   r   │   └─────────┬──────────────────┘                                                      │   t   │
│   e   │             │                                                                         │   e   │
│       │             ▼                                                                         │   r   │
│   │   │   ┌────────────────────────────┐      ┌────────────────────────────┐                  │   n   │
│   │   │   │ Trigger App:               │◀────▶│ Trigger Domain:            │                  │   a   │
│   ▼   │   │   RuleCheckService         │      │   NumberRuleCheck          │                  │   l   │
│       │   └─────────┬──────────────────┘      └────────────────────────────┘                  │       │
│   E   │             │                                                                         │   S   │
│   v   │             ▼                                                                         │   y   │
│   e   │   ┌────────────────────────────┐                                                      │   s   │
│   n   │   │ Trigger App: S.Port        │                                                      │   t   │
│   t   │   │   RuleCheckResultPort      │                                                      │   e   │
│       │   └─────────┬──────────────────┘                                                      │   m   │
│   M   │             │                                                                         │       │
│   a   │             ▼                                                                         │       │
│   n   │   ┌────────────────────────────┐                                                      │       │
│   a   │   │ Trigger Infra: S.Adapter   │                                                      │       │
│   g   │   │   EventBusPublisherAdapter │                                                      │       │
│   e   │◀──└────────────────────────────┘                                                      │       │
│   r   │                                                                                       │       │
│       │                                                                                       │       │
│       │──▶┌────────────────────────────┐                                                      │       │
│       │   │ Run Infra: Listener        │                                                      │       │
│       │   │   EventBusSubscriber       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Run Infra: P.Adapter       │                                                      │       │
│       │   │   EventToFunctionAdapter   │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Run App: P.Port            │                                                      │       │
│       │   │   FunctionPort             │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐      ┌────────────────────────────┐                  │       │
│       │   │ Run App:                   │◀────▶│ Run Domain:                │                  │       │
│       │   │   FunctionService          │      │   SquareFunction           │                  │       │
│       │   └─────────┬──────────────────┘      └────────────────────────────┘                  │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Run App: S.Port            │                                                      │       │
│       │   │   FunctionResultPort       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Run Infra: S.Adapter       │                                                      │       │
│       │   │   EventBusPublisherAdapter │                                                      │       │
│       │◀──└────────────────────────────┘                                                      │       │
│       │                                                                                       │       │
│       │                                                                                       │       │
│       │──▶┌────────────────────────────┐                                                      │       │
│       │   │ Store Infra: Listener      │                                                      │       │
│       │   │   EventBusSubscriber       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Store Infra: P.Adapter     │                                                      │       │
│       │   │   DataToKvAdapter          │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│   C   │   │ Store App: P.Port          │                                                      │   E   │
│   o   │   │   DataKvPort               │                                                      │   x   │
│   r   │   └─────────┬──────────────────┘                                                      │   t   │
│   e   │             │                                                                         │   e   │
│       │             ▼                                                                         │   r   │
│   │   │   ┌────────────────────────────┐      ┌────────────────────────────┐                  │   n   │
│   │   │   │ Store App:                 │◀────▶│ Store Domain:              │                  │   a   │
│   ▼   │   │   DataStoreService         │      │   ScoreDataEntry           │                  │   l   │
│       │   └─────────┬──────────────────┘      └────────────────────────────┘                  │       │
│   E   │             │                                                                         │   S   │
│   v   │             ▼                                                                         │   y   │
│   e   │   ┌────────────────────────────┐                                                      │   s   │
│   n   │   │ Store App: S.Port          │                                                      │   t   │
│   t   │   │   DataStoredPort           │                                                      │   e   │
│       │   └─────────┬──────────────────┘─────────────────────────────────────────────────────▶│   m   │
│   M   │             │                                                                         │       │
│   a   │             ▼                                                                         │       │
│   n   │   ┌────────────────────────────┐                                                      │       │
│   a   │   │ Store Infra: S.Adapter     │                                                      │       │
│   g   │   │   EventBusPublisherAdapter │                                                      │       │
│   e   │◀──└────────────────────────────┘                                                      │       │
│   r   │                                                                                       │       │
│       │                                                                                       │       │
│       │──▶┌────────────────────────────┐                                                      │       │
│       │   │ Cast Infra: Listener       │                                                      │       │
│       │   │   EventBusSubscriber       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Cast Infra: P.Adapter      │                                                      │       │
│       │   │   EventToPublishAdapter    │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐      ┌─────────────────────────────┐                 │       │
│       │   │ Cast App:                  │◀────▶│ Cast Domain:                │                 │       │
│       │   │   PublishService           │      │   ScoreMessage              │                 │       │
│       │   └─────────┬──────────────────┘      └─────────────────────────────┘                 │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Cast App: S.Port           │                                                      │       │
│       │   │   PublishedPort            │                                                      │       │
│       │   └─────────┬──────────────────┘─────────────────────────────────────────────────────▶│       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Cast Infra: S.Adapter      │                                                      │       │
│       │   │   EventToPublishAdapter    │                                                      │       │
│       │◀──└────────────────────────────┘                                                      │       │
│       │                                                                                       │       │
│       │                                                                                       │       │
│       │──▶┌────────────────────────────┐                                                      │       │
│       │   │ Watch Infra: Listener      │                                                      │       │
│       │   │   EventBusSubscriber       │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│       │             ▼                                                                         │       │
│       │   ┌────────────────────────────┐                                                      │       │
│       │   │ Watch Infra: P.Adapter     │                                                      │       │
│       │   │   EventToLogAdapter        │                                                      │       │
│       │   └─────────┬──────────────────┘                                                      │       │
│       │             │                                                                         │       │
│   C   │             ▼                                                                         │   E   │
│   o   │   ┌────────────────────────────┐                                                      │   x   │
│   r   │   │ Watch App: P.Port          │                                                      │   t   │
│   e   │   │   LogPort                  │                                                      │   e   │
│       │   └─────────┬──────────────────┘                                                      │   r   │
│   │   │             │                                                                         │   n   │
│   │   │             ▼                                                                         │   a   │
│   ▼   │   ┌────────────────────────────┐      ┌────────────────────────────┐                  │   l   │
│       │   │ Watch  App:                │◀────▶│ Watch Domain:              │                  │       │
│   E   │   │   LogService               │      │   LogEntry                 │                  │   S   │
│   v   │   └─────────┬──────────────────┘      └────────────────────────────┘                  │   y   │
│   e   │             │                                                                         │   s   │
│   n   │             ▼                                                                         │   t   │
│   t   │   ┌────────────────────────────┐                                                      │   e   │
│       │   │ Watch App: S.Port          │                                                      │   m   │
│   M   │   │   LogWrittenPort           │                                                      │       │
│   a   │   └─────────┬──────────────────┘                                                      │       │
│   n   │             │                                                                         │       │
│   a   │             ▼                                                                         │       │
│   g   │   ┌────────────────────────────┐                                                      │       │
│   e   │   │ Watch Infra: S.Adapter     │                                                      │       │
│   r   │   │   LogToExternalAdapter     │                                                      │       │
└───────┘   └────────────────────────────┘─────────────────────────────────────────────────────▶└───────┘      
```

---

## 🔄 Data Lifecycle in a Module: DDD & Hexagonal Architecture Perspective

Below is a step-by-step summary of the journey of data through the **Store (KV Store)** module, from entry to propagation to other modules or external systems, following DDD, Hexagonal Architecture, and Clean Code principles. This model applies to all hexaFn modules.

1. **Input: Controller or Listener (Infrastructure Layer)**
   - Data from the outside world (e.g., HTTP API, MQTT, EventBus) is received by a *controller* or *listener* in the module's Infrastructure layer (e.g., `KVStoreController`, `EventBusSubscriber`).
   - This layer isolates external protocols and framework dependencies.

2. **Primary Adapter: Application Port Implementation (Infrastructure Layer)**
   - The received data is converted by an *adapter* into the interface of a *primary port* in the Application layer (e.g., `KvStorePort`).
   - The adapter transforms the external data format into the internal model and passes it according to the port contract.
   - This ensures the Application layer remains independent from the outside world.

3. **Application Service: Workflow and Orchestration (Application Layer)**
   - The Application Service (e.g., `StoreService`) receives the request via the port and initiates the workflow.
   - Validation, business rules, and orchestration are handled here.
   - The service delegates to domain services or entities/value objects as needed for business logic.

4. **Domain Service or Entity: Business Logic (Domain Layer)**
   - The Domain Service (e.g., `KvStore`) applies pure business logic and rules.
   - Entities or Value Objects ensure data correctness and integrity.
   - This layer is completely isolated and pure, with no external dependencies.

5. **Secondary Port: Dependency Inversion (Application Layer)**
   - If the business logic requires access to an external resource (e.g., database, event bus, another service), the Application Service calls a *secondary port* (e.g., `DataStoredPort`).
   - The port is just an interface; its implementation is in the infrastructure layer.
   - This applies the Dependency Inversion (IOC) principle: the Application layer manages external dependencies via interfaces.

6. **Secondary Adapter: Communication with External Resource (Infrastructure Layer)**
   - The adapter implementing the secondary port (e.g., `EventBusPublisherAdapter`) sends the data to the external resource (e.g., Core Event Manager, another module, external service).
   - This adapter handles the actual communication and shields the Application layer from external dependencies.

7. **Inter-Module via Core Event Manager**
   - If the data needs to be propagated to other modules, it is published via the Core module's Event Manager (EventBus/EventListener).
   - Other modules can subscribe to these events and start their own workflows.

---

### **Advantages of This Flow**
- **Loose Coupling:** Each layer depends only on the interfaces of the layer above; implementation details are hidden in infrastructure.
- **IOC (Inversion of Control):** The Application layer manages external dependencies via ports/adapters; dependencies are injected from outside.
- **Testability:** Domain and Application layers are isolated from the outside world, making them easy to test.
- **Extensibility:** New adapters or ports can be added without affecting existing business logic.
- **Clean Code:** Each layer has a clear responsibility, making the code readable and maintainable.

---

> This model summarizes how data flow is managed in all hexaFn modules, ensuring loose coupling and clean code through DDD and Hexagonal Architecture principles.
