<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📋 TODO List - hexaFn Project

This document tracks the development progress of the hexaFn project organized by milestones and sprints. Each item represents a specific task or feature aligned with the **6F Lifecycle Flow** and **Hexagonal Architecture** principles.

---

## 📊 Status Summary (as of May 25, 2025)
- **Total Issues**: 375
- **Completed**: 3
- **Open**: 372

---

## 🚀 Milestone 1: Establish Trigger → Run Flow Using DSL

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 447 | Initialize project structure and base modules | OPEN | Set up the initial Rust project with Cargo and create the base directory structure for core and modules (core pipeline, trigger engine, run engine, etc.) according to the planned architecture. |
| 448 | Design the internal DSL for function logic | OPEN | Define the syntax and capabilities of the internal domain-specific language (DSL) used for writing function logic in HexaRun. |
| 449 | Implement basic DSL parser and executor | OPEN | Develop a minimal parser and interpreter for the internal DSL to execute function logic. |
| 450 | Define HexaTrigger trait and event model | OPEN | Design and implement the core 'Trigger' trait along with an event/context model that triggers will evaluate. |
| 451 | Implement a basic value-based trigger | OPEN | Create a simple concrete trigger implementation (e.g., a 'ValueTrigger') that fires based on a condition in the event data. |
| 452 | Design HexaRun trait for function execution | OPEN | Define an abstraction (trait) in the HexaRun module (e.g., 'FunctionRunner') that specifies how to execute a function's logic. |
| 453 | Implement DSL-based function runner | OPEN | Implement a concrete FunctionRunner that uses the internal DSL interpreter to run function logic. |
| 454 | Implement trigger-to-function execution flow | OPEN | Connect the HexaTrigger and HexaRun modules to complete the Trigger → Run pipeline. |
| 455 | Create unit tests for HexaTrigger and HexaRun components | OPEN | Write unit tests to thoroughly cover the new trigger and runner logic. |
| 456 | Add integration test for trigger-to-run pipeline | OPEN | Develop an integration test to verify the complete trigger-to-function flow. |
| 457 | Add configuration loader for trigger definitions | OPEN | Develop a loader that reads trigger definitions from external configuration files. |
| 458 | Enable multiple trigger instances from config | OPEN | Extend the trigger system to support loading and managing multiple trigger definitions at runtime from a single config file. |
| 459 | Implement trigger registry for runtime access | OPEN | Create a global or orchestrator-level registry that stores active trigger instances and enables runtime querying, modification, or inspection. |
| 460 | Improve error handling in trigger evaluation | OPEN | Enhance error management within trigger evaluation logic to catch invalid payloads or rule mismatches. |
| 461 | Support compound trigger rules (AND/OR expressions) | OPEN | Expand the DSL or internal representation to allow compound conditions like AND, OR, NOT for more expressive rule building. |
| 462 | Design function invocation context structure | OPEN | Introduce a standard 'ExecutionContext' struct that will be passed from the trigger system to function runners. |
| 463 | Add CLI command to reload triggers from config file | OPEN | Implement a CLI utility that reloads triggers during runtime by reading the updated config file. |
| 464 | Implement dynamic trigger-function binding | OPEN | Allow config files to specify which function should be executed by each trigger. |
| 465 | Write integration test: reload config and execute | OPEN | Create a test case that simulates updating the trigger config file, reloads it, and verifies that the new trigger correctly invokes a DSL function. |
| 466 | Write unit tests for compound condition parser | OPEN | Test parsing of compound trigger rules (AND, OR, NOT). |
| 467 | Add validation for function runner output types | OPEN | Ensure that all outputs from FunctionRunner implementations conform to expected types and format. |
| 468 | Support fallback execution if function fails | OPEN | Allow defining fallback functions in config in case the primary function fails. |
| 469 | Implement trigger deactivation and timeouts | OPEN | Add functionality to temporarily or permanently disable a trigger via config or runtime control. |
| 470 | Add structured error messages to CLI output | OPEN | Improve CLI ergonomics by displaying structured error information when triggers or functions fail. |
| 471 | Refactor orchestrator to support trigger priorities | OPEN | Modify the core orchestrator so that triggers can be assigned execution priority. |
| 472 | Log trigger and function execution metadata | OPEN | Implement a logging module that records metadata such as trigger ID, function ID, timestamp, status, and runtime duration. |
| 473 | Create example use cases for common triggers | OPEN | Document and provide sample config files showcasing real-world trigger scenarios. |
| 474 | Implement dry-run mode for DSL functions via CLI | OPEN | Add a --dry-run flag to the CLI to simulate execution of a DSL function without invoking side effects. |
| 475 | Ensure config hot-reload maintains state consistency | OPEN | Validate that when configs are reloaded at runtime, no orphaned or duplicate trigger instances remain. |
| 476 | Add stress test for trigger → run performance | OPEN | Simulate high event throughput and measure latency of end-to-end trigger → run execution. |
| 477 | Review and refactor trigger evaluation engine | OPEN | Conduct a full code review of the trigger evaluation logic to identify complexity, duplication, or readability issues. |
| 478 | Validate full Trigger → Run flow against production scenarios | OPEN | Design and test production-like scenarios to validate the robustness of the Trigger → Run flow. |
| 479 | Finalize documentation for Trigger, Run, and DSL interfaces | OPEN | Complete and polish developer-facing docs for all public structs, traits, and config structures. |
| 480 | Conduct API consistency and naming review | OPEN | Review all exposed APIs, function names, and config keys in Trigger and Run modules to ensure consistent naming. |
| 481 | Prepare milestone 1 release checklist and changelog | OPEN | Summarize completed features, refactors, and fixes in a detailed changelog. |
| 290 | Initialize roadmap in ROADMAP.md | OPEN | Draft phases and delivery plan for the initial release. |
| 285 | Set up README Quick Start section | OPEN | Enable users to get up and running quickly via concise setup instructions. |
| 320 | Define color palette and font family in branding.md | OPEN | Maintain design consistency and visual harmony across documentation. |
| 319 | Add favicon and social share images for docs site | OPEN | Support browser tab visibility and social sharing with branded assets. |
| 318 | Display project logo prominently in README and docs | CLOSED | Strengthen branding and improve recognizability across channels. |
| 282 | Add project logo and favicon to documentation | CLOSED | Include a logo and favicon in the README and documentation site to strengthen branding and improve visual identity. |
| 339 | Create a comprehensive and visually appealing README | CLOSED | Structure README with clear sections, visuals, and quick start guidance to leave a strong impression. |

