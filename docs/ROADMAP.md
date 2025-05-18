<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# hexaFn Roadmap

This roadmap outlines the planned phases for building hexaFn — a programmable event-function engine with KV, pub-sub, and dynamic execution modules.

---

## ✅ v0.1 – Core MVP (Foundations)
🔹 Status: In Progress  
🔹 Planned Modules:
- HexaStore (in-memory key-value store)
- HexaRun (basic WASM-based function runner)
- CLI prototype (manual function execution)
- Initial docs + branding + CI/CD

---

## 🔄 v0.2 – Reactive Engine (Events)
🔹 Status: Planned  
🔹 Additions:
- HexaTrigger (event binding)
- Basic lifecycle routing: Feed → Filter → Function
- Event simulation CLI
- Minimal web API layer
- First external trigger support (manual webhook)

---

## 🚀 v0.3 – Streaming & Pub/Sub
🔹 Status: Planned  
🔹 Additions:
- HexaCast (Pub/Sub engine)
- Topic system with fanout
- Retain/replay support
- CLI commands for publish/subscribe

---

## 🧠 v0.4 – Feedback & Observability
🔹 Status: Planned  
🔹 Additions:
- HexaWatch (feedback/metrics loop)
- Logging, tracing, internal metrics
- Event visualization CLI/web demo
- First production-ready integration

---

## 🎯 v1.0 – Full Lifecycle Stable Release
🔹 Status: Pending  
🔹 All 6 modules production-ready:
  - HexaStore, HexaRun, HexaCast, HexaTrigger, HexaWatch, HexaBridge
- Declarative pipelines (YAML)
- Plugin system (modular runtime)
- Developer SDKs
- Documentation site launch
