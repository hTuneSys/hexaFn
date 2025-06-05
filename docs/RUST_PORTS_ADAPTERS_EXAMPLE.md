<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Rust Ports & Adapters Components Documentation (hexaFn)

This document provides a comprehensive, module-by-module, layer-by-layer mapping of all ports, adapters, and domain components implemented in hexaFn's Hexagonal Architecture. Each table is derived from the real data models and flow diagrams in the project documentation.

---

## 🔵 Core Module (`hexafn-core`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `Pipeline` | 6F lifecycle pipeline abstraction | `PipelineStage`, `PipelineContext` |
| Trait | `PipelineStage` | Represents a stage in the pipeline (Feed, Filter, etc.) | `PipelineStageType`, `PipelineContext` |
| Enum | `PipelineStageType` | Stage type: Feed, Filter, Format, Function, Forward, Feedback | - |
| Struct | `PipelineContext` | Shared context/data for pipeline execution | - |
| Trait | `Event` | Base event abstraction | `EventId`, `DomainEvent` |
| Struct | `EventId` | Unique event identifier | `Event` |
| Trait | `DomainEvent` | Domain event contract | `EventId` |
| Trait | `HexaError` | Error abstraction for all modules | `HexaErrorKind`, `HexaErrorSeverity` |
| Enum | `HexaErrorKind` | Error kind (NotFound, Validation, etc.) | `HexaError` |
| Enum | `HexaErrorSeverity` | Error severity (Low, Medium, High, Critical) | `HexaError` |
| Trait | `CoreDomainEvent` | Core event for cross-module communication | - |
| Struct | `PipelineBuilder` | Builder for constructing pipelines | `Pipeline`, `PipelineStage` |
| Struct | `PipelineStageRegistry` | Registry for pipeline stage factories | `PipelineStageFactory` |
| Trait | `PipelineStageFactory` | Factory for creating pipeline stages | `PipelineStageType` |
| Struct | `PipelineStageMetadata` | Metadata for pipeline stages | - |
| Struct | `PipelineLock` | Locking for pipeline concurrency | - |
| Struct | `PipelineAuditTrail` | Audit trail for pipeline execution | `PipelineAuditEvent` |
| Struct | `PipelineTag` | Tag for pipeline categorization | - |
| Struct | `PipelineDependencyGraph` | Dependency graph for pipelines | - |
| Struct | `PipelineRollbackPoint` | Rollback point for pipeline state | - |
| Struct | `PipelineAccessControl` | Access control for pipelines | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Service | `PipelineOrchestrator` | Orchestrates pipeline execution and lifecycle | `PipelineConfig`, `PipelineInstance` |
| Service | `PipelineConfigLoader` | Loads pipeline configs from files | `PipelineConfig` |
| Struct | `PipelineConfig` | Pipeline configuration | `PipelineStageConfig` |
| Struct | `PipelineStageConfig` | Stage configuration | `PipelineStageType` |
| Struct | `PipelineInstance` | Running pipeline instance | `PipelineStage`, `PipelineStatus` |
| Enum | `PipelineStatus` | Pipeline status (Running, Stopped, etc.) | - |
| Command | `PipelineCommand` | Pipeline commands (Start, Stop, Reload) | - |
| Query | `PipelineQuery` | Pipeline queries (List, Status) | - |
| Service | `PipelineValidator` | Validates pipeline configs | `PipelineConfig` |
| Service | `PipelineAuditService` | Records pipeline audit events | `PipelineAuditTrail` |
| Service | `PipelineLockService` | Manages pipeline locks | - |
| Service | `PipelineDependencyService` | Resolves pipeline dependencies | - |
| Service | `PipelineRollbackService` | Manages rollback points | `PipelineRollbackPoint` |
| Service | `PipelineTaggingService` | Manages pipeline tags | `PipelineTag` |
| Service | `PipelineAccessControlService` | Manages pipeline access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `PipelineCli` | CLI for pipeline management | `PipelineOrchestrator` |
| Adapter | `PipelineRepository` | Persists pipeline instances | `PipelineInstance` |
| Adapter | `PipelineEventBus` | Publishes pipeline events | `CoreDomainEvent` |
| Mapper | `PipelineMapper` | Maps between pipeline entities and DTOs | `PipelineDto`, `PipelineInstance` |
| DTO | `PipelineDto` | Data transfer object for pipelines | - |
| Adapter | `PipelineAuditRepository` | Persists pipeline audit trails | `PipelineAuditTrail` |
| Adapter | `PipelineLockManager` | Manages pipeline locks | - |
| Adapter | `PipelineDependencyAdapter` | Fetches pipeline dependencies | - |
| Adapter | `PipelineRollbackAdapter` | Manages rollback points | `PipelineRollbackPoint` |
| Adapter | `PipelineTagStore` | Stores pipeline tags | `PipelineTag` |
| Adapter | `PipelineAccessControlAdapter` | Manages pipeline access | - |