## 🌐 Milestone 2: Webhook Integration and Event Broadcasting

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 482 | Design and implement incoming webhook endpoint | OPEN | Create an HTTP POST endpoint to receive external events (e.g., webhooks). |
| 483 | Define external event schema for bridge input | OPEN | Specify the structure and required fields for incoming events accepted by the Bridge module. |
| 484 | Implement event normalization logic in Bridge | OPEN | Add a normalization layer that maps raw webhook payloads into standard Event structs used internally. |
| 485 | Support dynamic topic routing from webhook payload | OPEN | Enable webhook payloads to specify the topic or channel where the event should be published. |
| 486 | Implement in-memory pub-sub system in Cast | OPEN | Create a lightweight in-memory topic-based publish/subscribe engine. |
| 487 | Add logging for received and published events | OPEN | Log every received event (at the Bridge) and every published event (at Cast). |
| 488 | Secure webhook endpoint with token verification | OPEN | Protect the public webhook endpoint using shared secret tokens or header-based verification. |
| 489 | Create CLI utility to simulate webhook input | OPEN | Develop a CLI tool that allows developers to simulate sending webhook events to the local Bridge endpoint for testing. |
| 490 | Write integration test: webhook → cast pipeline | OPEN | Test the full pipeline from webhook reception to event broadcast through Cast. |
| 491 | Document webhook configuration options | OPEN | Write documentation explaining how users can register and configure webhook sources. |
| 492 | Implement asynchronous message queue for Cast engine | OPEN | Introduce an internal async message queue to decouple webhook reception from downstream processing. |
| 493 | Enable multiple subscriber support per topic | OPEN | Extend the Cast module to support multiple independent subscribers per topic. |
| 494 | Develop subscriber registration and lifecycle management | OPEN | Implement subscriber registration APIs and maintain lifecycle state (active/inactive). |
| 495 | Add support for topic filters on event payload | OPEN | Allow subscribers to define filters (e.g., match conditions on payload fields). |
| 496 | Log event delivery status per subscriber | OPEN | Track and log delivery outcomes for each event and subscriber pair. |
| 497 | Design Cast subscriber trait and interface | OPEN | Define a standard trait (e.g., 'Subscriber') that each module or function can implement to receive events. |
| 498 | Simulate burst event scenario for Cast performance test | OPEN | Inject a large number of webhook events in a short period (burst load) and verify the Cast engine's throughput and latency. |
| 499 | Add retry mechanism for failed event deliveries | OPEN | If a subscriber fails to process an event, implement retry logic with exponential backoff and max-attempt limits. |
| 500 | Document Cast pub-sub architecture and usage examples | OPEN | Create architecture diagrams and documentation showing how the pub-sub system works internally. |
| 501 | Add CLI command to list current topics and subscribers | OPEN | Provide a CLI utility that queries and displays all active topics and their current subscribers. |
| 502 | Add metrics endpoint to expose event stats | OPEN | Expose a metrics endpoint that publishes key stats such as received webhooks and published events. |
| 503 | Implement topic retention policy | OPEN | Allow configuration of message retention policies for topics, such as max event count or time-to-live (TTL). |
| 504 | Add subscriber acknowledgment support | OPEN | Require subscribers to explicitly acknowledge receipt of events. |
| 505 | Implement event replay for debugging purposes | OPEN | Allow replaying recent events to a specific subscriber (or all) for debugging or recovery. |
| 506 | Refactor Cast engine for better concurrency | OPEN | Improve internal architecture of the Cast module to better support high-throughput concurrent publishing and delivery. |
| 507 | Add CLI tool to send events to Cast manually | OPEN | Provide a CLI command to manually publish events to a specified topic. |
| 508 | Create visualization of topic flow and subscribers | OPEN | Generate a visual map (e.g., DOT graph or HTML) of active topics, subscribers, and connections. |
| 509 | Stress test subscriber filters under load | OPEN | Evaluate the performance impact of complex topic filters during high-load scenarios. |
| 510 | Write documentation for retry and ack system | OPEN | Document how retry and acknowledgment works for event deliveries. |
| 511 | Document security practices for webhook input | OPEN | Create guidelines for securely handling webhooks. |
| 512 | Review and optimize Bridge payload parsing logic | OPEN | Conduct a review of how webhook payloads are parsed and normalized in the Bridge module. |
| 513 | Audit Cast delivery guarantees and retry logic | OPEN | Verify the correctness and reliability of event delivery and retry logic in the Cast engine. |
| 514 | Document webhook integration lifecycle with examples | OPEN | Write comprehensive documentation for the webhook-to-subscriber event lifecycle. |
| 515 | Consolidate and clean up Cast module API surface | OPEN | Review the public API of the Cast module and refactor it for consistency and clarity. |
| 516 | Publish milestone changelog and close release checklist | OPEN | Write and publish a changelog summarizing all changes introduced in Milestone 2. |
| 340 | Write comprehensive Getting Started guide with examples | OPEN | Create a step-by-step onboarding guide for new users. |
| 342 | Develop FAQ and Troubleshooting documents | OPEN | Address common problems and frequently asked questions. |
| 344 | Adopt Code of Conduct policy | OPEN | Publish an inclusive and respectful participation guideline to establish a healthy project culture. |
| 345 | Standardize issue and PR templates | OPEN | Use YAML-based templates to ensure all contributions include necessary context. |
| 283 | Create CONTRIBUTING.md guidelines | OPEN | Provide a clear contributing guide for developers with PR, commit, and code review practices. |
| 286 | Integrate GitHub issue templates | OPEN | Define templates for bug, feature, and support requests using YAML files. |
| 296 | Add code review and PR merge process | OPEN | Set expectations for how and when contributions are reviewed and merged. |
| 302 | Set up real-time communication via Discord or Slack | OPEN | Facilitate direct interaction and foster a sense of community with real-time support. |
| 343 | Establish and improve CONTRIBUTING.md guide | OPEN | Define clear contribution rules including code standards, testing requirements, and review processes. |
| 341 | Generate API documentation and architecture diagrams | OPEN | Provide a complete API reference and architectural schematics to explain internal design. |
| 383 | Add .editorconfig for consistent formatting | OPEN | Ensure all developers use the same indentation and line ending style across editors. |
| 384 | Include .env.example for environment variables | OPEN | Provide a sample configuration file to clarify required ENV vars while keeping secrets private. |
| 385 | Add .nvmrc or .tool-versions for version locking | OPEN | Specify Node.js or other runtime versions to avoid incompatibilities across development environments. |
| 386 | Create Makefile or justfile for common dev tasks | OPEN | Simplify setup with shortcut commands for build, lint, test, and documentation generation. |
| 387 | Document SUMMARY.md structure for docs | OPEN | Explain the purpose of SUMMARY.md in mdBook or GitBook-based documentation systems. |
| 390 | Create docs/index.md as a simplified entry | OPEN | Provide a clean, GitHub-readable version of the documentation landing page for direct repo visitors. |
| 395 | Populate examples/ with real-world cases | OPEN | Add meaningful usage examples like CLI scripts, I/O files, or annotated command sequences. |
| 394 | Modularize CONTRIBUTING into subfolders | OPEN | Split contribution guidelines by type (e.g. tests, UI, plugins) under docs/contributing/. |
| 398 | Group developer setup files into a DX guide | OPEN | Bundle .editorconfig, .env.example, Makefile, and local scripts into a developer onboarding experience. |
| 399 | Summarize all docs structure references in one page | OPEN | Link to SUMMARY.md, index.md, and example files to provide a bird's-eye view of the documentation system. |
| 406 | Add refactor.md template | OPEN | Encourage structured code improvements (non-functional changes) through a dedicated suggestion format. |
| 293 | Document module READMEs under /src | OPEN | Each module should have a README explaining its purpose, usage, and API. |
| 292 | Create FAQ.md with common questions and solutions | OPEN | Provide answers to frequently asked questions to reduce support load. |

