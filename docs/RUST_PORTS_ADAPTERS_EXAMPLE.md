<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Rust Ports & Adapters Components Documentation

This document provides a comprehensive, table-based overview of all ports, adapters, and domain components implemented in hexaFn's Hexagonal Architecture pattern. Each table maps to a specific module and its corresponding architecture layers.

---

## 🔵 Core Module (hexafn-core)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Pipeline` | Central domain entity representing the 6F Lifecycle Flow | `PipelineStage`, `PipelineContext` |
| Entity | `Event` | Domain event representation with metadata | `EventMetadata`, `EventPayload` |
| Value Object | `PipelineStage` | Represents a specific stage in the 6F lifecycle | `Pipeline` |
| Value Object | `EventMetadata` | Immutable metadata about an event | `Event` |
| Value Object | `EventId` | Unique identifier for events | `Event` |
| Domain Service | `PipelineExecutor` | Handles the execution flow of pipeline stages | `Pipeline`, `PipelineStage` |
| Domain Event | `PipelineStageCompletedEvent` | Signals completion of a pipeline stage | `PipelineStage` |
| Domain Event | `PipelineCompletedEvent` | Signals the full pipeline has completed | `Pipeline` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `PipelineControlPort` | Interface for pipeline lifecycle management | `PipelineService` |
| Port (Input) | `EventProcessingPort` | Interface for event submission and processing | `EventProcessingService` |
| Port (Output) | `EventStorePort` | Interface for event persistence | `EventRepository` |
| Port (Output) | `EventPublisherPort` | Interface for event publishing | `EventPublisher` |
| Service | `PipelineService` | Orchestrates pipeline creation and execution | `Pipeline`, `PipelineExecutor` |
| Service | `EventProcessingService` | Handles event routing and processing | `Event`, `EventPublisherPort` |
| Command | `CreatePipelineCommand` | Command to create a new pipeline | `PipelineService` |
| Command | `ExecutePipelineCommand` | Command to execute a pipeline | `PipelineService` |
| Query | `GetPipelineStatusQuery` | Query for pipeline execution status | `PipelineService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `PipelineControllerAdapter` | Adapts external requests to pipeline control | `PipelineControlPort` |
| Adapter (Primary) | `EventReceiverAdapter` | Adapts incoming events to the event processing system | `EventProcessingPort` |
| Adapter (Secondary) | `InMemoryEventStore` | In-memory implementation of event storage | `EventStorePort` |
| Adapter (Secondary) | `EventPublisherAdapter` | Implementation of event publishing | `EventPublisherPort` |
| Mapper | `PipelineDtoMapper` | Maps between Pipeline entities and DTOs | `Pipeline`, `PipelineDto` |
| Mapper | `EventDtoMapper` | Maps between Event entities and DTOs | `Event`, `EventDto` |
| DTO | `PipelineDto` | Data transfer object for pipelines | - |
| DTO | `EventDto` | Data transfer object for events | - |

---

## 🔄 Trigger Module (hexafn-trigger)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Trigger` | Core entity representing a triggerable condition | `TriggerCondition`, `TriggerAction` |
| Entity | `TriggerExecution` | Records the execution of a trigger | `Trigger`, `ExecutionResult` |
| Value Object | `TriggerCondition` | Encapsulates the logic for when a trigger fires | `Trigger` |
| Value Object | `TriggerName` | Identifies a trigger uniquely | `Trigger` |
| Value Object | `TriggerPriority` | Defines the priority of trigger evaluation | `Trigger` |
| Domain Service | `TriggerEvaluator` | Core business logic for evaluating trigger conditions | `Trigger`, `TriggerCondition` |
| Domain Event | `TriggerCreatedEvent` | Signals a new trigger was created | `Trigger` |
| Domain Event | `TriggerFiredEvent` | Signals a trigger condition was met | `Trigger`, `TriggerExecution` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `TriggerManagementPort` | Interface for trigger CRUD operations | `TriggerManagementService` |
| Port (Input) | `TriggerEvaluationPort` | Interface for evaluating triggers against events | `TriggerEvaluationService` |
| Port (Output) | `TriggerRepositoryPort` | Interface for trigger persistence | `TriggerRepository` |
| Port (Output) | `TriggerEventPublisherPort` | Interface for publishing trigger events | `TriggerEventPublisher` |
| Service | `TriggerManagementService` | Handles trigger creation, update, deletion | `Trigger`, `TriggerRepositoryPort` |
| Service | `TriggerEvaluationService` | Orchestrates trigger evaluation against incoming events | `TriggerEvaluator`, `TriggerRepositoryPort` |
| Command | `CreateTriggerCommand` | Command to create a new trigger | `TriggerManagementService` |
| Command | `UpdateTriggerCommand` | Command to update an existing trigger | `TriggerManagementService` |
| Query | `GetTriggersQuery` | Query to retrieve triggers | `TriggerManagementService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `TriggerApiAdapter` | Adapts HTTP/API requests to trigger management | `TriggerManagementPort` |
| Adapter (Primary) | `TriggerEventConsumerAdapter` | Consumes events for trigger evaluation | `TriggerEvaluationPort` |
| Adapter (Secondary) | `TriggerDatabaseRepository` | Persists triggers to database | `TriggerRepositoryPort` |
| Adapter (Secondary) | `TriggerEventPublisherImpl` | Publishes trigger events to event bus | `TriggerEventPublisherPort` |
| Mapper | `TriggerDtoMapper` | Maps between Trigger entities and DTOs | `Trigger`, `TriggerDto` |
| Mapper | `TriggerConditionMapper` | Maps between different condition formats | `TriggerCondition`, `TriggerConditionDto` |
| DTO | `TriggerDto` | Data transfer object for triggers | - |
| DTO | `TriggerConditionDto` | Data transfer object for trigger conditions | - |

---

## ⚡ Run Module (hexafn-run)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Function` | Core entity representing executable logic | `FunctionCode`, `FunctionConfig` |
| Entity | `FunctionExecution` | Records function execution with inputs/outputs | `Function`, `ExecutionResult` |
| Value Object | `FunctionCode` | Contains the executable code of a function | `Function` |
| Value Object | `FunctionConfig` | Configuration parameters for function execution | `Function` |
| Value Object | `ExecutionResult` | Output and status of a function execution | `FunctionExecution` |
| Domain Service | `FunctionExecutor` | Core business logic for function execution | `Function`, `Runtime` |
| Domain Event | `FunctionRegisteredEvent` | Signals a new function was registered | `Function` |
| Domain Event | `FunctionExecutedEvent` | Signals a function was executed | `FunctionExecution` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `FunctionRegistryPort` | Interface for registering functions | `FunctionRegistryService` |
| Port (Input) | `FunctionExecutionPort` | Interface for executing functions | `FunctionExecutionService` |
| Port (Output) | `FunctionRepositoryPort` | Interface for function persistence | `FunctionRepository` |
| Port (Output) | `RuntimeProviderPort` | Interface for runtime management | `RuntimeProvider` |
| Service | `FunctionRegistryService` | Manages function registration and discovery | `Function`, `FunctionRepositoryPort` |
| Service | `FunctionExecutionService` | Orchestrates function execution | `FunctionExecutor`, `RuntimeProviderPort` |
| Command | `RegisterFunctionCommand` | Command to register a new function | `FunctionRegistryService` |
| Command | `ExecuteFunctionCommand` | Command to execute a function | `FunctionExecutionService` |
| Query | `GetFunctionQuery` | Query to retrieve a function | `FunctionRegistryService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `FunctionApiAdapter` | Adapts HTTP/API requests to function operations | `FunctionRegistryPort`, `FunctionExecutionPort` |
| Adapter (Primary) | `FunctionEventConsumerAdapter` | Consumes events for function execution | `FunctionExecutionPort` |
| Adapter (Secondary) | `FunctionDatabaseRepository` | Persists functions to database | `FunctionRepositoryPort` |
| Adapter (Secondary) | `WasmRuntimeAdapter` | WebAssembly runtime implementation | `RuntimeProviderPort` |
| Adapter (Secondary) | `JsRuntimeAdapter` | JavaScript runtime implementation | `RuntimeProviderPort` |
| Adapter (Secondary) | `LuaRuntimeAdapter` | Lua runtime implementation | `RuntimeProviderPort` |
| Adapter (Secondary) | `DslRuntimeAdapter` | Internal DSL runtime implementation | `RuntimeProviderPort` |
| Mapper | `FunctionDtoMapper` | Maps between Function entities and DTOs | `Function`, `FunctionDto` |
| DTO | `FunctionDto` | Data transfer object for functions | - |
| DTO | `ExecutionRequestDto` | DTO for function execution requests | - |

---

## 📦 Store Module (hexafn-store)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `KeyValueEntry` | Core entity representing a stored KV pair | `Key`, `Value`, `Metadata` |
| Entity | `Namespace` | Logical grouping of related KV entries | `KeyValueEntry` |
| Value Object | `Key` | Identifier for a stored value | `KeyValueEntry` |
| Value Object | `Value` | Content of a stored entry | `KeyValueEntry` |
| Value Object | `Metadata` | Additional data about a KV entry | `KeyValueEntry` |
| Domain Service | `EntryValidator` | Validates entries against schemas | `KeyValueEntry`, `Schema` |
| Domain Event | `EntryCreatedEvent` | Signals a new KV entry was created | `KeyValueEntry` |
| Domain Event | `EntryUpdatedEvent` | Signals a KV entry was updated | `KeyValueEntry` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `KvStorePort` | Interface for KV CRUD operations | `KvStoreService` |
| Port (Input) | `NamespaceManagementPort` | Interface for namespace operations | `NamespaceService` |
| Port (Output) | `KvStoragePort` | Interface for KV backend storage | `KvStorageAdapter` |
| Port (Output) | `SchemaValidatorPort` | Interface for schema validation | `SchemaValidator` |
| Service | `KvStoreService` | Manages KV operations and validation | `EntryValidator`, `KvStoragePort` |
| Service | `NamespaceService` | Manages namespaces | `Namespace`, `KvStoragePort` |
| Command | `StoreEntryCommand` | Command to store a KV entry | `KvStoreService` |
| Command | `DeleteEntryCommand` | Command to delete a KV entry | `KvStoreService` |
| Query | `GetEntryQuery` | Query to retrieve a KV entry | `KvStoreService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `KvApiAdapter` | Adapts HTTP/API requests to KV operations | `KvStorePort` |
| Adapter (Primary) | `KvCliAdapter` | CLI interface for KV operations | `KvStorePort` |
| Adapter (Secondary) | `InMemoryKvAdapter` | In-memory implementation of KV storage | `KvStoragePort` |
| Adapter (Secondary) | `FileKvAdapter` | File-based implementation of KV storage | `KvStoragePort` |
| Adapter (Secondary) | `RocksDbKvAdapter` | RocksDB implementation of KV storage | `KvStoragePort` |
| Adapter (Secondary) | `JsonSchemaValidator` | JSON Schema validation implementation | `SchemaValidatorPort` |
| Mapper | `KvEntryDtoMapper` | Maps between KV entities and DTOs | `KeyValueEntry`, `KvEntryDto` |
| DTO | `KvEntryDto` | Data transfer object for KV entries | - |
| DTO | `NamespaceDto` | Data transfer object for namespaces | - |