---

## 🟢 Trigger Module (`hexafn-trigger`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `Trigger` | Trigger abstraction (id, name, evaluate, etc.) | `TriggerCondition` |
| Trait | `TriggerCondition` | Condition for trigger evaluation | `CompoundType` |
| Enum | `CompoundType` | Compound condition type (And, Or, Not) | - |
| Trait | `TriggerEvent` | Base event for triggers | - |
| Struct | `TriggerCreatedEvent` | Event for trigger creation | - |
| Struct | `TriggerFiredEvent` | Event for trigger firing | - |
| Struct | `TriggerDeactivatedEvent` | Event for trigger deactivation | - |
| Struct | `TriggerAuditTrail` | Audit trail for triggers | `TriggerAuditEvent` |
| Struct | `TriggerLock` | Locking for triggers | - |
| Struct | `TriggerDependencyGraph` | Dependency graph for triggers | - |
| Struct | `TriggerRollbackPoint` | Rollback point for triggers | - |
| Struct | `TriggerTag` | Tag for triggers | - |
| Struct | `TriggerAccessControl` | Access control for triggers | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `TriggerRegistryPort` | Register, unregister, list triggers | `Trigger` |
| Port | `TriggerEvaluationPort` | Evaluate triggers | `Trigger` |
| Service | `TriggerService` | Activate, deactivate, reload triggers | `TriggerRegistryPort`, `TriggerEvaluationPort` |
| Command | `TriggerCommand` | Trigger commands (Activate, Deactivate, etc.) | - |
| Query | `TriggerQuery` | Trigger queries (List, Get) | - |
| Service | `TriggerValidator` | Validates trigger configs | `TriggerConfig` |
| Service | `TriggerConfigLoader` | Loads trigger configs from files | `TriggerConfig` |
| Struct | `TriggerConfig` | Trigger configuration | `TriggerConditionConfig` |
| Struct | `TriggerConditionConfig` | Condition configuration | `CompoundType` |
| Service | `TriggerAuditService` | Records trigger audit events | `TriggerAuditTrail` |
| Service | `TriggerLockService` | Manages trigger locks | - |
| Service | `TriggerDependencyService` | Resolves trigger dependencies | - |
| Service | `TriggerRollbackService` | Manages rollback points | `TriggerRollbackPoint` |
| Service | `TriggerTaggingService` | Manages trigger tags | `TriggerTag` |
| Service | `TriggerAccessControlService` | Manages trigger access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `TriggerRepository` | Persists triggers | `Trigger` |
| Adapter | `TriggerEventBus` | Publishes trigger events | `TriggerEvent` |
| Adapter | `TriggerCli` | CLI for trigger management | `Trigger` |
| Mapper | `TriggerMapper` | Maps between trigger entities and DTOs | `TriggerDto`, `Trigger` |
| DTO | `TriggerDto` | Data transfer object for triggers | - |
| Adapter | `TriggerAuditRepository` | Persists trigger audit trails | `TriggerAuditTrail` |
| Adapter | `TriggerLockManager` | Manages trigger locks | - |
| Adapter | `TriggerDependencyAdapter` | Fetches trigger dependencies | - |
| Adapter | `TriggerRollbackAdapter` | Manages rollback points | `TriggerRollbackPoint` |
| Adapter | `TriggerTagStore` | Stores trigger tags | `TriggerTag` |
| Adapter | `TriggerAccessControlAdapter` | Manages trigger access | - |

