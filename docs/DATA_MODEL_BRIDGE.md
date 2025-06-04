<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Bridge Module (HexaBridge)

## Domain Layer
```mermaid
classDiagram
    class Integration {
        <<trait>>
        +connect() Result~ _ , HexaError~
        +disconnect() Result~ _ , HexaError~
        +is_connected() bool
        +get_name() String
        +get_status() ConnectionStatus
    }
    class Webhook {
        <<trait>>
        +endpoint() String
        +method() HttpMethod
        +headers() HashMap~String, String~
        +send(payload: Any) Result~Response, HexaError~
        +set_timeout(duration: Duration) void
    }
    class EventTransformer {
        <<trait>>
        +transform(input: Any) Result~Any, HexaError~
        +get_schema() Option~Schema~
        +validate_input(input: Any) Result~ _ , HexaError~
        +validate_output(output: Any) Result~ _ , HexaError~
    }
    class BridgeEvent {
        <<trait>>
        +event_type() &'static str
        +payload() serde_json::Value
    }
    class WebhookReceivedEvent {
        +webhook_id String
        +timestamp DateTime
        +payload serde_json::Value
    }
    class IntegrationRegisteredEvent {
        +integration_id String
        +timestamp DateTime
        +payload serde_json::Value
    }
    class ConnectionStatus {
        <<enumeration>>
        Connected
        Disconnected
        Error
    }
    class HttpMethod {
        <<enumeration>>
        GET
        POST
        PUT
        DELETE
        PATCH
    }
    class Response {
        +status u16
        +body serde_json::Value
        +headers HashMap~String, String~
    }
    Integration --> EventTransformer
    Webhook --|> Integration
    Webhook --> HttpMethod
    Webhook --> Response
    BridgeEvent <|-- WebhookReceivedEvent
    BridgeEvent <|-- IntegrationRegisteredEvent
    %% Additional missing domain structures from output.txt
    class BridgeAuditTrail {
        +bridge_id String
        +events Vec~BridgeAuditEvent~
    }
    class BridgeLock {
        +acquire(id: String) Result~_, HexaError~
        +release(id: String) Result~_, HexaError~
        +is_locked(id: String) bool
    }
    class BridgeDependencyGraph {
        +add_dependency(from: String, to: String)
        +remove_dependency(from: String, to: String)
        +get_dependencies(id: String) Vec~String~
    }
    class BridgeRollbackPoint {
        +bridge_id String
        +stage_index u32
        +state serde_json::Value
    }
    class BridgeTag {
        +name String
        +color String
    }
    class BridgeAccessControl {
        +bridge_id String
        +allowed_roles Vec~String~
        +is_allowed(user: String) bool
    }
    class IntegrationHealthCheck {
        +integration_id String
        +status String
        +last_checked DateTime
    }
    class IntegrationRetryPolicy {
        +integration_id String
        +max_retries u32
        +backoff_strategy String
    }
    class IntegrationCircuitBreaker {
        +integration_id String
        +state String
        +failure_count u32
        +reset_timeout Duration
    }
    class IntegrationRateLimit {
        +integration_id String
        +limit u32
        +window Duration
    }
    class IntegrationTransformationTemplate {
        +integration_id String
        +template String
    }
    class IntegrationApprovalWorkflow {
        +integration_id String
        +steps Vec~String~
        +status String
    }
    class IntegrationSecretManager {
        +integration_id String
        +secrets HashMap~String, String~
    }
    class IntegrationAuditTrail {
        +integration_id String
        +calls Vec~IntegrationCallEvent~
    }
```