---

## 📡 Cast Module (hexafn-cast)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Topic` | Core entity representing a message topic | `Subscription`, `Message` |
| Entity | `Subscription` | Represents a subscriber to a topic | `Topic`, `SubscriptionFilter` |
| Value Object | `Message` | Content and metadata of a topic message | `Topic` |
| Value Object | `SubscriptionFilter` | Filter criteria for topic messages | `Subscription` |
| Value Object | `TopicName` | Identifier for a topic | `Topic` |
| Domain Service | `MessageRouter` | Routes messages to subscriptions | `Topic`, `Subscription` |
| Domain Event | `TopicCreatedEvent` | Signals a new topic was created | `Topic` |
| Domain Event | `MessagePublishedEvent` | Signals a message was published | `Topic`, `Message` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `TopicManagementPort` | Interface for topic CRUD operations | `TopicService` |
| Port (Input) | `PublisherPort` | Interface for publishing messages | `PublisherService` |
| Port (Input) | `SubscriptionPort` | Interface for subscription management | `SubscriptionService` |
| Port (Output) | `MessageBusPort` | Interface for message delivery | `MessageBus` |
| Port (Output) | `TopicRepositoryPort` | Interface for topic persistence | `TopicRepository` |
| Service | `TopicService` | Manages topics and retention policies | `Topic`, `TopicRepositoryPort` |
| Service | `PublisherService` | Handles message publishing | `MessageRouter`, `MessageBusPort` |
| Service | `SubscriptionService` | Manages subscriptions | `Subscription`, `TopicRepositoryPort` |
| Command | `CreateTopicCommand` | Command to create a new topic | `TopicService` |
| Command | `PublishMessageCommand` | Command to publish a message | `PublisherService` |
| Query | `GetSubscriptionsQuery` | Query to retrieve subscriptions | `SubscriptionService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `CastApiAdapter` | Adapts HTTP/API requests to Cast operations | `TopicManagementPort`, `PublisherPort` |
| Adapter (Primary) | `CastCliAdapter` | CLI interface for Cast operations | `TopicManagementPort`, `SubscriptionPort` |
| Adapter (Secondary) | `InMemoryMessageBus` | In-memory implementation of message bus | `MessageBusPort` |
| Adapter (Secondary) | `TopicDatabaseRepository` | Persists topics and subscriptions | `TopicRepositoryPort` |
| Mapper | `TopicDtoMapper` | Maps between Topic entities and DTOs | `Topic`, `TopicDto` |
| Mapper | `MessageDtoMapper` | Maps between Message value objects and DTOs | `Message`, `MessageDto` |
| DTO | `TopicDto` | Data transfer object for topics | - |
| DTO | `MessageDto` | Data transfer object for messages | - |
| DTO | `SubscriptionDto` | Data transfer object for subscriptions | - |

---

## 🔍 Watch Module (hexafn-watch)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Trace` | Core entity representing a distributed trace | `Span`, `TraceContext` |
| Entity | `Span` | Individual operation within a trace | `Trace`, `SpanContext` |
| Value Object | `TraceContext` | Propagation context for a trace | `Trace` |
| Value Object | `SpanContext` | Context of an operation span | `Span` |
| Value Object | `LogEntry` | Structured log record | `Span` |
| Domain Service | `TracePropagator` | Manages trace context across boundaries | `Trace`, `TraceContext` |
| Domain Event | `SpanStartedEvent` | Signals a span was started | `Span` |
| Domain Event | `SpanFinishedEvent` | Signals a span was completed | `Span` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `TracingPort` | Interface for tracing operations | `TracingService` |
| Port (Input) | `LoggingPort` | Interface for structured logging | `LoggingService` |
| Port (Input) | `MetricsPort` | Interface for metrics collection | `MetricsService` |
| Port (Output) | `TraceExporterPort` | Interface for exporting traces | `TraceExporter` |
| Port (Output) | `LogExporterPort` | Interface for exporting logs | `LogExporter` |
| Port (Output) | `MetricsExporterPort` | Interface for exporting metrics | `MetricsExporter` |
| Service | `TracingService` | Manages trace lifecycle and spans | `Trace`, `TracePropagator` |
| Service | `LoggingService` | Manages structured logging | `LogEntry`, `LogExporterPort` |
| Service | `MetricsService` | Collects and exposes metrics | `MetricsExporterPort` |
| Command | `StartSpanCommand` | Command to start a new span | `TracingService` |
| Command | `LogEventCommand` | Command to log an event | `LoggingService` |
| Query | `GetTraceQuery` | Query to retrieve a trace | `TracingService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `TracingApiAdapter` | Adapts HTTP/API requests to tracing operations | `TracingPort` |
| Adapter (Primary) | `LoggingMacroAdapter` | Adapts logging macros to structured logging | `LoggingPort` |
| Adapter (Secondary) | `OpenTelemetryTraceExporter` | Exports traces to OpenTelemetry | `TraceExporterPort` |
| Adapter (Secondary) | `JsonLogExporter` | Exports logs as JSON lines | `LogExporterPort` |
| Adapter (Secondary) | `PrometheusMetricsExporter` | Exports metrics to Prometheus | `MetricsExporterPort` |
| Mapper | `TraceDtoMapper` | Maps between Trace entities and DTOs | `Trace`, `TraceDto` |
| Mapper | `LogEntryDtoMapper` | Maps between LogEntry value objects and DTOs | `LogEntry`, `LogEntryDto` |
| DTO | `TraceDto` | Data transfer object for traces | - |
| DTO | `SpanDto` | Data transfer object for spans | - |
| DTO | `LogEntryDto` | Data transfer object for log entries | - |

---

## 🌐 Bridge Module (hexafn-bridge)

### Domain Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Entity | `Integration` | Core entity representing an external integration | `IntegrationType`, `IntegrationConfig` |
| Entity | `Webhook` | Represents a webhook endpoint integration | `Integration`, `WebhookConfig` |
| Value Object | `IntegrationType` | Type of external integration | `Integration` |
| Value Object | `IntegrationConfig` | Configuration for an integration | `Integration` |
| Value Object | `WebhookConfig` | Configuration specific to webhooks | `Webhook` |
| Domain Service | `PayloadNormalizer` | Normalizes external data formats | `Integration`, `Event` |
| Domain Event | `IntegrationRegisteredEvent` | Signals a new integration was registered | `Integration` |
| Domain Event | `WebhookReceivedEvent` | Signals data was received via webhook | `Webhook` |

### Application Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Port (Input) | `IntegrationManagementPort` | Interface for integration CRUD operations | `IntegrationService` |
| Port (Input) | `WebhookReceiverPort` | Interface for receiving webhook data | `WebhookService` |
| Port (Output) | `IntegrationRepositoryPort` | Interface for integration persistence | `IntegrationRepository` |
| Port (Output) | `EventDispatcherPort` | Interface for dispatching normalized events | `EventDispatcher` |
| Service | `IntegrationService` | Manages external integrations | `Integration`, `IntegrationRepositoryPort` |
| Service | `WebhookService` | Handles webhook data reception | `Webhook`, `PayloadNormalizer` |
| Command | `RegisterIntegrationCommand` | Command to register a new integration | `IntegrationService` |
| Command | `ProcessWebhookCommand` | Command to process webhook data | `WebhookService` |
| Query | `GetIntegrationsQuery` | Query to retrieve integrations | `IntegrationService` |

### Infrastructure Layer

| Component Type | Name | Description | Related Components |
|----------------|------|-------------|-------------------|
| Adapter (Primary) | `IntegrationApiAdapter` | Adapts HTTP/API requests to integration management | `IntegrationManagementPort` |
| Adapter (Primary) | `WebhookHttpAdapter` | HTTP endpoint for receiving webhooks | `WebhookReceiverPort` |
| Adapter (Secondary) | `IntegrationDatabaseRepository` | Persists integrations to database | `IntegrationRepositoryPort` |
| Adapter (Secondary) | `EventDispatcherAdapter` | Dispatches normalized events to Cast | `EventDispatcherPort` |
| Mapper | `IntegrationDtoMapper` | Maps between Integration entities and DTOs | `Integration`, `IntegrationDto` |
| Mapper | `WebhookPayloadMapper` | Maps webhook payloads to normalized format | `WebhookConfig`, `Event` |
| DTO | `IntegrationDto` | Data transfer object for integrations | - |
| DTO | `WebhookDto` | Data transfer object for webhooks | - |
| DTO | `WebhookPayloadDto` | Data transfer object for webhook payloads | - |

---

## 📚 References

- Hexagonal Architecture (Ports & Adapters): [HEXAGONAL_ARCHITECTURE_GUIDE.md](HEXAGONAL_ARCHITECTURE_GUIDE.md)
- Module Structure: [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)
- 6F Lifecycle Flow: [ARCHITECTURE.md](ARCHITECTURE.md)

---

> This document maps all components of the hexaFn system to their respective architecture layers and shows their relationships. Use it as a reference for understanding the implementation of the Hexagonal Architecture pattern in Rust.