---

## 🟠 Run Module (`hexafn-run`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `FunctionRuntime` | Function runtime abstraction (execute, init, shutdown) | `FunctionContext`, `ExecutionResult` |
| Struct | `FunctionContext` | Context for function execution | - |
| Struct | `ExecutionResult` | Result of function execution | - |
| Enum | `ExecutionStatus` | Status of execution (Success, Failure, etc.) | - |
| Struct | `Error` | Error details for execution | - |
| Struct | `FunctionRegisteredEvent` | Event for function registration | - |
| Struct | `FunctionExecutedEvent` | Event for function execution | - |
| Struct | `FunctionAuditTrail` | Audit trail for functions | `FunctionExecutedEvent` |
| Struct | `FunctionLock` | Locking for functions | - |
| Struct | `FunctionDependencyGraph` | Dependency graph for functions | - |
| Struct | `FunctionRollbackPoint` | Rollback point for functions | - |
| Struct | `FunctionTag` | Tag for functions | - |
| Struct | `FunctionAccessControl` | Access control for functions | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `FunctionRegistryPort` | Register, remove, list functions | `FunctionDefinition` |
| Port | `FunctionExecutionPort` | Execute functions | `FunctionContext`, `ExecutionResult` |
| Service | `RunService` | Selects runtime, validates input/output, fallback chain | `FunctionRuntime` |
| Service | `FunctionConfigLoader` | Loads function configs from files | `FunctionConfig` |
| Struct | `FunctionConfig` | Function configuration | - |
| Struct | `FunctionDefinition` | Function definition | - |
| Command | `RunCommand` | Run commands (Register, Remove, Test) | - |
| Query | `RunQuery` | Run queries (List, Get) | - |
| Service | `RunValidator` | Validates function configs and results | `FunctionConfig`, `ExecutionResult` |
| Service | `FunctionAuditService` | Records function audit events | `FunctionAuditTrail` |
| Service | `FunctionLockService` | Manages function locks | - |
| Service | `FunctionDependencyService` | Resolves function dependencies | - |
| Service | `FunctionRollbackService` | Manages rollback points | `FunctionRollbackPoint` |
| Service | `FunctionTaggingService` | Manages function tags | `FunctionTag` |
| Service | `FunctionAccessControlService` | Manages function access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `FunctionRepository` | Persists function definitions | `FunctionDefinition` |
| Adapter | `RuntimeFactory` | Provides runtime implementations | `FunctionRuntime` |
| Adapter | `RunEventBus` | Publishes function execution events | `FunctionExecutedEvent` |
| Adapter | `RunCli` | CLI for function management/testing | `FunctionDefinition` |
| Mapper | `RunMapper` | Maps between function entities and DTOs | `FunctionDto`, `FunctionDefinition` |
| DTO | `FunctionDto` | Data transfer object for functions | - |
| Adapter | `FunctionAuditRepository` | Persists function audit trails | `FunctionAuditTrail` |
| Adapter | `FunctionLockManager` | Manages function locks | - |
| Adapter | `FunctionDependencyAdapter` | Fetches function dependencies | - |
| Adapter | `FunctionRollbackAdapter` | Manages rollback points | `FunctionRollbackPoint` |
| Adapter | `FunctionTagStore` | Stores function tags | `FunctionTag` |
| Adapter | `FunctionAccessControlAdapter` | Manages function access | - |

---

