<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# hexaFn – Use Cases

hexaFn is a programmable event-function engine that combines KV storage, pub-sub messaging, and dynamic function execution. Below are common scenarios where hexaFn can be applied.

---

## 🔁 1. Realtime Event Pipelines

Use hexaFn to receive, filter, transform, and dispatch realtime data from one system to another.

**Example:**
- Feed: IoT sensor stream
- Filter: only temperature > 50°C
- Format: JSON compact
- Function: trigger cooling system
- Forward: notify via webhook

---

## 🤖 2. AI Function Composition

Create modular function pipelines for AI workflows using HexaRun.

**Example:**
- Feed: user prompt input
- Filter: check for empty input
- Format: convert to structured prompt
- Function: run text generation
- Feedback: log latency + token usage

---

## 🧩 3. Plugin Execution for Apps

Power plugin systems for your own SaaS or CLI tool using hexaFn’s event-driven runtime.

**Example:**
- Feed: command event from user
- Format: standardize plugin payload
- Function: execute plugin logic (WASM or native)
- Forward: return output to UI

---

## 📦 4. Serverless Queue Workers

Build reliable event workers without maintaining infrastructure.

**Example:**
- Feed: new user registration
- Function: assign default resources
- Forward: send welcome email
- Feedback: log completion + error

---

## 📡 5. Integration with External APIs

Connect and transform data between external services through modular triggers.

**Example:**
- Feed: webhook from Stripe
- Filter: only invoice paid
- Function: update billing record
- Forward: sync with accounting platform

---

## 🌐 6. Multi-Stage Data Processing

Chain multiple processing stages using declarative lifecycle logic.

**Example:**
- Stage 1: ingest CSV → validate → convert to JSON
- Stage 2: enrich with 3rd-party API
- Stage 3: store in KV → notify admin

---

These use cases demonstrate the flexibility of hexaFn to serve as the control plane for modern, event-centric architectures.
