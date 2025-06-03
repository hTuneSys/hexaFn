<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Watch Module (HexaWatch)

## Domain Layer
```mermaid
classDiagram
    class Trace {
        <<trait>>
        +start_span(name: String) Result~Span, HexaError~
        +current_span() Option~Span~
        +get_trace_id() String
        +finish_span(span: Span) Result~ _ , HexaError~
    }
    class Span {
        +id String
        +name String
        +parent_id Option~String~
        +start_time DateTime
        +end_time Option~DateTime~
        +attributes HashMap~String, String~
    }
    class LogEntry {
        +timestamp DateTime
        +level LogLevel
        +message String
        +trace_id Option~String~
        +span_id Option~String~
        +fields HashMap~String, String~
    }
    class LogLevel {
        <<enumeration>>
        Trace
        Debug
        Info
        Warn
        Error
    }
    class MetricPoint {
        +name String
        +value f64
        +timestamp DateTime
        +tags HashMap~String, String~
    }
    class WatchEvent {
        <<trait>>
        +event_type() &'static str
        +payload() serde_json::Value
    }
    class AlertTriggeredEvent {
        +alert_id String
        +timestamp DateTime
        +message String
    }
    Trace --> Span
    LogEntry --> LogLevel
    MetricPoint --> LogEntry
    WatchEvent <|-- AlertTriggeredEvent
```

## Application Layer
```mermaid
classDiagram
    class LoggingPort {
        <<port>>
        +log(entry: LogEntry) Result~_, HexaError~
        +set_level(level: LogLevel) Result~_, HexaError~
    }
    class TracingPort {
        <<port>>
        +start_trace(name: String) Result~Trace, HexaError~
        +finish_trace(trace: Trace) Result~_, HexaError~
    }
    class MetricsPort {
        <<port>>
        +record(metric: MetricPoint) Result~_, HexaError~
        +get_metrics() Vec~MetricPoint~
    }
    class WatchService {
        +tail_logs(filter: Option~LogFilter~) Result~Vec~LogEntry~, HexaError~
        +export_metrics() Result~Vec~MetricPoint~, HexaError~
        +trigger_alert(alert: AlertConfig) Result~_, HexaError~
    }
    class LogFilter {
        +level Option~LogLevel~
        +trace_id Option~String~
        +span_id Option~String~
        +message_contains Option~String~
    }
    class AlertConfig {
        +id String
        +threshold f64
        +metric String
        +duration Duration
    }
    class WatchCommand {
        <<command>>
        +TailLogs
        +ExportMetrics
        +TriggerAlert
    }
    class WatchQuery {
        <<query>>
        +GetTrace
        +GetMetrics
    }
    LoggingPort --> LogEntry
    TracingPort --> Trace
    MetricsPort --> MetricPoint
    WatchService --> LoggingPort
    WatchService --> TracingPort
    WatchService --> MetricsPort
```

## Infrastructure Layer
```mermaid
classDiagram
    class LogExporter {
        +export(entry: LogEntry) Result~_, HexaError~
    }
    class TraceExporter {
        +export(trace: Trace) Result~_, HexaError~
    }
    class MetricsExporter {
        +export(metrics: Vec~MetricPoint~) Result~_, HexaError~
    }
    class WatchCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +tail_logs() Result~Vec~LogEntry~, HexaError~
        +export_metrics() Result~Vec~MetricPoint~, HexaError~
    }
    class WatchDto {
        +trace_id String
        +span_id String
        +level String
        +message String
        +timestamp DateTime
    }
    class WatchMapper {
        +to_dto(entry: LogEntry) WatchDto
        +from_dto(dto: WatchDto) LogEntry
    }
    LogExporter --> LogEntry
    TraceExporter --> Trace
    MetricsExporter --> MetricPoint
    WatchMapper --> WatchDto
    WatchMapper --> LogEntry
```

---