## 🟡 Store Module (`hexafn-store`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `KvStore` | Key-value store abstraction (CRUD, transaction) | `Namespace`, `KeyValueEntry`, `KvOp` |
| Struct | `Namespace` | Namespace for key scoping | - |
| Struct | `KeyValueEntry` | Key-value entry with metadata | - |
| Enum | `KvOp` | KV operation (Put, Delete) | - |
| Struct | `StoreEntryCreatedEvent` | Event for entry creation | - |
| Struct | `StoreEntryUpdatedEvent` | Event for entry update | - |
| Struct | `StoreEntryDeletedEvent` | Event for entry deletion | - |
| Struct | `StoreAuditTrail` | Audit trail for store | `StoreAuditEvent` |
| Struct | `StoreLock` | Locking for store | - |
| Struct | `StoreDependencyGraph` | Dependency graph for store | - |
| Struct | `StoreRollbackPoint` | Rollback point for store | - |
| Struct | `StoreTag` | Tag for store | - |
| Struct | `StoreAccessControl` | Access control for store | - |
| Struct | `StoreSnapshot` | Store snapshot for backup | - |
| Struct | `StoreReplication` | Store replication status | - |
| Struct | `StoreChangeFeed` | Change feed for store | - |
| Struct | `StoreAccessAudit` | Access audit for store | - |
| Struct | `StoreQuotaManager` | Quota management for store | - |
| Struct | `StoreSchemaMigration` | Schema migration for store | - |
| Struct | `StoreSoftDelete` | Soft delete for entries | - |
| Struct | `StoreKeyRotation` | Key rotation for entries | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `KvStorePort` | CRUD and transaction operations | `KeyValueEntry`, `KvOp` |
| Service | `StoreService` | Backup, restore, TTL, validation | `KvStorePort` |
| Command | `StoreCommand` | Store commands (Backup, SetTTL, etc.) | - |
| Query | `StoreQuery` | Store queries (ListNamespaces, GetEntry) | - |
| Service | `StoreValidator` | Validates entry schemas | `KeyValueEntry` |
| Service | `StoreConfigLoader` | Loads store configs from files | `StoreConfig` |
| Struct | `StoreConfig` | Store configuration | `NamespaceConfig` |
| Struct | `NamespaceConfig` | Namespace configuration | - |
| Service | `StoreAuditService` | Records store audit events | `StoreAuditTrail` |
| Service | `StoreLockService` | Manages store locks | - |
| Service | `StoreDependencyService` | Resolves store dependencies | - |
| Service | `StoreRollbackService` | Manages rollback points | `StoreRollbackPoint` |
| Service | `StoreTaggingService` | Manages store tags | `StoreTag` |
| Service | `StoreAccessControlService` | Manages store access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `StoreRepository` | Persists key-value entries | `KeyValueEntry` |
| Adapter | `StoreEventBus` | Publishes store events | `StoreEntryCreatedEvent` |
| Adapter | `StoreCli` | CLI for store management | `KeyValueEntry` |
| Mapper | `StoreMapper` | Maps between store entities and DTOs | `StoreDto`, `KeyValueEntry` |
| DTO | `StoreDto` | Data transfer object for store entries | - |
| Adapter | `StoreAuditRepository` | Persists store audit trails | `StoreAuditTrail` |
| Adapter | `StoreLockManager` | Manages store locks | - |
| Adapter | `StoreDependencyAdapter` | Fetches store dependencies | - |
| Adapter | `StoreRollbackAdapter` | Manages rollback points | `StoreRollbackPoint` |
| Adapter | `StoreTagStore` | Stores store tags | `StoreTag` |
| Adapter | `StoreAccessControlAdapter` | Manages store access | - |

---