## 🔍 Milestone 3: Observability, Logging, and Event Tracing

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 517 | Design event trace model and ID propagation | OPEN | Define a trace metadata structure (e.g., trace_id, span_id, parent_id) to be attached to each event. |
| 518 | Implement trace-aware logger middleware | OPEN | Build a centralized logging middleware that attaches trace context to each log entry. |
| 519 | Add logging macros for trace-aware logging | OPEN | Create custom macros that automatically include trace metadata in structured logs. |
| 520 | Track trigger evaluation spans with structured logs | OPEN | Instrument trigger evaluation points with trace spans. |
| 521 | Log function execution duration and result | OPEN | Measure and log the runtime and output of each function execution. |
| 522 | Implement structured log serialization to JSON lines | OPEN | Configure logger to serialize structured log entries as newline-delimited JSON. |
| 523 | Add CLI command to tail recent logs with filter | OPEN | Create a CLI command that lets users stream recent logs from memory with optional filters. |
| 524 | Write unit tests for logger trace injection | OPEN | Test whether all logging macros correctly inject the expected trace context. |
| 525 | Document trace lifecycle and logging conventions | OPEN | Write documentation for how traces are created, propagated, and logged throughout the system. |
| 526 | Verify trace_id propagation across modules | OPEN | Perform integration testing to confirm that trace_id remains consistent across modules. |
| 527 | Add OpenTelemetry export support for traces | OPEN | Integrate OpenTelemetry to export tracing data from the system to compatible backends. |
| 528 | Expose real-time metrics via Prometheus endpoint | OPEN | Add an HTTP endpoint exposing Prometheus-compatible metrics for the entire system. |
| 529 | Instrument function execution with tracing spans | OPEN | Add span instrumentation to all function executions within HexaRun. |
| 530 | Create centralized in-memory log buffer with TTL | OPEN | Implement a shared in-memory buffer that stores recent structured logs with time-to-live eviction. |
| 531 | Support log export to external systems via bridge | OPEN | Add the ability to forward logs to external systems like Logstash or Loki through the Bridge module. |
| 532 | Add custom log level control via config file | OPEN | Allow configuring per-module log levels in the configuration file. |
| 533 | Implement alert logging for anomalies and errors | OPEN | Add logic to detect and tag anomalous behaviors or frequent failures in the logs as alerts. |
| 534 | Add CLI command to export logs as JSON archive | OPEN | Develop a CLI command that dumps all recent structured logs to a JSON file for diagnostics or backup. |
| 535 | Document OpenTelemetry integration steps | OPEN | Write a guide for setting up OpenTelemetry with HexaFn. |
| 536 | Test span nesting and correlation across async tasks | OPEN | Write integration tests to verify that span relationships and trace context are preserved across async boundaries. |
| 537 | Implement correlation ID support across external APIs | OPEN | Ensure external HTTP API requests automatically include correlation IDs for traceability. |
| 538 | Support custom tagging of spans and events | OPEN | Allow users to define custom tags for spans and event logs in config. |
| 539 | Integrate span sampling strategy configuration | OPEN | Expose options for configuring span sampling strategies via config. |
| 540 | Detect and alert on span timeouts or long-running tasks | OPEN | Monitor for spans that exceed a defined execution time threshold and log alerts when exceeded. |
| 541 | Enable export of metrics to external dashboards | OPEN | Allow exporting system metrics to external observability platforms. |
| 542 | Create admin CLI to dump full trace for an event ID | OPEN | Develop a CLI tool that accepts a trace_id or event_id and exports the full trace tree as a JSON file. |
| 543 | Write integration test for multi-module trace correlation | OPEN | Ensure a full pipeline (Trigger → Run → Cast → Bridge) produces traceable spans across modules. |
| 544 | Document best practices for observability in pipelines | OPEN | Provide developers with guidelines on how to make their custom functions, modules, or plugins observable. |
| 545 | Support live streaming logs to external viewer over WebSocket | OPEN | Add experimental support for streaming logs over WebSocket to a remote viewer or browser-based dashboard. |
| 546 | Refactor watch module structure for separation of concerns | OPEN | Split metrics, logging, and tracing logic into separate submodules within watch. |
| 547 | Audit trace propagation coverage across all modules | OPEN | Conduct a system-wide review to verify that trace IDs and span context are correctly propagated across modules. |
| 548 | Review structured logging format compliance | OPEN | Inspect all modules to ensure that log entries follow the expected structured JSON line format. |
| 549 | Document event tracing flow with diagrams | OPEN | Create a full visual diagram showing how a single event flows through the system with trace context. |
| 550 | Refactor trace span naming for clarity and searchability | OPEN | Standardize span names used across the system to make them more descriptive, unique, and consistent. |
| 551 | Finalize and publish observability changelog and checklist | OPEN | Compile a milestone changelog summarizing all observability, logging, and tracing features. |
| 349 | Configure CI to run tests on every commit | OPEN | Set up GitHub Actions for automated testing with pull request validation rules. |
| 350 | Enable linting and static analysis checks | OPEN | Enforce formatting and secure coding standards with Clippy, rustfmt, and optional CodeQL. |
| 351 | Protect main branch with review and CI requirements | OPEN | Require successful CI and reviewer approvals for all changes merged to main. |
| 352 | Automate labeling and stale issue handling | OPEN | Configure labeler and stale bots to auto-tag and close inactive PRs and issues. |
| 353 | Automate dependency updates with Dependabot | OPEN | Maintain security and compatibility by using Dependabot for automatic PRs on outdated packages. |
| 288 | Link CI status badge to main branch | OPEN | Show current CI status of the repository on the README. |
| 287 | Define label usage in LABELLING_STRATEGY.md | OPEN | Establish a consistent tagging system for issue triage and filtering. |
| 294 | Publish Rustdoc API reference on docs.rs | OPEN | Enable developers to understand the API through generated docs. |
| 307 | Add test coverage reporting via Codecov or Coveralls | OPEN | Track and display code coverage metrics to monitor testing completeness. |
| 306 | Integrate test commands (e.g. 'cargo test') into CI workflow | OPEN | Automatically validate code before merging to ensure quality standards. |
| 308 | Configure labeler.yml for path-based automatic labeling | OPEN | Improve PR triage and clarity by auto-assigning labels based on file changes. |
| 309 | Configure stale.yml to tag inactive issues and PRs | OPEN | Keep the repository clean by flagging abandoned threads automatically. |
| 310 | Enforce commit and PR title formatting using linting rules | OPEN | Ensure consistent commit messages to streamline automation and changelogs. |
| 381 | Implement opt-in telemetry with clear notice | OPEN | Respect user privacy while enabling insights collection to inform product development. |
| 375 | Use telemetry to detect popular and error-prone commands | OPEN | Collect usage and error data (opt-in) to prioritize improvements based on real user behavior. |
| 374 | Add optional telemetry flag for CLI usage | OPEN | Introduce a --telemetry flag in CLI to collect anonymous usage stats (opt-in only). |
| 389 | Add script for documentation generation (scripts/generate-docs.sh) | OPEN | Automate API docs generation using rustdoc or a custom tool to ensure consistency. |
| 388 | Add local CI workflow script (scripts/ci-check.sh) | OPEN | Allow contributors to run test, lint, and build steps locally before pushing. |
| 404 | Create performance.md template | OPEN | Use a dedicated template for performance reports (e.g. memory, latency) separate from bug reports. |
| 409 | Introduce ci-tests.yml for standalone test workflow | OPEN | Add a separate CI job to run 'cargo test' if the main workflow does not cover test execution. |
| 410 | Configure coverage.yml with grcov or tarpaulin | OPEN | Set up a test coverage job using a Rust-compatible tool. |
| 411 | Set up build.yml to verify binaries | OPEN | Ensure that the CLI builds successfully across OSes (Linux/macOS/Windows). |
| 412 | Add doc-check.yml to validate documentation | OPEN | Check that 'cargo doc' compiles without errors on every push to prevent broken docs. |
| 413 | Enable spellcheck.yml using codespell | OPEN | Catch spelling errors in documentation files automatically with codespell. |
| 414 | Implement markdown-link-check.yml | OPEN | Automatically verify all links in '.md' files and prevent broken references. |
| 417 | Create performance-benchmark.yml | OPEN | Run performance benchmarks ('cargo bench') on each PR to track runtime changes. |
| 420 | Write integration-test.yml workflow | OPEN | Run end-to-end integration tests across modules or directories to verify real-world compatibility. |
| 429 | Configure docs.yml or sync-docs.yml workflow | OPEN | Enable automated publishing or syncing of documentation using mdbook or GitHub Pages. |
| 432 | Enable semantic-release for versioning and changelog | OPEN | Automate release processes and ensure consistent versioning practices. |
| 433 | Enable semantic-release for version management | OPEN | Use semantic-release to automate changelog generation and version tagging. |
| 396 | Write tests/structure.md to guide new test contributors | OPEN | Document how tests are organized and where to add new ones. |
| 407 | Create test-case.md template for test failures | OPEN | Provide a structured template for reporting failed or missing test scenarios. |

