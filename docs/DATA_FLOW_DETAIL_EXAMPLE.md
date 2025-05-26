<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# hexaFn Detailed Component Architecture Diagram

This diagram illustrates the detailed component architecture of the hexaFn system, including the specific interfaces, structs, methods, and connections between all parts of the system organized according to **Hexagonal Architecture** principles and the **6F Lifecycle Flow**.

## Detailed Component Diagram

```mermaid
flowchart TB
    %% External Event
    ExternalEvent[External Event]
    
    %% ===== BRIDGE MODULE =====
    subgraph "hexafn-bridge (Feed Phase Entry)"
        %% Infrastructure Layer (Adapters - Primary/Inbound)
        subgraph BridgeInfra["Infrastructure Layer"]
            WebhookEndpoint["struct WebhookController\nimplements RESTController"]
            WebhookParser["struct HttpRequestParser\nimplements RequestParser"]
            TokenValidator["struct JwtTokenValidator\nimplements TokenValidation"]
        end
        
        %% Application Layer (Ports and Use Cases)
        subgraph BridgeApp["Application Layer"]
            WebhookUseCase["trait WebhookIngestionPort\nfn ingest_event(&self, payload: &[u8], headers: HeaderMap) -> Result<EventId, Error>"]
            EventPublishPort["trait EventPublisherPort\nfn publish(&self, event: WebhookEvent) -> Result<(), Error>"]
        end
        
        %% Domain Layer (Core Business Logic)
        subgraph BridgeDomain["Domain Layer"]
            WebhookEvent["struct WebhookEvent\n{\nevent_id: EventId,\ntimestamp: Timestamp,\nsource: WebhookSource,\npayload: EventPayload\n}"]
            WebhookSource["struct WebhookSource\n{\nid: SourceId,\nname: String,\ntype: SourceType\n}"]
            EventTransformer["trait EventTransformer\nfn transform(&self, raw: &[u8], headers: HeaderMap) -> Result<WebhookEvent, Error>"]
            NormalizationRules["struct EventNormalizationRules\nfn apply(&self, event: &mut WebhookEvent) -> Result<(), Error>"]
        end
        
        %% Infrastructure Layer (Adapters - Secondary/Outbound)
        subgraph BridgeOutboundInfra["Infrastructure Layer (Outbound)"]
            EventPublisher["struct KafkaEventPublisher\nimplements EventPublisherPort"]
            TopicRouter["struct TopicRouter\nfn route(&self, event: &WebhookEvent) -> String"]
        end
    end
    
    %% ===== TRIGGER MODULE =====
    subgraph "hexafn-trigger (Feed & Filter Phases)"
        %% Infrastructure Layer (Adapters - Primary/Inbound)
        subgraph TriggerInfra["Infrastructure Layer"]
            SubscriberAdapter["struct EventSubscriber\nimplements MessageConsumerPort"]
            ConfigLoader["struct YamlConfigLoader\nimplements ConfigLoaderPort"]
        end
        
        %% Application Layer (Ports and Use Cases)
        subgraph TriggerApp["Application Layer"]
            TriggerEvalUseCase["trait TriggerEvaluationPort\nfn evaluate(&self, event: &Event) -> Result<bool, Error>"]
            RuleMatchingService["struct RuleMatchingService\nfn match_rules(&self, trigger: &Trigger, event: &Event) -> Result<bool, Error>"]
            TriggerRegistryPort["trait TriggerRegistryPort\nfn find_by_id(&self, id: &TriggerId) -> Result<Option<Trigger>, Error>"]
            ExecutionRequestPort["trait ExecutionRequestPort\nfn request_execution(&self, ctx: ExecutionContext) -> Result<RequestId, Error>"]
        end
        
        %% Domain Layer (Core Business Logic)
        subgraph TriggerDomain["Domain Layer"]
            Trigger["struct Trigger\n{\nid: TriggerId,\nname: TriggerName,\nconditions: Vec<TriggerCondition>,\nstatus: TriggerStatus\n}"]
            TriggerCondition["enum TriggerCondition\n{\nValueEquals{field: String, value: Value},\nValueGreaterThan{field: String, threshold: f64},\nTimeBased{TimeCondition}\n}"]
            TriggerRule["struct TriggerRule\n{\noperator: RuleOperator,\nconditions: Vec<TriggerCondition>\n}"]
            TriggerPolicy["trait TriggerPolicy\nfn check_policy(&self, trigger: &Trigger) -> Result<(), PolicyError>"]
            TriggerActivatedEvent["struct TriggerActivatedEvent\n{\nevent_id: EventId,\ntrigger_id: TriggerId,\ntimestamp: Timestamp,\ncontext: EventContext\n}"]
        end
        
        %% Infrastructure Layer (Adapters - Secondary/Outbound)
        subgraph TriggerOutboundInfra["Infrastructure Layer (Outbound)"]
            TriggerRepository["struct InMemoryTriggerRepository\nimplements TriggerRegistryPort"]
            ExecutionRequestPublisher["struct ExecutionRequestPublisher\nimplements ExecutionRequestPort"]
        end
    end
    
    %% ===== RUN MODULE =====
    subgraph "hexafn-run (Format & Function Phases)"
        %% Infrastructure Layer (Adapters - Primary/Inbound)
        subgraph RunInfra["Infrastructure Layer"]
            ExecutionSubscriber["struct ExecutionSubscriber\nimplements MessageConsumerPort"]
            FunctionLoader["struct FunctionLoader\nimplements FunctionLoaderPort"]
        end
        
        %% Application Layer (Ports and Use Cases)
        subgraph RunApp["Application Layer"]
            ExecutionUseCase["trait FunctionExecutionPort\nfn execute(&self, req: ExecutionRequest) -> Result<Output, Error>"]
            ContextPreparationService["struct ContextPreparationService\nfn prepare(&self, req: &ExecutionRequest) -> Result<ExecutionContext, Error>"]
            FormatService["struct FormatService\nfn format(&self, data: &Data) -> Result<FormattedData, Error>"]
            RuntimeSelectionService["struct RuntimeSelectionService\nfn select_runtime(&self, func: &Function) -> Box<dyn FunctionRuntime>"]
            ResultProcessingPort["trait ResultProcessingPort\nfn process(&self, result: FunctionResult) -> Result<ProcessedResult, Error>"]
        end
        
        %% Domain Layer (Core Business Logic)
        subgraph RunDomain["Domain Layer"]
            Function["struct Function\n{\nid: FunctionId,\nname: String,\nruntime_type: RuntimeType,\ncode: FunctionCode,\nschema: FunctionSchema\n}"]
            ExecutionContext["struct ExecutionContext\n{\nrequest_id: RequestId,\ninput: Input,\nenv: Environment,\ntrace_id: TraceId\n}"]
            RuntimeEnvironment["enum RuntimeType\n{\nDSL,\nWASM,\nJavaScript,\nLua\n}"]
            FunctionSchema["struct FunctionSchema\n{\ninput_schema: JsonSchema,\noutput_schema: JsonSchema\n}"]
            Validator["trait SchemaValidator\nfn validate(&self, data: &Value, schema: &JsonSchema) -> Result<(), ValidationError>"]
            FunctionExecutedEvent["struct FunctionExecutedEvent\n{\nevent_id: EventId,\nfunction_id: FunctionId,\nexecution_time: Duration,\nstatus: ExecutionStatus,\nresult: Option<Output>\n}"]
        end
        
        %% Infrastructure Layer (Adapters - Secondary/Outbound)
        subgraph RunOutboundInfra["Infrastructure Layer (Outbound)"]
            FunctionRepository["struct FilesystemFunctionRepository\nimplements FunctionRepositoryPort"]
            WASMRuntime["struct WasmtimeRuntime\nimplements FunctionRuntime"]
            DSLInterpreter["struct DSLInterpreter\nimplements FunctionRuntime"]
            JSRuntime["struct QuickJSRuntime\nimplements FunctionRuntime"]
            ResultPublisher["struct ResultPublisher\nimplements ResultProcessingPort"]
        end
    end
    
    %% ===== FORWARD PHASE =====
    subgraph "Forward Phase"
        %% ===== STORE MODULE =====
        subgraph "hexafn-store"
            %% Infrastructure Layer (Adapters - Primary/Inbound)
            subgraph StoreInfra["Infrastructure Layer"]
                StoreAPIController["struct KVStoreController\nimplements RESTController"]
                ResultSubscriber["struct ResultSubscriber\nimplements MessageConsumerPort"]
            end
            
            %% Application Layer (Ports and Use Cases)
            subgraph StoreApp["Application Layer"]
                KVStoreUseCase["trait KVStorePort\nfn set(&self, key: Key, value: Value) -> Result<(), Error>\nfn get(&self, key: &Key) -> Result<Option<Value>, Error>"]
                KeyValidationService["struct KeyValidationService\nfn validate_key(&self, key: &Key) -> Result<(), ValidationError>"]
                PersistencePort["trait PersistencePort\nfn store(&self, key: &Key, value: &Value) -> Result<(), Error>"]
                ChangeEventPort["trait ChangeEventPort\nfn emit_change(&self, key: &Key, old: Option<&Value>, new: &Value) -> Result<(), Error>"]
            end
            
            %% Domain Layer (Core Business Logic)
            subgraph StoreDomain["Domain Layer"]
                KVPair["struct KVPair\n{\nkey: Key,\nvalue: Value,\nversion: Version,\nttl: Option<Duration>\n}"]
                Key["struct Key\n{\nnamespace: Namespace,\npath: String\n}"]
                Value["enum Value\n{\nString(String),\nNumber(f64),\nBool(bool),\nObject(HashMap<String, Value>),\nArray(Vec<Value>),\nNull\n}"]
                Namespace["struct Namespace(String)"]
                VersionedEntry["struct VersionedEntry\n{\nkey: Key,\nvalue: Value,\nversion: u64,\ntimestamp: Timestamp\n}"]
                KVValidator["trait KVValidator\nfn validate(&self, pair: &KVPair) -> Result<(), ValidationError>"]
                KVUpdatedEvent["struct KVUpdatedEvent\n{\nkey: Key,\nold_value: Option<Value>,\nnew_value: Value,\nversion: u64,\ntimestamp: Timestamp\n}"]
            end
            
            %% Infrastructure Layer (Adapters - Secondary/Outbound)
            subgraph StoreOutboundInfra["Infrastructure Layer (Outbound)"]
                InMemoryStore["struct InMemoryStore\nimplements PersistencePort"]
                RocksDBStore["struct RocksDBStore\nimplements PersistencePort"]
                FileSystemStore["struct FileSystemStore\nimplements PersistencePort"]
                ChangePublisher["struct ChangePublisher\nimplements ChangeEventPort"]
            end
        end
        
        %% ===== CAST MODULE =====
        subgraph "hexafn-cast"
            %% Infrastructure Layer (Adapters - Primary/Inbound)
            subgraph CastInfra["Infrastructure Layer"]
                PublishController["struct PublishController\nimplements RESTController"]
                SubscriptionController["struct SubscriptionController\nimplements RESTController"]
                ResultForwarder["struct ResultForwarder\nimplements MessageConsumerPort"]
            end
            
            %% Application Layer (Ports and Use Cases)
            subgraph CastApp["Application Layer"]
                PublishUseCase["trait PublisherPort\nfn publish(&self, topic: &Topic, message: Message) -> Result<MessageId, Error>"]
                SubscribeUseCase["trait SubscriberPort\nfn subscribe(&self, topic: &Topic, callback: SubscriberCallback) -> Result<SubscriptionId, Error>"]
                TopicRoutingService["struct TopicRoutingService\nfn route(&self, message: &Message) -> Vec<Topic>"]
                MessageBusPort["trait MessageBusPort\nfn send(&self, topic: &Topic, message: &Message) -> Result<(), Error>"]
                DeliveryStatusPort["trait DeliveryStatusPort\nfn record_status(&self, msg_id: &MessageId, status: DeliveryStatus) -> Result<(), Error>"]
            end
            
            %% Domain Layer (Core Business Logic)
            subgraph CastDomain["Domain Layer"]
                Topic["struct Topic\n{\nname: String,\npartitions: u32\n}"]
                Message["struct Message\n{\nid: MessageId,\ntopic: Topic,\npayload: MessagePayload,\nheaders: MessageHeaders,\ntimestamp: Timestamp\n}"]
                Subscriber["struct Subscriber\n{\nid: SubscriberId,\nfilters: Vec<TopicFilter>,\ncallback: SubscriberCallback\n}"]
                TopicFilter["enum TopicFilter\n{\nExact(String),\nPrefix(String),\nPattern(Regex)\n}"]
                DeliveryPolicy["trait DeliveryPolicy\nfn apply(&self, message: &Message, subscriber: &Subscriber) -> DeliveryDecision"]
                MessageDeliveredEvent["struct MessageDeliveredEvent\n{\nmessage_id: MessageId,\nsubscriber_id: SubscriberId,\ndelivery_time: Duration,\nstatus: DeliveryStatus\n}"]
            end
            
            %% Infrastructure Layer (Adapters - Secondary/Outbound)
            subgraph CastOutboundInfra["Infrastructure Layer (Outbound)"]
                InMemoryBus["struct InMemoryBus\nimplements MessageBusPort"]
                RetryManager["struct RetryManager\nfn schedule_retry(&self, msg_id: &MessageId, attempt: u32) -> Result<(), Error>"]
                DeliveryLogger["struct DeliveryLogger\nimplements DeliveryStatusPort"]
            end
        end
    end
    
    %% ===== WATCH MODULE =====
    subgraph "hexafn-watch (Feedback Phase)"
        %% Infrastructure Layer (Adapters - Primary/Inbound)
        subgraph WatchInfra["Infrastructure Layer"]
            MetricsEndpoint["struct MetricsEndpoint\nimplements PrometheusExporter"]
            LogSubscriber["struct LogSubscriber\nimplements MessageConsumerPort"]
            TracingHook["struct TracingHook\nimplements InstrumentationHook"]
        end
        
        %% Application Layer (Ports and Use Cases)
        subgraph WatchApp["Application Layer"]
            LoggingUseCase["trait LoggingPort\nfn log(&self, level: LogLevel, message: &str, meta: &LogMeta) -> Result<(), Error>"]
            TracingUseCase["trait TracingPort\nfn create_span(&self, name: &str, parent: Option<SpanId>) -> Result<SpanId, Error>\nfn end_span(&self, id: &SpanId) -> Result<(), Error>"]
            MetricsUseCase["trait MetricsPort\nfn record(&self, name: &str, value: f64, tags: &MetricTags) -> Result<(), Error>"]
            AlertingService["struct AlertingService\nfn evaluate_conditions(&self) -> Vec<Alert>"]
            ObservabilityPort["trait ObservabilityPort\nfn export(&self, data: &ObservabilityData) -> Result<(), Error>"]
        end
        
        %% Domain Layer (Core Business Logic)
        subgraph WatchDomain["Domain Layer"]
            Trace["struct Trace\n{\ntrace_id: TraceId,\nname: String,\nstart_time: Timestamp,\nend_time: Option<Timestamp>,\nspans: Vec<Span>\n}"]
            Span["struct Span\n{\nspan_id: SpanId,\nparent_id: Option<SpanId>,\nname: String,\nstart_time: Timestamp,\nend_time: Option<Timestamp>,\nattributes: HashMap<String, Value>\n}"]
            LogEntry["struct LogEntry\n{\nid: LogId,\ntimestamp: Timestamp,\nlevel: LogLevel,\nmessage: String,\ncontext: LogContext,\ntrace_id: Option<TraceId>\n}"]
            MetricPoint["struct MetricPoint\n{\nname: String,\nvalue: f64,\ntimestamp: Timestamp,\ntags: HashMap<String, String>,\nmetric_type: MetricType\n}"]
            TraceCorrelator["trait TraceCorrelator\nfn correlate(&self, log: &LogEntry, trace: &Trace) -> Result<LogEntry, Error>"]
            AnomalyDetector["struct AnomalyDetector\nfn detect(&self, metrics: &[MetricPoint]) -> Vec<Anomaly>"]
            AlertTriggeredEvent["struct AlertTriggeredEvent\n{\nalert_id: AlertId,\nseverity: AlertSeverity,\nmessage: String,\ntimestamp: Timestamp,\ncontext: AlertContext\n}"]
        end
        
        %% Infrastructure Layer (Adapters - Secondary/Outbound)
        subgraph WatchOutboundInfra["Infrastructure Layer (Outbound)"]
            StructuredLogger["struct JsonLogger\nimplements LoggingPort"]
            PrometheusExporter["struct PrometheusClient\nimplements MetricsPort"]
            TracingExporter["struct OTelExporter\nimplements TracingPort"]
            AlertNotifier["struct AlertNotifier\nimplements NotificationPort"]
        end
    end
    
    %% External Output
    ExternalOutput[External Output/Response]
    
    %% Core Module (Influences Everything)
    subgraph "hexafn-core (Shared Kernel)"
        DomainEvents["trait DomainEvent\nfn event_type(&self) -> &'static str\nfn aggregate_id(&self) -> &str\nfn correlation_id(&self) -> &str"]
        SharedTraits["trait Identifiable\ntrait Timestamped\ntrait Serializable\ntrait Loggable"]
        Pipeline["struct Pipeline\nfn new() -> Self\nfn feed(impl Feed) -> Self\nfn filter(impl Filter) -> Self\nfn format(impl Format) -> Self\nfn function(impl Function) -> Self\nfn forward(impl Forward) -> Self\nfn feedback(impl Feedback) -> Self\nfn build() -> Result<Self, Error>\nfn execute() -> Result<PipelineResult, Error>"]
        ErrorHandling["enum AppError\nstruct ErrorContext\ntrait ErrorHandler"]
        
        %% 6F Lifecycle Traits
        subgraph CoreTraits["6F Lifecycle Traits"]
            FeedTrait["trait Feed<T>\nfn fetch(&self) -> Result<T, Error>"]
            FilterTrait["trait Filter<T>\nfn check(&self, input: &T) -> Result<bool, Error>"]
            FormatTrait["trait Format<Input, Output>\nfn transform(&self, input: Input) -> Result<Output, Error>"]
            FunctionTrait["trait Function<Context, Output>\nfn run(&self, context: Context) -> Result<Output, Error>"]
            ForwardTrait["trait Forward<T>\nfn send(&self, data: T) -> Result<(), Error>"]
            FeedbackTrait["trait Feedback\nfn record(&self, trace_id: &str, status: &str) -> Result<(), Error>"]
        end
    end
    
    %% Main Flow Connections
    ExternalEvent --> WebhookEndpoint
    
    %% Bridge to Trigger
    TopicRouter --> SubscriberAdapter
    
    %% Trigger to Run
    ExecutionRequestPublisher --> ExecutionSubscriber
    
    %% Run to Forward (Store & Cast)
    ResultPublisher --> ResultSubscriber
    ResultPublisher --> ResultForwarder
    
    %% Forward to Watch (both Store and Cast send to Watch)
    ChangePublisher --> LogSubscriber
    DeliveryLogger --> LogSubscriber
    
    %% Watch to External Output
    AlertNotifier --> ExternalOutput
    
    %% Alternative Output Paths
    TopicRouter --> ExternalOutput
    ResultPublisher --> ExternalOutput
    
    %% Core influences (simplified for clarity)
    DomainEvents -.-> BridgeDomain
    DomainEvents -.-> TriggerDomain
    DomainEvents -.-> RunDomain
    DomainEvents -.-> StoreDomain
    DomainEvents -.-> CastDomain
    DomainEvents -.-> WatchDomain
    
    FeedTrait -.-> BridgeApp
    FilterTrait -.-> TriggerApp
    FormatTrait -.-> RunApp
    FunctionTrait -.-> RunApp
    ForwardTrait -.-> StoreApp
    ForwardTrait -.-> CastApp
    FeedbackTrait -.-> WatchApp
    
    Pipeline -.-> BridgeApp
    Pipeline -.-> TriggerApp
    Pipeline -.-> RunApp
    Pipeline -.-> StoreApp
    Pipeline -.-> CastApp
    Pipeline -.-> WatchApp
    
    ErrorHandling -.-> BridgeInfra
    ErrorHandling -.-> TriggerInfra
    ErrorHandling -.-> RunInfra
    ErrorHandling -.-> StoreInfra
    ErrorHandling -.-> CastInfra
    ErrorHandling -.-> WatchInfra
    
    %% Styling
    classDef infrastructure fill:#f9d5e5,stroke:#333,stroke-width:1px;
    classDef application fill:#eeeeee,stroke:#333,stroke-width:1px;
    classDef domain fill:#d5e8f9,stroke:#333,stroke-width:1px;
    classDef external fill:#e0e0e0,stroke:#333,stroke-width:1px;
    classDef core fill:#f9f9d5,stroke:#333,stroke-width:1px;
    
    class BridgeInfra,BridgeOutboundInfra,TriggerInfra,TriggerOutboundInfra,RunInfra,RunOutboundInfra,StoreInfra,StoreOutboundInfra,CastInfra,CastOutboundInfra,WatchInfra,WatchOutboundInfra infrastructure;
    class BridgeApp,TriggerApp,RunApp,StoreApp,CastApp,WatchApp application;
    class BridgeDomain,TriggerDomain,RunDomain,StoreDomain,CastDomain,WatchDomain domain;
    class ExternalEvent,ExternalOutput external;
    class DomainEvents,SharedTraits,Pipeline,ErrorHandling,CoreTraits core;
```

## Architecture Explanation

This detailed component diagram shows:

1. **Complete Component Details**: All struct definitions, trait interfaces, methods, and connections.

2. **Hexagonal Architecture Layers**:
   - **Infrastructure Layer**: Handles external concerns and implements adapters
   - **Application Layer**: Contains use cases and port definitions
   - **Domain Layer**: Houses core business rules and models

3. **6F Lifecycle Organization**:
   - **Feed Phase**: Data ingestion (hexafn-bridge)
   - **Filter Phase**: Condition checking (hexafn-trigger)
   - **Format & Function Phases**: Data transformation and execution (hexafn-run)
   - **Forward Phase**: Result delivery (hexafn-store, hexafn-cast)
   - **Feedback Phase**: Observability and audit (hexafn-watch)

4. **Data Flow**: Event flow from external ingestion through the entire system.

5. **Core Module**: Shared kernel providing common traits, interfaces, and pipelines.

The architecture follows clean separation of concerns with domain-driven design principles while enabling flexible composition through the 6F Lifecycle Flow model.