## 🟣 Cast Module (`hexafn-cast`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Struct | `Topic` | Pub/sub topic | - |
| Struct | `RetentionPolicy` | Retention policy for topics | - |
| Trait | `Publisher` | Publishes messages to topics | `Topic` |
| Trait | `Subscriber` | Subscribes to topics | `Topic` |
| Enum | `SubscriberLifecycle` | Subscriber lifecycle state | - |
| Struct | `Message` | Pub/sub message | `DeliveryStatus` |
| Enum | `DeliveryStatus` | Message delivery status | - |
| Struct | `MessageDeliveredEvent` | Event for message delivery | - |
| Struct | `TopicCreatedEvent` | Event for topic creation | - |
| Struct | `TopicRetentionPolicyChangedEvent` | Event for retention policy change | - |
| Struct | `CastAuditTrail` | Audit trail for cast | `CastAuditEvent` |
| Struct | `CastLock` | Locking for cast | - |
| Struct | `CastDependencyGraph` | Dependency graph for cast | - |
| Struct | `CastRollbackPoint` | Rollback point for cast | - |
| Struct | `CastTag` | Tag for cast | - |
| Struct | `CastAccessControl` | Access control for cast | - |
| Struct | `TopicPartitioning` | Topic partitioning info | - |
| Struct | `TopicACL` | Topic access control list | - |
| Struct | `TopicVersioning` | Topic version info | - |
| Struct | `TopicCompaction` | Topic compaction policy | - |
| Struct | `TopicReplayWindow` | Topic replay window | - |
| Struct | `SubscriberGroup` | Group of subscribers | - |
| Struct | `SubscriberOffsetManagement` | Offset management for subscribers | - |
| Struct | `SubscriberDeadLetterQueue` | Dead letter queue for subscribers | - |
| Struct | `MessageEncryption` | Message encryption metadata | - |
| Struct | `MessageCompression` | Message compression info | - |
| Struct | `MessageOrderingGuarantee` | Message ordering policy | - |
| Struct | `MessageDeduplication` | Message deduplication info | - |
| Struct | `MessageAuditTrail` | Audit trail for messages | `DeliveryStatus` |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `TopicManagementPort` | Manage topics | `Topic` |
| Port | `PublisherPort` | Publish messages | `Topic` |
| Port | `SubscriberPort` | Manage subscribers | `Subscriber` |
| Service | `CastService` | Registers subscribers, publishes messages | `TopicManagementPort`, `PublisherPort`, `SubscriberPort` |
| Service | `TopicConfigLoader` | Loads topic configs from files | `TopicConfig` |
| Struct | `TopicConfig` | Topic configuration | - |
| Struct | `TopicFilter` | Filter for topic messages | - |
| Command | `CastCommand` | Cast commands (CreateTopic, etc.) | - |
| Query | `CastQuery` | Cast queries (ListTopics, etc.) | - |
| Service | `CastValidator` | Validates topic configs | `TopicConfig` |
| Service | `CastAuditService` | Records cast audit events | `CastAuditTrail` |
| Service | `CastLockService` | Manages cast locks | - |
| Service | `CastDependencyService` | Resolves cast dependencies | - |
| Service | `CastRollbackService` | Manages rollback points | `CastRollbackPoint` |
| Service | `CastTaggingService` | Manages cast tags | `CastTag` |
| Service | `CastAccessControlService` | Manages cast access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `CastRepository` | Persists topics and subscribers | `Topic`, `Subscriber` |
| Adapter | `CastEventBus` | Publishes message delivery events | `MessageDeliveredEvent` |
| Adapter | `CastCli` | CLI for cast management | `Topic`, `Subscriber` |
| Mapper | `CastMapper` | Maps between cast entities and DTOs | `CastDto`, `Message` |
| DTO | `CastDto` | Data transfer object for cast messages | - |
| Adapter | `OutputForwarder` | Forwards outputs to targets | `OutputTarget` |
| Enum | `OutputTarget` | Output target type (Http, Cast, etc.) | - |
| Adapter | `CastAuditRepository` | Persists cast audit trails | `CastAuditTrail` |
| Adapter | `CastLockManager` | Manages cast locks | - |
| Adapter | `CastDependencyAdapter` | Fetches cast dependencies | - |
| Adapter | `CastRollbackAdapter` | Manages rollback points | `CastRollbackPoint` |
| Adapter | `CastTagStore` | Stores cast tags | `CastTag` |
| Adapter | `CastAccessControlAdapter` | Manages cast access | - |

---