## 🗄️ Milestone 4: KV Storage and Configuration Persistence

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 552 | Define KV storage interface with CRUD operations | OPEN | Design a trait-based abstraction (e.g., KvStore) that provides generic CRUD operations for key-value pairs. |
| 553 | Implement in-memory KV store backend | OPEN | Create a simple in-memory backend implementation of the KvStore trait to be used for local testing and default operation. |
| 554 | Add JSON serialization and deserialization to KV entries | OPEN | Support storing arbitrary structured data as values in the KV store using serde-based serialization. |
| 555 | Enable namespaced key scoping in KV API | OPEN | Allow users and modules to specify logical namespaces for keys in the store. |
| 556 | Create CLI tool for interacting with KV store | OPEN | Add CLI commands to read, write, list, and delete entries in the KV store. |
| 557 | Implement version tracking for KV entries | OPEN | Track version metadata for each entry in the KV store to detect and handle conflicts, rollback, and audit changes. |
| 558 | Add unit tests for KV trait and memory backend | OPEN | Write comprehensive unit tests for the KvStore trait and its in-memory backend implementation. |
| 559 | Write documentation for KV module usage | OPEN | Provide clear documentation for using the KV module including usage examples, supported operations, and integration points. |
| 560 | Benchmark in-memory store for read/write throughput | OPEN | Evaluate the performance of the in-memory KV backend under varying workloads. |
| 561 | Evaluate external KV backends for future support | OPEN | Explore possible external backends (e.g., Redis, RocksDB) that can be used in place of the in-memory store. |
| 562 | Implement file-based KV store backend using JSON | OPEN | Develop a file-backed KV store implementation that persists entries as JSON on disk. |
| 563 | Add config schema validation before persisting | OPEN | Before saving config entries to the KV store, validate them against defined schemas to avoid storing malformed data. |
| 564 | Support hot reload of persisted config at runtime | OPEN | Enable automatic reloading of configuration data from the KV store into active memory at runtime. |
| 565 | Implement backup and restore tool for KV storage | OPEN | Create utilities that can export the entire KV store to a backup archive and restore from one. |
| 566 | Add access control hooks for protected keys | OPEN | Introduce role-based access hooks or ACLs to restrict read/write/delete access for sensitive key paths. |
| 567 | Refactor store APIs to support TTL as first-class option | OPEN | Make TTL (time-to-live) an optional but explicit field in all store write APIs. |
| 568 | Implement integration test: reload config after update | OPEN | Write a full test case that simulates a config update in the KV store and verifies that the updated values are reloaded. |
| 569 | Benchmark disk-backed store vs memory store | OPEN | Run comparative performance benchmarks for the memory-backed vs file-backed KV store implementations. |
| 570 | Write developer guide for custom KV backends | OPEN | Document how developers can implement and register their own custom KV backends using the provided KvStore trait interface. |
| 571 | Support atomic multi-key write transactions | OPEN | Add API support for atomic write operations involving multiple keys. |
| 572 | Implement RocksDB backend for persistent KV storage | OPEN | Integrate RocksDB as an alternative backend to support high-performance persistent storage for KV entries. |
| 573 | Add migration tool between KV backends | OPEN | Develop a tool to migrate data from one backend (e.g., memory or file) to another (e.g., RocksDB) without data loss. |
| 574 | Implement change notification hook for KV updates | OPEN | Allow registering callback hooks or message emitters that notify subscribers when specific keys are updated. |
| 575 | Add history tracking for KV modifications | OPEN | Maintain an audit log of changes made to the KV store including timestamps, old/new values, and operation types. |
| 576 | Support encryption at rest for KV values | OPEN | Implement optional encryption support for KV value storage at rest. |
| 577 | Add stress test for RocksDB write amplification | OPEN | Simulate high-frequency writes to the RocksDB backend and monitor for performance degradation due to write amplification. |
| 578 | Document data integrity and corruption recovery mechanisms | OPEN | Explain how each backend handles crashes, integrity checking, and recovery procedures. |
| 579 | Evaluate alternative embedded stores (e.g., sled, LMDB) | OPEN | Research and compare other embeddable KV databases for size, speed, features, and ecosystem fit. |
| 580 | Refactor KV API for consistent async support | OPEN | Ensure all KV backend operations are exposed as async methods where applicable. |
| 581 | Create recovery test: restart after partial write crash | OPEN | Simulate a process crash during write operation and ensure that the store recovers correctly without data corruption. |
| 582 | Audit consistency of KV key naming across modules | OPEN | Review how keys are structured and named throughout the system. |
| 583 | Validate multi-backend support with integration tests | OPEN | Create integration test suite to verify that both memory, file, and RocksDB backends comply with the KvStore trait interface. |
| 584 | Document KV backend selection and fallback strategy | OPEN | Write developer-facing documentation describing how to configure and switch between KV backends. |
| 585 | Refactor CLI and config tools for backend abstraction | OPEN | Ensure CLI tools and configuration management utilities interact with the KV layer abstractly, without assuming a specific backend. |
| 586 | Publish changelog and checklist for KV and config milestone | OPEN | Compile all completed tasks and features from Milestone 4 into a changelog. |
| 346 | Set up public communication channels | OPEN | Enable GitHub Discussions or create a Discord/Slack community to encourage transparent collaboration. |
| 347 | Implement contributor recognition mechanism | OPEN | Use tools like all-contributors to highlight community contributions in README and website. |
| 298 | Enable GitHub Discussions | OPEN | Open up a community space for support, questions, and proposals. |
| 300 | Update SUPPORT.md with help contact points | OPEN | Clarify how to get help and support for different issue types. |
| 301 | Add all contributor links to README and CONTRIBUTING.md | OPEN | Improve visibility of community channels and recognition. |
| 299 | List response time expectations in SUPPORT.md | OPEN | Set communication expectations for issue/discussion replies. |
| 303 | Clarify contact channels in SUPPORT.md | OPEN | Clearly indicate where users should go for different kinds of questions to reduce confusion. |
| 304 | Add community links to README and CONTRIBUTING.md | OPEN | Ensure contributors can easily access social and collaboration channels. |
| 305 | Specify response time expectations for issues and discussions | OPEN | Help contributors understand expected feedback timelines to build trust. |
| 324 | Set up all-contributors bot integration | OPEN | Automatically recognize contributors in README based on contribution types to boost engagement. |
| 325 | Add Contributors section in README | OPEN | Display a dedicated section listing contributors to increase visibility and appreciation. |
| 326 | Highlight contributors in PRs and Releases | OPEN | Mention key contributors in PR descriptions and release notes to reinforce community value. |
| 327 | List contributors in new release announcements | OPEN | Acknowledge contributions publicly to motivate ongoing involvement. |
| 328 | Issue digital badges for contributors | OPEN | Use shields.io to award badges based on contribution levels, enhancing motivation and recognition. |
| 368 | Create onboarding guide and badge for new contributors | OPEN | Build docs/onboarding.md with a guided walkthrough and reward first-time contributors. |
| 369 | Record a quickstart video for new contributors | OPEN | Create a 2-minute walkthrough video titled 'Make Your First Contribution'. |
| 373 | Add a personal call-to-action for contributors | OPEN | Include a heartfelt section in the README explaining why people should contribute to the project. |
| 376 | Generate contribution certificates | OPEN | Provide contributors with auto-generated PDF certificates using GitHub Actions to support their resumes. |
| 377 | Add 'Get Your Certificate' CTA in README | OPEN | Encourage contributions by linking to automated certificate generation flow from the README. |
| 378 | Write a guided onboarding doc for contributors | OPEN | Create an easy-to-follow onboarding document to help new contributors get started confidently. |
| 382 | Issue contribution certificate on successful PR merge | OPEN | Reward contributors with a professional-looking PDF badge to recognize their efforts. |
| 391 | Track contributors in CONTRIBUTORS.json | OPEN | Maintain a structured list of contributors for use with bots like all-contributors or custom tools. |
| 403 | Add translation.md template for localization | OPEN | Provide a clear translation contribution guide for starting and submitting new language versions. |
| 416 | Configure triage.yml for auto-replies | OPEN | Respond to specific tags or new issues with automatic guidance or contributor onboarding suggestions. |
| 418 | Set up generate-contributors.yml | OPEN | Automatically update 'all-contributors.json' and README contributor section after PR merges. |
| 421 | Enable notify-discord.yml or notify-slack.yml | OPEN | Send notifications about new releases, failed CI runs, or important PRs directly to Discord or Slack. |
| 423 | Maintain CONTRIBUTORS.md if all-contributors not used | OPEN | List all contributors manually to acknowledge their involvement, especially if not using a bot. |
| 428 | Create translations/README.md for i18n contributors | OPEN | Explain how contributors can submit translations for multilingual support. |
| 430 | Add acknowledgements.md to thank contributors | OPEN | Recognize individual and organizational supporters to foster transparency and trust. |
| 436 | Respond quickly to issues and PRs | OPEN | Triage issues and pull requests regularly to ensure contributors get feedback within 24–48 hours. |
| 437 | Promote first contributions with onboarding issues | OPEN | Tag beginner-friendly tasks as 'good first issue' and provide clear instructions to lower entry barriers. |
| 438 | Award contributor badges and recognitions | OPEN | Introduce digital badges or certifications for contributors to boost motivation and highlight effort. |
| 439 | Enable real-time chat for community support | OPEN | Create a Discord or Matrix room for quick user and contributor interaction. |
| 442 | Add gamified leaderboard or points system | OPEN | Introduce a LEADERBOARD.md and points system to encourage and recognize contributions. |
| 289 | Add all-contributors configuration | OPEN | Setup automatic recognition for contributions in README with all-contributors. |
| 400 | Create contribution flow overview | OPEN | Link files like CONTRIBUTORS.json, structure.md, and release steps into one cohesive contribution section. |

