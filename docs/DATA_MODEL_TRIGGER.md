<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Trigger Module (HexaTrigger)

## Domain Layer
```mermaid
classDiagram
    class Trigger {
        <<trait>>
        +id() String
        +name() String
        +is_active() bool
        +evaluate(context: Any) Result~bool, HexaError~
        +get_conditions() Vec~TriggerCondition~
        +priority() u32
        +deactivate() void
        +activate() void
        +timeout() Option~Duration~
    }
    class TriggerCondition {
        <<trait>>
        +matches(context: Any) Result~bool, HexaError~
        +description() String
        +get_priority() u32
        +compound_type() Option~CompoundType~
    }
    class CompoundType {
        <<enumeration>>
        And
        Or
        Not
    }
    class TriggerEvent {
        <<trait>>
        +trigger_id() String
        +event_type() &'static str
        +timestamp() chrono::DateTime~chrono::Utc~
        +payload() serde_json::Value
    }
    class TriggerCreatedEvent {
        +trigger_id String
        +timestamp DateTime
        +payload serde_json::Value
    }
    class TriggerFiredEvent {
        +trigger_id String
        +event_id String
        +timestamp DateTime
        +payload serde_json::Value
    }
    class TriggerDeactivatedEvent {
        +trigger_id String
        +timestamp DateTime
    }
    Trigger --> TriggerCondition
    TriggerEvent <|-- TriggerCreatedEvent
    TriggerEvent <|-- TriggerFiredEvent
    TriggerEvent <|-- TriggerDeactivatedEvent
```

## Application Layer
```mermaid
classDiagram
    class TriggerRegistryPort {
        <<port>>
        +register(trigger: Trigger) Result~_, HexaError~
        +unregister(id: String) Result~_, HexaError~
        +get(id: String) Option~Trigger~
        +list() Vec~Trigger~
        +reload_from_config(path: String) Result~usize, HexaError~
    }
    class TriggerEvaluationPort {
        <<port>>
        +evaluate(trigger_id: String, context: Any) Result~bool, HexaError~
    }
    class TriggerService {
        +activate_trigger(id: String) Result~_, HexaError~
        +deactivate_trigger(id: String) Result~_, HexaError~
        +reload_triggers() Result~usize, HexaError~
        +list_active() Vec~Trigger~
        +set_priority(id: String, priority: u32) Result~_, HexaError~
    }
    class TriggerCommand {
        <<command>>
        +ActivateTrigger
        +DeactivateTrigger
        +ReloadTriggers
        +SetPriority
    }
    class TriggerQuery {
        <<query>>
        +ListTriggers
        +GetTrigger
    }
    class TriggerValidator {
        +validate_config(config: TriggerConfig) Result~_, HexaError~
        +validate_compound(conditions: Vec~TriggerCondition~) Result~_, HexaError~
    }
    class TriggerConfigLoader {
        +load_from_file(path: String) Result~Vec~TriggerConfig~, HexaError~
    }
    class TriggerConfig {
        +id String
        +name String
        +conditions Vec~TriggerConditionConfig~
        +priority u32
        +active bool
        +timeout Option~Duration~
    }
    class TriggerConditionConfig {
        +type String
        +params serde_json::Value
        +compound CompoundType
    }
    TriggerRegistryPort --> Trigger
    TriggerEvaluationPort --> Trigger
    TriggerService --> TriggerRegistryPort
    TriggerService --> TriggerEvaluationPort
    TriggerConfigLoader --> TriggerConfig
    TriggerValidator --> TriggerConfig
```

## Infrastructure Layer
```mermaid
classDiagram
    class TriggerRepository {
        +save(trigger: Trigger) Result~_, HexaError~
        +load(id: String) Result~Option~Trigger~, HexaError~
        +list() Result~Vec~Trigger~, HexaError~
    }
    class TriggerEventBus {
        +publish(event: TriggerEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(TriggerEvent))
    }
    class TriggerCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +reload() Result~_, HexaError~
        +list() Result~Vec~Trigger~, HexaError~
    }
    class TriggerDto {
        +id String
        +name String
        +priority u32
        +active bool
        +conditions Vec~String~
    }
    class TriggerMapper {
        +to_dto(trigger: Trigger) TriggerDto
        +from_dto(dto: TriggerDto) Trigger
    }
    TriggerRepository --> Trigger
    TriggerEventBus --> TriggerEvent
    TriggerMapper --> TriggerDto
    TriggerMapper --> Trigger
```

---