## 🟤 Bridge Module (`hexafn-bridge`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `Integration` | External integration abstraction | `EventTransformer` |
| Trait | `Webhook` | Webhook endpoint abstraction | `HttpMethod`, `Response` |
| Trait | `EventTransformer` | Transforms and validates events | - |
| Trait | `BridgeEvent` | Base event for bridge | - |
| Struct | `WebhookReceivedEvent` | Event for webhook received | - |
| Struct | `IntegrationRegisteredEvent` | Event for integration registration | - |
| Enum | `ConnectionStatus` | Integration connection status | - |
| Enum | `HttpMethod` | HTTP method for webhooks | - |
| Struct | `Response` | HTTP response for webhooks | - |
| Struct | `BridgeAuditTrail` | Audit trail for bridge | `BridgeAuditEvent` |
| Struct | `BridgeLock` | Locking for bridge | - |
| Struct | `BridgeDependencyGraph` | Dependency graph for bridge | - |
| Struct | `BridgeRollbackPoint` | Rollback point for bridge | - |
| Struct | `BridgeTag` | Tag for bridge | - |
| Struct | `BridgeAccessControl` | Access control for bridge | - |
| Struct | `IntegrationHealthCheck` | Health check for integrations | - |
| Struct | `IntegrationRetryPolicy` | Retry policy for integrations | - |
| Struct | `IntegrationCircuitBreaker` | Circuit breaker for integrations | - |
| Struct | `IntegrationRateLimit` | Rate limiting for integrations | - |
| Struct | `IntegrationTransformationTemplate` | Transformation template for integrations | - |
| Struct | `IntegrationApprovalWorkflow` | Approval workflow for integrations | - |
| Struct | `IntegrationSecretManager` | Secret management for integrations | - |
| Struct | `IntegrationAuditTrail` | Audit trail for integrations | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `IntegrationManagementPort` | Register, remove, list integrations | `Integration` |
| Port | `WebhookReceiverPort` | Receive webhooks | `Webhook` |
| Service | `BridgeService` | Normalizes payloads, dispatches events | `IntegrationManagementPort`, `WebhookReceiverPort` |
| Service | `IntegrationConfigLoader` | Loads integration configs from files | `IntegrationConfig` |
| Struct | `IntegrationConfig` | Integration configuration | - |
| Command | `BridgeCommand` | Bridge commands (RegisterIntegration, etc.) | - |
| Query | `BridgeQuery` | Bridge queries (ListIntegrations, etc.) | - |
| Service | `BridgeAuditService` | Records bridge audit events | `BridgeAuditTrail` |
| Service | `BridgeLockService` | Manages bridge locks | - |
| Service | `BridgeDependencyService` | Resolves bridge dependencies | - |
| Service | `BridgeRollbackService` | Manages rollback points | `BridgeRollbackPoint` |
| Service | `BridgeTaggingService` | Manages bridge tags | `BridgeTag` |
| Service | `BridgeAccessControlService` | Manages bridge access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `IntegrationRepository` | Persists integrations | `Integration` |
| Adapter | `BridgeEventBus` | Publishes bridge events | `BridgeEvent` |
| Adapter | `BridgeCli` | CLI for bridge management | `Integration` |
| Mapper | `BridgeMapper` | Maps between integration entities and DTOs | `IntegrationDto`, `Integration` |
| DTO | `IntegrationDto` | Data transfer object for integrations | - |
| Adapter | `BridgeAuditRepository` | Persists bridge audit trails | `BridgeAuditTrail` |
| Adapter | `BridgeLockManager` | Manages bridge locks | - |
| Adapter | `BridgeDependencyAdapter` | Fetches bridge dependencies | - |
| Adapter | `BridgeRollbackAdapter` | Manages rollback points | `BridgeRollbackPoint` |
| Adapter | `BridgeTagStore` | Stores bridge tags | `BridgeTag` |
| Adapter | `BridgeAccessControlAdapter` | Manages bridge access | - |

---

## 🟠 Watch Module (`hexafn-watch`)

### Domain Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Trait | `Trace` | Distributed trace abstraction | `Span` |
| Struct | `Span` | Trace span | - |
| Struct | `LogEntry` | Structured log entry | `LogLevel` |
| Enum | `LogLevel` | Log level (Trace, Debug, etc.) | - |
| Struct | `MetricPoint` | Metric data point | - |
| Trait | `WatchEvent` | Base event for watch | - |
| Struct | `AlertTriggeredEvent` | Event for alert triggered | - |
| Struct | `WatchAuditTrail` | Audit trail for watch | `WatchAuditEvent` |
| Struct | `WatchLock` | Locking for watch | - |
| Struct | `WatchDependencyGraph` | Dependency graph for watch | - |
| Struct | `WatchRollbackPoint` | Rollback point for watch | - |
| Struct | `WatchTag` | Tag for watch | - |
| Struct | `WatchAccessControl` | Access control for watch | - |
| Struct | `LogRedaction` | Log redaction logic | - |
| Struct | `LogRetentionPolicy` | Log retention policy | - |
| Struct | `LogExporters` | Log exporters | - |
| Struct | `TraceCorrelation` | Trace correlation logic | - |
| Struct | `TraceSamplingPolicy` | Trace sampling policy | - |
| Struct | `AlertSuppression` | Alert suppression logic | - |
| Struct | `AlertEscalation` | Alert escalation logic | - |
| Struct | `MetricsDownsampling` | Metrics downsampling logic | - |
| Struct | `MetricsCustomAggregator` | Custom metrics aggregation | - |
| Struct | `WatchAccessAudit` | Access audit for watch | - |