## ⚙️ Milestone 5: Runtime Support for WASM/JS/Lua Execution

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 587 | Define generic FunctionRuntime trait with execution contract | OPEN | Design a trait (e.g., 'FunctionRuntime') that provides a common interface for executing user-defined functions. |
| 588 | Implement DSL runtime as default FunctionRuntime | OPEN | Use the previously implemented DSL interpreter as the default implementation of the FunctionRuntime trait. |
| 589 | Add WASM runtime support using wasmtime | OPEN | Integrate the Wasmtime engine to support WebAssembly execution. |
| 590 | Add JavaScript runtime support using QuickJS | OPEN | Embed the QuickJS engine to support lightweight JavaScript execution. |
| 591 | Add Lua runtime support using rlua or mlua | OPEN | Use an embeddable Lua engine (e.g., rlua or mlua) to run Lua functions. |
| 592 | Design runtime context object for passing inputs | OPEN | Create a 'RuntimeContext' structure that holds input parameters, environment variables, and metadata. |
| 593 | Implement runtime registration and factory system | OPEN | Create a registry or factory to dynamically select the correct FunctionRuntime based on config or declared type. |
| 594 | Write unit tests for each runtime executor interface | OPEN | Ensure WASM, JS, and Lua runtime wrappers conform to FunctionRuntime contract. |
| 595 | Benchmark runtime execution latency across modes | OPEN | Run benchmarks comparing average and worst-case execution times for DSL, WASM, JS, and Lua runtimes. |
| 596 | Document multi-runtime architecture and usage examples | OPEN | Write comprehensive documentation explaining the runtime abstraction layer. |
| 597 | Support runtime input validation schema per language | OPEN | Allow each function to declare an input validation schema to verify inputs before execution. |
| 598 | Allow per-runtime configuration in function definitions | OPEN | Extend function config structure to allow specifying runtime-specific parameters. |
| 599 | Track runtime execution metrics and errors | OPEN | Collect and expose metrics per runtime including execution duration, success rate, memory usage, and failure causes. |
| 600 | Implement resource limiter for WASM runtime | OPEN | Add CPU and memory usage limits for the WASM runtime using Wasmtime's configuration. |
| 601 | Enable runtime selection from incoming event metadata | OPEN | Allow dynamic selection of execution runtime based on fields in the event metadata. |
| 602 | Support runtime fallback chain in function config | OPEN | Permit function config to specify a fallback execution chain (e.g., if WASM fails, use JS). |
| 603 | Test concurrent executions across different runtimes | OPEN | Verify that concurrent function executions across DSL, WASM, JS, and Lua do not interfere with each other. |
| 604 | Add CLI command to test function in specific runtime | OPEN | Build CLI utility to test a single function in a specified runtime with provided input. |
| 605 | Write tutorial: how to write and deploy WASM functions | OPEN | Create a step-by-step guide for writing, compiling, and deploying a function in WebAssembly. |
| 606 | Benchmark runtime performance under mixed workload | OPEN | Simulate a workload with mixed function types and gather metrics for performance analysis. |
| 607 | Add runtime-specific logging with trace context | OPEN | Enhance each runtime to include structured logs enriched with trace_id and runtime type. |
| 608 | Validate output schema for each function runtime | OPEN | Enable output validation against declared schemas after function execution. |
| 609 | Add memory usage and CPU time tracking per function | OPEN | Track the memory and CPU time consumed during each function execution across all runtimes. |
| 610 | Allow environment variable injection into runtimes | OPEN | Permit secure and scoped injection of environment variables into runtimes at execution time. |
| 611 | Add runtime output to feedback channel | OPEN | Publish function execution results, metadata, and logs to a feedback system for audit and inspection. |
| 612 | Simulate error injection in each runtime | OPEN | Test how each runtime reacts to injected failures like syntax errors, timeouts, or invalid inputs. |
| 613 | Support execution timeouts in all runtimes | OPEN | Apply max execution timeouts across all runtimes. |
| 614 | Refactor runtime error messages for consistency | OPEN | Standardize error messages returned by all FunctionRuntime implementations. |
| 615 | Document runtime security sandbox policies | OPEN | Describe security boundaries for each runtime. |
| 616 | Add multi-language examples in documentation | OPEN | Provide working function examples written in DSL, WASM, JS, and Lua with configuration and expected output for each. |
| 617 | Review multi-runtime integration consistency | OPEN | Validate that all runtime implementations adhere to the FunctionRuntime trait contract. |
| 618 | Audit security isolation guarantees across runtimes | OPEN | Review the security configurations and sandboxing effectiveness of each runtime. |
| 619 | Document runtime fallback, timeout, and retry behaviors | OPEN | Write a reference guide explaining how fallback chains, execution timeouts, and retry mechanisms work. |
| 620 | Refactor runtime module layout for maintainability | OPEN | Reorganize the runtime module into subdirectories per runtime. |
| 621 | Publish milestone changelog and finalize runtime checklist | OPEN | Prepare a milestone changelog summarizing runtime features added, bugs fixed, and architecture decisions. |
| 446 | Add docker-build.yml for container publishing | OPEN | Build and push Docker images to Docker Hub or GHCR after tagged releases. |
| 348 | Define ownership and assign maintainers | OPEN | Introduce CODEOWNERS and delegate review responsibilities to streamline scaling of code review and governance. |
| 297 | Create CODEOWNERS file for review assignment | OPEN | Assign reviewers automatically based on file paths. |
| 295 | Explain CLA/DCO process in CONTRIBUTING.md | OPEN | Clarify how contributors can comply with licensing requirements. |
| 291 | Embed license and version badge | OPEN | Add SPDX license identifier and latest release badge on top of README. |
| 284 | Draft SECURITY.md policy | OPEN | Outline responsible disclosure process and basic vulnerability reporting steps. |
| 312 | Add SPDX-License-Identifier headers to all source files | OPEN | Enable machine-readable license metadata and REUSE compatibility. |
| 313 | Include full license texts for all dependencies under LICENSES/ | OPEN | Ensure transparency and legal clarity by bundling all license files. |
| 314 | Create DEPENDENCIES_LICENSES.md with license list of dependencies | OPEN | Provide a full overview of third-party license compliance. |
| 315 | Ensure LICENSE file is complete MIT and linked in README | OPEN | Improve user confidence and contributor clarity with visible licensing. |
| 316 | Run reuse lint in CI using GitHub Actions | OPEN | Automatically validate license headers and compliance on each commit. |
| 317 | Set up CLA or DCO process for contributor license agreement | OPEN | Protect legal validity of contributions with automated approval flow. |
| 359 | Add LICENSE file with a clear open source license | OPEN | Ensure the repository includes a LICENSE file with a well-defined open source license such as MIT or Apache 2.0. |
| 360 | Ensure REUSE compliance using SPDX identifiers | OPEN | Apply SPDX-License-Identifier headers to all source files and include full license texts to support REUSE best practices. |
| 361 | Track and document third-party licenses | OPEN | List licenses for all dependencies in LICENSES/ directory and ensure full legal compliance for redistribution. |
| 362 | Create GitHub Project board for task tracking | OPEN | Visualize progress using a Kanban board with Todo/In Progress/Done columns. |
| 363 | Release regularly and maintain changelog | OPEN | Use GitHub Releases and CHANGELOG.md to track and publish version updates and enhancements. |
| 392 | Add LICENSE.txt for visual/media assets | OPEN | Specify licensing terms separately for non-code assets like logos, illustrations, and screenshots. |
| 393 | Document code quality metrics in CODE_METRICS.md | OPEN | Track lines of code, coverage, module count, and other metrics to help measure project health and progress. |
| 401 | Create sustainability and maintainability index | OPEN | Bundle CODE_METRICS.md, CHARTER.md, MAINTAINERS.md to establish a sustainability scorecard. |
| 402 | Clarify image/media license terms | OPEN | Add LICENSE.txt specifically for design and branding assets under assets/ to comply with distribution norms. |
| 405 | Create security.md template | OPEN | Provide instructions for responsible disclosure, possibly including private contact or form. |
| 408 | Add meta.md for project-wide suggestions | OPEN | Use this template for infrastructure, CI/CD, documentation process, or workflow-related proposals that affect the whole project. |
| 415 | Add sync-labels.yml for automated label sync | OPEN | Keep labels in sync with labeler.yml configuration via GitHub Action using github-script. |
| 419 | Create release-assets.yml to upload binaries | OPEN | Package and publish platform-specific pre-built binaries during each release using GitHub Actions. |
| 422 | Create GOVERNANCE.md to define decision structure | OPEN | Explain how the project is maintained, how decisions are made, and how contributors can become maintainers. |
| 424 | Add multilingual versions of Code of Conduct | OPEN | Provide CODE_OF_CONDUCT translations for better inclusivity and accessibility among non-English contributors. |
| 425 | Rename SECURITY.md to SECURITY_POLICY.md | OPEN | Follow GitHub convention and rename the file to ensure automatic linking and recognition on the platform. |
| 426 | Write RELEASE_GUIDE.md for versioning process | OPEN | Document release steps including tagging, changelog updates, and version bumping rules (semantic versioning). |
| 427 | Create MAINTAINERS.md for project leadership | OPEN | List contributors with merge/review rights and outline their responsibilities. |
| 431 | Duplicate LICENSE.md in .github/ directory | OPEN | Include a copy of LICENSE in .github/ to ensure GitHub detects it correctly and renders license info. |
| 434 | Publish project roadmap and version goals | OPEN | Create or update ROADMAP.md with upcoming plans and link it to GitHub milestones for visibility. |
| 435 | Create milestone per major version | OPEN | Use GitHub milestones to organize issues and PRs for each release cycle. |
| 329 | Visual task board with Todo/In Progress/Done columns | OPEN | Set up a GitHub Project board to track development progress transparently. |
| 330 | Link roadmap to milestones and board | OPEN | Ensure ROADMAP.md includes references to project board and milestones for unified planning. |
| 331 | Add time-based goals to ROADMAP.md | OPEN | Break roadmap into time frames (e.g. 3/6/12 months) to communicate project direction clearly. |
| 332 | Specify versioning scheme in CONTRIBUTING.md | OPEN | Clarify usage of semantic versioning to guide maintainers and contributors. |

