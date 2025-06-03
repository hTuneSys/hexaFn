<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# Store Module (HexaStore)

## Domain Layer
```mermaid
classDiagram
    class KvStore {
        <<trait>>
        +get(namespace: Namespace, key: String) Result~Option~KeyValueEntry~, HexaError~
        +put(namespace: Namespace, key: String, value: KeyValueEntry) Result~ _ , HexaError~
        +delete(namespace: Namespace, key: String) Result~ _ , HexaError~
        +list(namespace: Namespace, prefix: String) Result~Vec~KeyValueEntry~, HexaError~
        +exists(namespace: Namespace, key: String) Result~bool, HexaError~
        +transaction(ops: Vec~KvOp~) Result~_, HexaError~
    }
    class Namespace {
        +value String
        +new(name: String) Result~Namespace, HexaError~
        +to_string() String
        +is_valid() bool
    }
    class KeyValueEntry {
        +key String
        +value Vec~u8~
        +metadata HashMap~String, String~
        +created_at DateTime
        +updated_at DateTime
        +version u64
        +ttl Option~Duration~
    }
    class KvOp {
        <<enumeration>>
        Put
        Delete
    }
    class StoreEntryCreatedEvent {
        +namespace String
        +key String
        +timestamp DateTime
    }
    class StoreEntryUpdatedEvent {
        +namespace String
        +key String
        +timestamp DateTime
    }
    class StoreEntryDeletedEvent {
        +namespace String
        +key String
        +timestamp DateTime
    }
    KvStore --> Namespace
    KvStore --> KeyValueEntry
    KvStore --> KvOp
    StoreEntryCreatedEvent --> KeyValueEntry
    StoreEntryUpdatedEvent --> KeyValueEntry
    StoreEntryDeletedEvent --> KeyValueEntry
```

## Application Layer
```mermaid
classDiagram
    class KvStorePort {
        <<port>>
        +get(namespace: String, key: String) Result~Option~KeyValueEntry~, HexaError~
        +put(namespace: String, key: String, value: Vec~u8~) Result~_, HexaError~
        +delete(namespace: String, key: String) Result~_, HexaError~
        +list(namespace: String, prefix: String) Result~Vec~KeyValueEntry~, HexaError~
        +transaction(ops: Vec~KvOp~) Result~_, HexaError~
    }
    class StoreService {
        +backup() Result~_, HexaError~
        +restore(archive: Vec~u8~) Result~_, HexaError~
        +validate_entry(entry: KeyValueEntry) Result~_, HexaError~
        +hot_reload() Result~_, HexaError~
        +set_ttl(namespace: String, key: String, ttl: Duration) Result~_, HexaError~
    }
    class StoreCommand {
        <<command>>
        +BackupStore
        +RestoreStore
        +HotReload
        +SetTTL
    }
    class StoreQuery {
        <<query>>
        +ListNamespaces
        +GetEntry
    }
    class StoreValidator {
        +validate_schema(entry: KeyValueEntry) Result~_, HexaError~
    }
    class StoreConfigLoader {
        +load_from_file(path: String) Result~Vec~StoreConfig~, HexaError~
    }
    class StoreConfig {
        +namespaces Vec~NamespaceConfig~
    }
    class NamespaceConfig {
        +name String
        +ttl Option~Duration~
    }
    KvStorePort --> KeyValueEntry
    StoreService --> KvStorePort
    StoreConfigLoader --> StoreConfig
    StoreValidator --> KeyValueEntry
```

## Infrastructure Layer
```mermaid
classDiagram
    class StoreRepository {
        +save(entry: KeyValueEntry) Result~_, HexaError~
        +load(namespace: String, key: String) Result~Option~KeyValueEntry~, HexaError~
        +list(namespace: String) Result~Vec~KeyValueEntry~, HexaError~
        +delete(namespace: String, key: String) Result~_, HexaError~
    }
    class StoreEventBus {
        +publish(event: StoreEntryCreatedEvent) Result~_, HexaError~
        +subscribe(event_type: String, handler: fn(StoreEntryCreatedEvent))
    }
    class StoreCli {
        +run(args: Vec~String~) Result~_, HexaError~
        +backup() Result~_, HexaError~
        +restore() Result~_, HexaError~
    }
    class StoreDto {
        +namespace String
        +key String
        +value String
        +metadata HashMap~String, String~
    }
    class StoreMapper {
        +to_dto(entry: KeyValueEntry) StoreDto
        +from_dto(dto: StoreDto) KeyValueEntry
    }
    StoreRepository --> KeyValueEntry
    StoreEventBus --> StoreEntryCreatedEvent
    StoreMapper --> StoreDto
    StoreMapper --> KeyValueEntry
```

---
