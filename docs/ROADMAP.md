<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# ROADMAP.md

This document outlines the sprint-based development process for the hexaFn project. Each milestone consists of 3 sprints, each with a specific theme. Sprints build functional modules incrementally to ensure the system's overall integrity.

---

## Milestone 1: Establish Trigger → Run Flow Using DSL

### Sprint 1 – Basic Trigger and DSL Foundation  
Set up the basic structure for the trigger and function execution system. Design an internal DSL and implement a simple DSL interpreter. Establish the Trigger → Run flow where a trigger can execute a function defined in DSL.

### Sprint 2 – Configuration-Based Trigger Management  
Enable loading of triggers from configuration files. Add a registry system and error handling for triggers. Introduce function context and CLI support for trigger reloading.

### Sprint 3 – Advanced Flow and Fault Tolerance  
Integrate features such as timeout handling, fallback functions, and trigger disabling. Improve CLI error output, stress test the system, and initiate logging for observability.

---

## Milestone 2: Webhook Integration and Event Broadcasting

### Sprint 1 – Webhook Input and Basic Broadcasting  
Accept incoming webhook events from external systems, normalize them, and publish within the internal system. Implement HTTP endpoints, a basic Cast module, and essential security measures.

### Sprint 2 – Robust Pub-Sub System and Subscription Management  
Make the Cast module asynchronous, support multi-subscriber and topic filtering. Ensure reliable event delivery and enhance visibility via CLI. Conduct performance testing.

### Sprint 3 – Observability and Advanced Features  
Make event delivery observable, add a metrics endpoint, implement replay and acknowledgment features, and support manual publishing via CLI.

---

## Milestone 3: Observability, Logging, and Event Tracing

### Sprint 1 – Tracing and Structured Logging Foundation  
Assign `trace_id` to all events and track transitions across modules. Produce structured log output and make it accessible through a system CLI tool.

### Sprint 2 – External System Integration and Metrics  
Export trace data to external systems via OpenTelemetry. Provide Prometheus-compatible metrics, forward logs externally, and add dynamic log level controls.

### Sprint 3 – Advanced Tracing and Live Monitoring  
Detect span timeouts, support custom labeling and sampling strategies. Add live WebSocket log streaming and enable full event tracking via CLI.

---

## Milestone 4: KV Storage and Configuration Persistence

### Sprint 1 – KV Interface and In-Memory Storage  
Define a `KvStore` trait supporting CRUD operations and build the first in-memory implementation. Add JSON-based storage, namespace structure, and version tracking.

### Sprint 2 – Persistent Storage and Validation  
Introduce disk-based storage (JSON file format). Implement schema validation for KV entries, enable hot-reload of configurations. Add backup/restore tools and ACL controls.

### Sprint 3 – Advanced Backend and Security Features  
Integrate RocksDB. Support backend switching and data migration. Add audit logs for key changes, encrypted storage, and change watchers.

---

## Milestone 5: Runtime Support for WASM/JS/Lua Execution

### Sprint 1 – Multi-Runtime Architecture  
Support function execution with DSL, WASM (Wasmtime), JS (QuickJS), and Lua (rlua/mlua). Provide a common `FunctionRuntime` interface and test with sample functions.

### Sprint 2 – Feature Parity and Operational Stability  
Add input validation, custom configurations, fallback support, and error tolerance for all runtimes. Support isolated parallel execution of functions.

### Sprint 3 – Monitoring, Feedback, and Security  
Measure CPU/memory usage per execution, pass environment variables, and forward outputs to feedback channels. Standardize error messages and document with examples.

---

## Milestone 6: Output Forwarding and External Delivery

### Sprint 1 – Basic Forwarding Infrastructure  
Enable function outputs to be forwarded via HTTP, Cast, and filesystem. Extend output formatting and allow delivery configuration via files.

### Sprint 2 – Reliability and New Targets  
Introduce retries, exponential backoff, conditional delivery, and a Kafka forwarder. Handle delivery timeouts and apply redaction mechanisms for security.

### Sprint 3 – Parallel Delivery and Resilience  
Allow outputs to be sent to multiple targets simultaneously. Support live output streaming via WebSocket. Use circuit breakers to temporarily disable failed targets. Monitor target status via CLI.

---

## 📊 Live Project Tracking

- GitHub Project Board: [BOARD](https://github.com/orgs/hTuneSys/projects/15/views/1)
- Milestone Progress: [MILESTONES](https://github.com/hTuneSys/hexaFn/milestones)
- GitHub Issues: [ISSUES](https://github.com/hTuneSys/hexaFn/issues)