## Application Layer
```mermaid
classDiagram
    class IntegrationManagementPort {
        <<port>>
        +register_integration(config: IntegrationConfig) Result~_, HexaError~
        +remove_integration(id: String) Result~_, HexaError~
        +list_integrations() Vec~Integration~
    }
    class WebhookReceiverPort {
        <<port>>
        +receive_webhook(id: String, payload: serde_json::Value) Result~_, HexaError~
    }
    class BridgeService {
        +normalize_payload(payload: serde_json::Value) Result~serde_json::Value, HexaError~
        +dispatch_event(event: BridgeEvent) Result~_, HexaError~
        +list_integrations() Vec~Integration~
    }
    class IntegrationConfigLoader {
        +load_from_file(path: String) Result~Vec~IntegrationConfig~, HexaError~
    }
    class IntegrationConfig {
        +id String
        +type String
        +endpoint String
        +headers HashMap~String, String~
        +token Option~String~
    }
    class BridgeCommand {
        <<command>>
        +RegisterIntegration
        +RemoveIntegration
        +SimulateWebhook
    }
    class BridgeQuery {
        <<query>>
        +ListIntegrations
        +GetIntegration
    }
    IntegrationManagementPort --> Integration
    WebhookReceiverPort --> Webhook
    BridgeService --> IntegrationManagementPort
    BridgeService --> WebhookReceiverPort
    IntegrationConfigLoader --> IntegrationConfig
    %% Additional missing application structures from output.txt
    class BridgeAuditService {
        +record_event(bridge_id: String, event: BridgeAuditEvent)
        +get_audit_trail(bridge_id: String) BridgeAuditTrail
    }
    class BridgeLockService {
        +lock(bridge_id: String) Result~_, HexaError~
        +unlock(bridge_id: String) Result~_, HexaError~
        +is_locked(bridge_id: String) bool
    }
    class BridgeDependencyService {
        +resolve_dependencies(bridge_id: String) Vec~String~
    }
    class BridgeRollbackService {
        +create_rollback_point(bridge_id: String, stage_index: u32)
        +rollback_to_point(bridge_id: String, point: BridgeRollbackPoint)
    }
    class BridgeTaggingService {
        +add_tag(bridge_id: String, tag: BridgeTag)
        +remove_tag(bridge_id: String, tag: BridgeTag)
        +list_tags(bridge_id: String) Vec~BridgeTag~
    }
    class BridgeAccessControlService {
        +grant_access(bridge_id: String, user: String)
        +revoke_access(bridge_id: String, user: String)
        +check_access(bridge_id: String, user: String) bool
    }
```

## Infrastructure Layer
```mermaid
classDiagram
    class IntegrationRepository {
        +save(integration: Integration) Result~_, HexaError~
        +load(id: String) Result~Option~Integration~, HexaError~
        +list() Result~Vec~Integration~, HexaError~
    }
    class BridgeEventBus {
        +publish(event: BridgeEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(BridgeEvent))
    }
    class BridgeCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +simulate_webhook(id: String, payload: serde_json::Value) Result~_, HexaError~
    }
    class IntegrationDto {
        +id String
        +type String
        +endpoint String
        +headers HashMap~String, String~
    }
    class BridgeMapper {
        +to_dto(integration: Integration) IntegrationDto
        +from_dto(dto: IntegrationDto) Integration
    }
    IntegrationRepository --> Integration
    BridgeEventBus --> BridgeEvent
    BridgeMapper --> IntegrationDto
    BridgeMapper --> Integration
    %% Additional missing infrastructure structures from output.txt
    class BridgeAuditRepository {
        +save(trail: BridgeAuditTrail)
        +load(bridge_id: String) Option~BridgeAuditTrail~
    }
    class BridgeLockManager {
        +acquire_lock(bridge_id: String)
        +release_lock(bridge_id: String)
        +is_locked(bridge_id: String) bool
    }
    class BridgeDependencyAdapter {
        +fetch_dependencies(bridge_id: String) Vec~String~
    }
    class BridgeRollbackAdapter {
        +save_point(point: BridgeRollbackPoint)
        +load_points(bridge_id: String) Vec~BridgeRollbackPoint~
    }
    class BridgeTagStore {
        +add(bridge_id: String, tag: BridgeTag)
        +remove(bridge_id: String, tag: BridgeTag)
        +list(bridge_id: String) Vec~BridgeTag~
    }
    class BridgeAccessControlAdapter {
        +set_access(bridge_id: String, user: String, allowed: bool)
        +get_access(bridge_id: String, user: String) bool
    }
```

---

