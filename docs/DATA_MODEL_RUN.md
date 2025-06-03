<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Run Module (HexaRun)

## Domain Layer
```mermaid
classDiagram
    class FunctionRuntime {
        <<trait>>
        +execute(context: FunctionContext) Result~ExecutionResult, HexaError~
        +init() Result~ _ , HexaError~
        +shutdown() Result~ _ , HexaError~
        +get_runtime_type() String
    }
    class FunctionContext {
        +inputs HashMap~String, Any~
        +metadata HashMap~String, String~
        +environment HashMap~String, String~
        +get_input(key: String) Option~Any~
        +set_output(key: String, value: Any) void
        +get_metadata(key: String) Option~String~
    }
    class ExecutionResult {
        +status ExecutionStatus
        +outputs HashMap~String, Any~
        +error Option~Error~
        +duration Duration
        +memory_used u64
        +is_success() bool
        +get_output(key: String) Option~Any~
    }
    class ExecutionStatus {
        <<enumeration>>
        Success
        Failure
        Timeout
        Cancelled
    }
    class Error {
        +code String
        +message String
        +kind String
        +severity String
    }
    class FunctionRegisteredEvent {
        +function_id String
        +timestamp DateTime
        +payload serde_json::Value
    }
    class FunctionExecutedEvent {
        +function_id String
        +execution_id String
        +timestamp DateTime
        +status ExecutionStatus
        +outputs HashMap~String, Any~
    }
    FunctionRuntime --> FunctionContext
    FunctionRuntime --> ExecutionResult
    ExecutionResult --> ExecutionStatus
    ExecutionResult --> Error
    FunctionExecutedEvent --> ExecutionResult
```

## Application Layer
```mermaid
classDiagram
    class FunctionRegistryPort {
        <<port>>
        +register_function(config: FunctionConfig) Result~_, HexaError~
        +remove_function(id: String) Result~_, HexaError~
        +list_functions() Vec~FunctionDefinition~
    }
    class FunctionExecutionPort {
        <<port>>
        +execute_function(id: String, context: FunctionContext) Result~ExecutionResult, HexaError~
    }
    class RunService {
        +select_runtime(type: String) Result~FunctionRuntime, HexaError~
        +validate_input(context: FunctionContext) Result~_, HexaError~
        +validate_output(result: ExecutionResult) Result~_, HexaError~
        +fallback_chain(id: String) Vec~String~
    }
    class FunctionConfigLoader {
        +load_from_file(path: String) Result~Vec~FunctionConfig~, HexaError~
    }
    class FunctionConfig {
        +id String
        +runtime String
        +code String
        +input_schema Option~Schema~
        +output_schema Option~Schema~
        +env HashMap~String, String~
        +timeout Option~Duration~
        +fallback Vec~String~
    }
    class FunctionDefinition {
        +id String
        +runtime String
        +metadata HashMap~String, String~
    }
    class RunCommand {
        <<command>>
        +RegisterFunction
        +RemoveFunction
        +TestFunction
    }
    class RunQuery {
        <<query>>
        +ListFunctions
        +GetFunction
    }
    class RunValidator {
        +validate_config(config: FunctionConfig) Result~_, HexaError~
        +validate_input(context: FunctionContext) Result~_, HexaError~
        +validate_output(result: ExecutionResult) Result~_, HexaError~
    }
    FunctionRegistryPort --> FunctionDefinition
    FunctionExecutionPort --> FunctionDefinition
    RunService --> FunctionRegistryPort
    RunService --> FunctionExecutionPort
    FunctionConfigLoader --> FunctionConfig
    RunValidator --> FunctionConfig
```

## Infrastructure Layer
```mermaid
classDiagram
    class FunctionRepository {
        +save(definition: FunctionDefinition) Result~_, HexaError~
        +load(id: String) Result~Option~FunctionDefinition~, HexaError~
        +list() Result~Vec~FunctionDefinition~, HexaError~
    }
    class RuntimeFactory {
        +get_runtime(type: String) Result~FunctionRuntime, HexaError~
    }
    class RunEventBus {
        +publish(event: FunctionExecutedEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(FunctionExecutedEvent))
    }
    class RunCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +test_function(id: String, input: serde_json::Value) Result~_, HexaError~
    }
    class FunctionDto {
        +id String
        +runtime String
        +code String
        +metadata HashMap~String, String~
    }
    class RunMapper {
        +to_dto(definition: FunctionDefinition) FunctionDto
        +from_dto(dto: FunctionDto) FunctionDefinition
    }
    FunctionRepository --> FunctionDefinition
    RuntimeFactory --> FunctionRuntime
    RunEventBus --> FunctionExecutedEvent
    RunMapper --> FunctionDto
    RunMapper --> FunctionDefinition
```

---