## 📦 Milestone 6: Output Forwarding and External Delivery

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| 622 | Design output forwarding abstraction | OPEN | Define a pluggable interface (e.g., 'OutputForwarder') for delivering function results to external systems. |
| 623 | Implement HTTP forwarder for function outputs | OPEN | Develop a forwarder that delivers function results to a configured HTTP endpoint. |
| 624 | Support topic-based forwarding via Cast | OPEN | Allow function outputs to be forwarded to specific Cast topics for further consumption within the system. |
| 625 | Add delivery metadata to function output structure | OPEN | Extend function output format to include delivery instructions such as destination type, endpoint, and retry strategy. |
| 626 | Implement local file sink for debug delivery | OPEN | Create a simple output sink that writes results to a local file in JSON format. |
| 627 | Create config format for defining output targets | OPEN | Define a configuration schema allowing developers to declare output forwarding rules, destinations, and parameters. |
| 628 | Emit delivery event logs with status and latency | OPEN | Log all delivery attempts with metadata such as success/failure, endpoint, latency, and payload size. |
| 629 | Write test for HTTP delivery with mock server | OPEN | Test the HTTP forwarder by delivering function outputs to a local mock server. |
| 630 | Write documentation for configuring output delivery | OPEN | Provide user-facing documentation on how to enable, configure, and test output delivery. |
| 631 | Benchmark performance of HTTP and Cast forwarders | OPEN | Measure and compare the performance characteristics of HTTP vs Cast-based output delivery. |
| 632 | Support batching of output deliveries | OPEN | Implement batching logic to group multiple outputs into a single delivery when supported. |
| 633 | Add retry with exponential backoff on delivery failure | OPEN | Introduce a retry mechanism with exponential backoff strategy for failed delivery attempts. |
| 634 | Support conditional delivery based on function metadata | OPEN | Enable conditional logic to determine whether a function's output should be forwarded or dropped. |
| 635 | Implement Kafka forwarder for message streaming | OPEN | Add support for forwarding outputs to Kafka topics. |
| 636 | Add output redaction support before delivery | OPEN | Enable output filtering or redaction rules to remove sensitive fields from output payloads. |
| 637 | Add delivery timeout configuration per destination | OPEN | Support destination-specific timeout settings in config to control max time allowed for delivery to complete. |
| 638 | Write tests for conditional delivery logic | OPEN | Write test cases to confirm that conditional delivery rules are honored under different execution states. |
| 639 | Test retry and backoff logic with simulated failures | OPEN | Create a test environment to simulate delivery failures and confirm that retry/backoff logic works as expected. |
| 640 | Document delivery flow with supported backends | OPEN | Provide documentation and flow diagrams showing how delivery works across HTTP, Cast, Kafka, and file sinks. |
| 641 | Benchmark throughput of Kafka forwarder under load | OPEN | Measure the performance of Kafka output delivery under high volume conditions. |
| 642 | Implement WebSocket forwarder for live output streaming | OPEN | Add support for real-time output delivery over WebSocket. |
| 643 | Support delivery to multiple targets in parallel | OPEN | Enable configuration of multiple simultaneous delivery targets for a single function output. |
| 644 | Add circuit breaker to disable failing destinations | OPEN | Implement a circuit breaker pattern for external destinations. |
| 645 | Emit structured delivery error reports | OPEN | Log detailed structured error reports when delivery fails. |
| 646 | Enable delivery result hook for audit plugins | OPEN | Allow audit or monitoring plugins to hook into delivery result events for additional processing or storage. |
| 647 | Create CLI to list active delivery targets and status | OPEN | Add CLI support to inspect currently configured delivery targets, recent success/failure status, and connection info. |
| 648 | Test multi-destination parallel delivery scenarios | OPEN | Write test cases that verify proper delivery and failure isolation when output is routed to multiple targets simultaneously. |
| 649 | Test WebSocket forwarder with real-time stream client | OPEN | Develop a lightweight WebSocket client to subscribe to output streams. |
| 650 | Document advanced delivery strategies and fallback flows | OPEN | Create documentation on designing robust delivery strategies, including retries, circuit breakers, redirection, and fallback routing. |
| 651 | Benchmark resource usage of multi-target delivery engine | OPEN | Measure CPU, memory, and latency implications of delivering outputs to multiple concurrent targets under stress. |
| 652 | Review output delivery interface for completeness | OPEN | Audit the OutputForwarder trait and delivery pipeline to ensure all planned backends are implemented, tested, and consistent. |
| 653 | Test error recovery paths across delivery methods | OPEN | Simulate delivery failures across HTTP, Kafka, WebSocket, and file sinks. |
| 654 | Document delivery configuration best practices | OPEN | Create a practical configuration guide for developers defining output delivery pipelines. |
| 655 | Refactor bridge module structure for clarity | OPEN | Organize the bridge module into clear submodules by transport type. |
| 656 | Finalize and publish delivery milestone changelog | OPEN | Compile and publish the final changelog summarizing all delivery-related features, tests, and documentation. |
| 354 | Display status badges in README | OPEN | Include CI status, code coverage, license, and version badges at the top of the README. |
| 355 | Launch a simple project website with custom domain | OPEN | Host documentation and blog on a GitHub Pages site with a dedicated domain name. |
| 356 | Build awareness via social media strategy | OPEN | Create project accounts and share updates across platforms like X/Twitter and LinkedIn. |
| 357 | Produce short video demos and visual walkthroughs | OPEN | Create YouTube videos or GIFs showing key functionality for better engagement. |
| 358 | Produce short video demos and visual walkthroughs | OPEN | Create YouTube videos or GIFs showing key functionality for better engagement. |
| 364 | Add donation and sponsorship options | OPEN | Enable GitHub Sponsors or link to OpenCollective/Patreon for financial support of the project. |
| 365 | Launch newsletter or project blog | OPEN | Regularly publish updates, release news, and contributor spotlights to engage the community. |
| 366 | Publish user feedback insights in a document | OPEN | Create docs/USER_FEEDBACK.md to share summarized responses and learnings. |
| 367 | Record terminal demo with Asciinema | OPEN | Showcase CLI usage via a lightweight terminal recording for quick preview. |
| 370 | Add SEO metadata and social cards | OPEN | Improve discoverability by adding meta tags, title/description, and social preview images to the documentation site. |
| 371 | Implement OpenGraph and Twitter card support | OPEN | Ensure proper previews when sharing the project on social platforms by defining OpenGraph metadata. |
| 372 | Define project mission and long-term goals | OPEN | Create a VISION.md or MISSION.md file to clarify the 'why' behind the project and its intended impact. |
| 380 | Create a vision section on the landing page | OPEN | Summarize the project's purpose to build emotional connection and shared goals among contributors. |
| 379 | Add meta tags and sitemap for better SEO | OPEN | Help search engines index the project effectively and improve organic reach. |
| 440 | Add user feedback form to gather insights | OPEN | Include a Typeform or Google Form in README to collect user feedback and suggestions. |
| 441 | Invite contributors via survey opt-in | OPEN | Add a post-survey question asking users if they want to contribute and guide them accordingly. |
| 443 | Collect structured user feedback | OPEN | Design a survey to gather insights about user needs, pain points, and feature expectations. |
| 444 | Provide interactive demo environment | OPEN | Set up a web-based or terminal demo that lets users try core features without installation. |
| 445 | Build browser-accessible or Gitpod demo | OPEN | Offer a 'Run it online' experience with Docker or Gitpod integration for frictionless testing. |
| 321 | Redesign static site with accessible and branded layout | OPEN | Ensure a professional first impression with a clean, color-consistent, and accessible design. |
| 322 | Define branding rules in BRANDING.md | OPEN | Document logo usage, color codes, fonts, and iconography to ensure consistent brand representation. |
| 323 | Create responsive logo and header images | OPEN | Provide high-quality visuals for both mobile and desktop displays to enhance UX. |
| 333 | Create social media accounts for the project | OPEN | Improve visibility by sharing updates on platforms like Twitter/X, LinkedIn, or Dev.to. |
| 334 | Start a newsletter or blog for releases | OPEN | Share updates regularly through articles or emails to retain and inform users. |
| 335 | Produce short YouTube demos or animations | OPEN | Create visual content to showcase features and simplify onboarding. |
| 336 | Embed promotional links in README | OPEN | Add videos, blog links, and social handles to README to improve engagement and discoverability. |
| 337 | Add donation links via GitHub Sponsors or OpenCollective | OPEN | Provide options for financial support to sustain project development. |
| 338 | Participate in community events like Hacktoberfest | OPEN | Attract new contributors by engaging with developer events and campaigns. |
| 397 | Define vision and values in CHARTER.md | OPEN | Outline long-term goals, values, and principles guiding project development in a philosophical format. |