### Application Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Port | `LoggingPort` | Structured logging | `LogEntry` |
| Port | `TracingPort` | Tracing operations | `Trace` |
| Port | `MetricsPort` | Metrics collection | `MetricPoint` |
| Service | `WatchService` | Tails logs, exports metrics, triggers alerts | `LoggingPort`, `TracingPort`, `MetricsPort` |
| Struct | `LogFilter` | Log filtering | - |
| Struct | `AlertConfig` | Alert configuration | - |
| Command | `WatchCommand` | Watch commands (TailLogs, etc.) | - |
| Query | `WatchQuery` | Watch queries (GetTrace, etc.) | - |
| Service | `WatchAuditService` | Records watch audit events | `WatchAuditTrail` |
| Service | `WatchLockService` | Manages watch locks | - |
| Service | `WatchDependencyService` | Resolves watch dependencies | - |
| Service | `WatchRollbackService` | Manages rollback points | `WatchRollbackPoint` |
| Service | `WatchTaggingService` | Manages watch tags | `WatchTag` |
| Service | `WatchAccessControlService` | Manages watch access | - |

### Infrastructure Layer
| Component Type | Name | Description | Related Components |
|---------------|------|-------------|-------------------|
| Adapter | `LogExporter` | Exports logs | `LogEntry` |
| Adapter | `TraceExporter` | Exports traces | `Trace` |
| Adapter | `MetricsExporter` | Exports metrics | `MetricPoint` |
| Adapter | `WatchCli` | CLI for watch management | `LogEntry`, `MetricPoint` |
| Mapper | `WatchMapper` | Maps between watch entities and DTOs | `WatchDto`, `LogEntry` |
| DTO | `WatchDto` | Data transfer object for watch logs | - |
| Adapter | `WatchAuditRepository` | Persists watch audit trails | `WatchAuditTrail` |
| Adapter | `WatchLockManager` | Manages watch locks | - |
| Adapter | `WatchDependencyAdapter` | Fetches watch dependencies | - |
| Adapter | `WatchRollbackAdapter` | Manages rollback points | `WatchRollbackPoint` |
| Adapter | `WatchTagStore` | Stores watch tags | `WatchTag` |
| Adapter | `WatchAccessControlAdapter` | Manages watch access | - |

---

## 📚 References

- Hexagonal Architecture (Ports & Adapters): [HEXAGONAL_ARCHITECTURE_GUIDE](HEXAGONAL_ARCHITECTURE_GUIDE.md)
- Module Structure: [PROJECT_STRUCTURE](PROJECT_STRUCTURE.md)
- 6F Lifecycle Flow: [ARCHITECTURE](ARCHITECTURE.md)
- Data Models: [DATA_MODEL_CORE](DATA_MODEL_CORE.md), [DATA_MODEL_TRIGGER](DATA_MODEL_TRIGGER.md), [DATA_MODEL_RUN](DATA_MODEL_RUN.md), [DATA_MODEL_STORE](DATA_MODEL_STORE.md), [DATA_MODEL_CAST](DATA_MODEL_CAST.md), [DATA_MODEL_BRIDGE](DATA_MODEL_BRIDGE.md), [DATA_MODEL_WATCH](DATA_MODEL_WATCH.md)
- Data Flow: [DATA_FLOW](DATA_FLOW.md), [DATA_FLOW_DETAIL](DATA_FLOW_DETAIL.md)
- TODO List: [TODO_LIST](TODO_LIST.md)

---

> This document maps all components of the hexaFn system to their respective architecture layers and shows their relationships. Use it as a reference for understanding the implementation of the Hexagonal Architecture pattern in Rust.