<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📅 MILESTONES.md

This document defines the structured milestone plan for the hexaFn project. Each milestone spans three development sprints, focusing on a specific theme or functional domain. The plan aims to support clarity, maintainability, and sustainable progress over time.

---

## 🧠 Milestone 1: Establish Trigger → Run flow using DSL

Introduce the core trigger and execution pipeline.  
🎯 The goal is to allow events to trigger function execution using a domain-specific language (DSL).  
🏗️ This milestone sets up the project structure and implements a basic trigger system and DSL interpreter,  
🔁 enabling an end-to-end flow where a defined trigger condition executes a DSL-based function.

---

## 🌐 Milestone 2: Webhook integration and event broadcasting

Enable external events to enter the system and be disseminated internally.  
🔌 This milestone focuses on creating a webhook endpoint to receive events  
📤 and a publish/subscribe mechanism to broadcast those events.  
🛡️ It establishes secure ingestion of HTTP webhook calls,  
🧽 normalizes incoming data into a standard event format,  
📡 and distributes events to various components via an in-memory bus (Cast).

---

## 🔍 Milestone 3: Observability, logging, and event tracing

Build a robust observability layer across the pipeline.  
🧵 The aim is to introduce structured logging and tracing so that every event, trigger evaluation, and function execution can be monitored.  
🪪 This milestone implements a trace ID propagation model,  
🧾 enriched logging with context,  
📈 and metrics endpoints,  
🔎 allowing developers to trace events end-to-end and gather performance insights in real time.

---

## 🗃️ Milestone 4: KV storage and configuration persistence

Provide persistent configuration storage through a key-value store interface.  
🧱 This milestone introduces a KV storage module to save and retrieve configuration (like trigger definitions or function settings) reliably.  
🧠 It starts with an in-memory store and evolves to a file-based (and potential external) storage,  
🗂️ supporting features like namespacing, versioning, data validation,  
♻️ and hot-reloading of configuration at runtime.

---

## ⚙️ Milestone 5: Runtime support for WASM/JS/Lua execution

Extend function execution beyond the DSL by supporting multiple runtimes.  
🧩 The goal of this milestone is to implement a unified FunctionRuntime interface  
🌍 and integrate WebAssembly (WASM), JavaScript, and Lua execution engines alongside the existing DSL.  
🔐 It ensures that functions can be written in different languages, with proper sandboxing and resource limits,  
📊 while maintaining a consistent API, metrics tracking, and error handling across all runtime types.

---

## 📤 Milestone 6: Output forwarding and external delivery

Allow function results to be delivered to external systems or outputs.  
🔁 This milestone defines an output forwarding framework  
🌐 and implements various output channels (such as HTTP callbacks, Kafka topics, WebSocket streams, or file sinks).  
🛠️ The focus is on configuring where and how function outcomes are sent,  
📦 including support for batching, retries, and result filtering,  
🧾 along with thorough logging of delivery attempts for audit and reliability.

---

## 📊 Live Project Tracking

- 🗂️ GitHub Project Board: [BOARD](https://github.com/orgs/hTuneSys/projects/15/views/1)  
- 📅 Milestone Progress: [MILESTONES](https://github.com/hTuneSys/hexaFn/milestones)  
- 🐛 GitHub Issues: [ISSUES](https://github.com/hTuneSys/hexaFn/issues)
