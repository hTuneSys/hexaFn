<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Cast Module (HexaCast)

## Domain Layer
```mermaid
classDiagram
    class Topic {
        +name String
        +description String
        +created_at DateTime
        +retention_policy RetentionPolicy
        +new(name: String) Result~Topic, HexaError~
        +with_description(description: String) Topic
        +is_valid() bool
    }
    class RetentionPolicy {
        +max_events u64
        +ttl Option~Duration~
    }
    class Publisher {
        <<trait>>
        +publish(topic: Topic, message: Any) Result~ _ , HexaError~
        +publish_batch(topic: Topic, messages: Vec~Any~) Result~ _ , HexaError~
        +get_published_count() u64
    }
    class Subscriber {
        <<trait>>
        +subscribe(topic: Topic, callback: fn(Any)) Result~ _ , HexaError~
        +unsubscribe(topic: Topic) Result~ _ , HexaError~
        +get_subscriptions() Vec~Topic~
        +is_subscribed(topic: Topic) bool
        +acknowledge(message_id: String) Result~_, HexaError~
        +replay(topic: Topic, from: DateTime) Result~_, HexaError~
        +lifecycle_state() SubscriberLifecycle
    }
    class SubscriberLifecycle {
        <<enumeration>>
        Active
        Inactive
        Pending
        Error
    }
    class Message {
        +id String
        +topic String
        +payload serde_json::Value
        +timestamp DateTime
        +metadata HashMap~String, String~
        +delivery_status DeliveryStatus
    }
    class DeliveryStatus {
        <<enumeration>>
        Pending
        Delivered
        Failed
        Retrying
    }
    class MessageDeliveredEvent {
        +message_id String
        +subscriber_id String
        +timestamp DateTime
        +status DeliveryStatus
    }
    class TopicCreatedEvent {
        +topic_name String
        +timestamp DateTime
    }
    class TopicRetentionPolicyChangedEvent {
        +topic_name String
        +old_policy RetentionPolicy
        +new_policy RetentionPolicy
        +timestamp DateTime
    }
    Publisher --> Topic
    Subscriber --> Topic
    Subscriber --> Message
    Message --> DeliveryStatus
    MessageDeliveredEvent --> Message
    TopicCreatedEvent --> Topic
    TopicRetentionPolicyChangedEvent --> Topic
```

## Application Layer
```mermaid
classDiagram
    class TopicManagementPort {
        <<port>>
        +create_topic(config: TopicConfig) Result~_, HexaError~
        +delete_topic(name: String) Result~_, HexaError~
        +set_retention(name: String, policy: RetentionPolicy) Result~_, HexaError~
        +list_topics() Vec~Topic~
    }
    class PublisherPort {
        <<port>>
        +publish(topic: String, message: serde_json::Value) Result~_, HexaError~
        +publish_batch(topic: String, messages: Vec~serde_json::Value~) Result~_, HexaError~
    }
    class SubscriberPort {
        <<port>>
        +subscribe(topic: String, filter: Option~TopicFilter~) Result~_, HexaError~
        +unsubscribe(topic: String) Result~_, HexaError~
        +list_subscribers(topic: String) Vec~Subscriber~
    }
    class CastService {
        +register_subscriber(subscriber: Subscriber) Result~_, HexaError~
        +remove_subscriber(id: String) Result~_, HexaError~
        +list_topics() Vec~Topic~
        +list_subscribers() Vec~Subscriber~
        +publish_message(topic: String, message: serde_json::Value) Result~_, HexaError~
        +replay_messages(topic: String, from: DateTime) Result~_, HexaError~
    }
    class TopicConfigLoader {
        +load_from_file(path: String) Result~Vec~TopicConfig~, HexaError~
    }
    class TopicConfig {
        +name String
        +description String
        +retention_policy RetentionPolicy
    }
    class TopicFilter {
        +field String
        +value serde_json::Value
        +operator String
    }
    class CastCommand {
        <<command>>
        +CreateTopic
        +DeleteTopic
        +SetRetention
        +ReplayMessages
    }
    class CastQuery {
        <<query>>
        +ListTopics
        +ListSubscribers
    }
    class CastValidator {
        +validate_topic(config: TopicConfig) Result~_, HexaError~
    }
    TopicManagementPort --> Topic
    PublisherPort --> Topic
    SubscriberPort --> Topic
    CastService --> TopicManagementPort
    CastService --> PublisherPort
    CastService --> SubscriberPort
    TopicConfigLoader --> TopicConfig
    CastValidator --> TopicConfig
```

## Infrastructure Layer
```mermaid
classDiagram
    class CastRepository {
        +save_topic(topic: Topic) Result~_, HexaError~
        +load_topic(name: String) Result~Option~Topic~, HexaError~
        +list_topics() Result~Vec~Topic~, HexaError~
        +save_subscriber(subscriber: Subscriber) Result~_, HexaError~
        +list_subscribers() Result~Vec~Subscriber~, HexaError~
    }
    class CastEventBus {
        +publish(event: MessageDeliveredEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(MessageDeliveredEvent))
    }
    class CastCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +list_topics() Result~Vec~Topic~, HexaError~
        +list_subscribers() Result~Vec~Subscriber~, HexaError~
    }
    class CastDto {
        +topic String
        +message String
        +timestamp DateTime
        +metadata HashMap~String, String~
    }
    class CastMapper {
        +to_dto(message: Message) CastDto
        +from_dto(dto: CastDto) Message
    }
    class OutputForwarder {
        <<trait>>
        +forward(output: serde_json::Value, target: OutputTarget) Result~_, HexaError~
    }
    class OutputTarget {
        <<enumeration>>
        Http
        Cast
        File
        Kafka
        WebSocket
    }
    CastRepository --> Topic
    CastRepository --> Subscriber
    CastEventBus --> MessageDeliveredEvent
    CastMapper --> CastDto
    CastMapper --> Message
    OutputForwarder --> OutputTarget
```

---
