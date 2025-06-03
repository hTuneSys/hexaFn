<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# hexaFn Data Model

This document provides a comprehensive, production-ready data model for the hexaFn project, covering all 6 main modules (Core, Trigger, Store, Cast, Watch, Bridge, Run) and all Hexagonal Architecture layers (Domain, Application, Infrastructure). It includes all required entities, traits, ports, adapters, services, repositories, DTOs, mappers, event flows, CLI/config/orchestrator abstractions, and advanced features as required by the issue list and project milestones.

---

# Core Module (Shared Kernel)

## Domain Layer
```mermaid
classDiagram
    class Pipeline {
        <<trait>>
        +execute(input: Input) Result~Output, HexaError~
        +add_stage(stage: Box~PipelineStage~) Result~_, HexaError~
        +get_stages() Vec~Box~PipelineStage~~
        +build() Result~Self, HexaError~
        +validate() Result~_, HexaError~
    }
    class PipelineStage {
        <<trait>>
        +stage_type() PipelineStageType
        +get_order() u32
        +execute(context: &mut PipelineContext) Result~_, Box~HexaError~~
        +validate() Result~_, Box~HexaError~~
    }
    class PipelineStageType {
        <<enumeration>>
        Feed
        Filter
        Format
        Function
        Forward
        Feedback
    }
    class PipelineContext {
        +data HashMap~String, serde_json::Value~
        +new() PipelineContext
        +get(key: &str) Option~serde_json::Value~
        +set(key: String, value: serde_json::Value) void
        +clone() PipelineContext
    }
    class Event {
        <<trait>>
        +event_type() &'static str
        +event_id() EventId
        +timestamp() chrono::DateTime~chrono::Utc~
        +payload() serde_json::Value
    }
    class EventId {
        +value uuid::Uuid
        +new() EventId
        +from_string(s: &str) Result~EventId, uuid::Error~
        +to_string() String
    }
    class DomainEvent {
        <<trait>>
        +aggregate_id() &str
        +sequence_number() u64
        +occurred_at() chrono::DateTime~chrono::Utc~
        +correlation_id() &str
    }
    class HexaError {
        <<trait>>
        +error_code() &'static str
        +error_message() String
        +kind() HexaErrorKind
        +severity() HexaErrorSeverity
        +source() Option~Box~dyn std::error::Error~~
    }
    class HexaErrorKind {
        <<enumeration>>
        NotFound
        Validation
        Timeout
        Internal
        External
        Unknown
    }
    class HexaErrorSeverity {
        <<enumeration>>
        Low
        Medium
        High
        Critical
    }
    class CoreDomainEvent {
        <<trait>>
        +event_type() &'static str
        +payload() serde_json::Value
    }
    Pipeline --> PipelineStage : contains
    PipelineStage --> PipelineContext : uses
    PipelineStage --> PipelineStageType : has type
    Event --> EventId : has
    DomainEvent --|> Event : extends
    HexaError --> HexaErrorKind : categorized by
    HexaError --> HexaErrorSeverity : prioritized by
```

## Application Layer
```mermaid
classDiagram
    class PipelineOrchestrator {
        +start_pipeline(config: PipelineConfig) Result~PipelineInstance, HexaError~
        +reload_pipelines() Result~usize, HexaError~
        +list_pipelines() Vec~PipelineInstance~
        +stop_pipeline(id: PipelineId) Result~_, HexaError~
    }
    class PipelineConfigLoader {
        +load_from_file(path: String) Result~Vec~PipelineConfig~, HexaError~
        +validate(config: PipelineConfig) Result~_, HexaError~
    }
    class PipelineConfig {
        +id String
        +stages Vec~PipelineStageConfig~
        +enabled bool
        +metadata HashMap~String, String~
    }
    class PipelineStageConfig {
        +stage_type PipelineStageType
        +order u32
        +params serde_json::Value
    }
    class PipelineInstance {
        +id String
        +status PipelineStatus
        +started_at DateTime
        +stages Vec~PipelineStage~
    }
    class PipelineStatus {
        <<enumeration>>
        Running
        Stopped
        Failed
        Completed
    }
    class PipelineCommand {
        <<command>>
        +StartPipeline
        +StopPipeline
        +ReloadPipelines
    }
    class PipelineQuery {
        <<query>>
        +ListPipelines
        +GetPipelineStatus
    }
    class PipelineValidator {
        +validate_config(config: PipelineConfig) Result~_, HexaError~
    }
    PipelineOrchestrator --> PipelineConfig
    PipelineOrchestrator --> PipelineInstance
    PipelineConfigLoader --> PipelineConfig
    PipelineInstance --> PipelineStage
    PipelineInstance --> PipelineStatus
    PipelineValidator --> PipelineConfig
```

## Infrastructure Layer
```mermaid
classDiagram
    class PipelineCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +reload() Result~_, HexaError~
        +list() Result~Vec~PipelineInstance~, HexaError~
    }
    class PipelineRepository {
        +save(instance: PipelineInstance) Result~_, HexaError~
        +load(id: String) Result~Option~PipelineInstance~, HexaError~
        +list() Result~Vec~PipelineInstance~, HexaError~
    }
    class PipelineEventBus {
        +publish(event: CoreDomainEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(CoreDomainEvent))
    }
    class PipelineDto {
        +id String
        +status String
        +stages Vec~String~
        +metadata HashMap~String, String~
    }
    class PipelineMapper {
        +to_dto(instance: PipelineInstance) PipelineDto
        +from_dto(dto: PipelineDto) PipelineInstance
    }
    PipelineCli --> PipelineOrchestrator
    PipelineRepository --> PipelineInstance
    PipelineEventBus --> CoreDomainEvent
    PipelineMapper --> PipelineDto
    PipelineMapper --> PipelineInstance
```

---

---

# Developer Tooling & Test Abstractions

```mermaid
classDiagram
    class TestCase {
        +id String
        +description String
        +setup fn() -> ()
        +execute fn() -> ()
        +teardown fn() -> ()
        +expected_result serde_json::Value
    }
    class IntegrationTestSuite {
        +name String
        +cases Vec~TestCase~
        +run_all() Result~_, HexaError~
    }
    class BenchmarkSuite {
        +name String
        +cases Vec~TestCase~
        +run_all() Result~_, HexaError~
    }
    class CliTestHarness {
        +run_command(cmd: String, args: Vec~String~) Result~String, HexaError~
        +assert_output(expected: String) Result~_, HexaError~
    }
    IntegrationTestSuite --> TestCase
    BenchmarkSuite --> TestCase
    CliTestHarness --> TestCase
```

---

# Cross-Module Event Flows & Orchestration

```mermaid
classDiagram
    class Orchestrator {
        +start() Result~_, HexaError~
        +stop() Result~_, HexaError~
        +reload_config() Result~_, HexaError~
        +handle_event(event: CoreDomainEvent) Result~_, HexaError~
        +metrics_endpoint() Result~Vec~MetricPoint~, HexaError~
    }
    class ConfigHotReloader {
        +watch(path: String) Result~_, HexaError~
        +on_change(callback: fn())
    }
    class MetricsEndpoint {
        +serve(port: u16) Result~_, HexaError~
        +get_stats() Result~Vec~MetricPoint~, HexaError~
    }
    Orchestrator --> CoreDomainEvent
    Orchestrator --> MetricsEndpoint
    ConfigHotReloader --> Orchestrator
    MetricsEndpoint --> MetricPoint
```

---

This model is designed to be exhaustive and production-ready, covering all layers, flows, and advanced features required by the hexaFn project and its roadmap.
