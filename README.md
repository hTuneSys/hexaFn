<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->
 
# hexaFn 

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

<p align="center">
  <img src="docs/assets/hexaFn-logo.png" alt="hexaFn Logo" />
</p>

**From Feed to Feedback, fully programmable.**  
A modern event-driven function pipeline powered by the 6F Lifecycle Flow.

---

## ⚙️ What is hexaFn?

hexaFn is a programmable, event-driven data engine designed around a modular architecture known as the **6F Lifecycle Flow**:

> **Feed → Filter → Format → Function → Forward → Feedback**

Built by [hexaTune Team](https://hexafn.com) and maintained under the [hTuneSys](https://github.com/hTuneSys) GitHub organization, hexaFn lets developers build reactive systems where data is not only stored or published—but actively **processed, transformed, and routed through logic**.

---

## 🔷 6F Lifecycle Flow

Each stage of the `6F` architecture represents a distinct processing step:

- **Feed**: Ingest events, data, or commands from external sources.
- **Filter**: Pre-process and evaluate conditions.
- **Format**: Normalize, reshape or validate incoming data.
- **Function**: Apply programmable logic through dynamic, pluggable functions.
- **Forward**: Dispatch processed output to destinations (KV, pubsub, webhooks, etc.).
- **Feedback**: Log, trigger, or record responses for traceability and orchestration.

<p align="center">
  <img src="docs/assets/diagram.png" alt="6F Flow Diagram" width="720"/>
</p>

---

## 🧩 Core Modules

| Module Name     | Description |
|------------------|-------------|
| `HexaStore`      | Typed, event-driven key-value store |
| `HexaCast`       | Real-time pub-sub & broadcast engine |
| `HexaRun`        | Dynamic function runtime (WASM, JS, DSL) |
| `HexaTrigger`    | Conditional triggers & flow orchestration |
| `HexaWatch`      | Observation, logging & audit system |
| `HexaBridge`     | External integrations (webhook, SDK, APIs) |

---

## 👤 Who is it for?

| Target User         | Description |
|----------------------|-------------|
| **Event Architects** | Backend developers building reactive systems |
| **Realtime Hackers** | Developers working on chat, IoT, gaming or low-latency platforms |
| **Product Integrators** | Webhook and automation developers |
| **AI Pipeliners** | ML engineers building live inference pipelines |
| **Infra Builders** | Architects building lightweight infra solutions |
| **Plugin Crafters** | Developers writing reusable logic and extensions |

---

## ❌ What hexaFn is NOT (Never-6)

| Code Name        | We are NOT... |
|------------------|---------------|
| `Never-SQL`      | A relational SQL database |
| `No-Bloat`       | A heavy monolith or bloated framework |
| `No-Wait`        | A batch-oriented delay system |
| `No-LockIn`      | A proprietary SaaS with vendor lock-in |
| `No-UIBuilder`   | A frontend builder or CMS |
| `No-Magic`       | An unpredictable, side-effect-prone black box |

---

## 📦 Getting Started

_Installation and usage instructions coming soon..._

> We're currently preparing core module APIs, contributor guides, and CLI tooling.  
> To follow progress, [watch this repo](https://github.com/hTuneSys/hexaFn) or join our discussions.

---

## 🤝 Contributing

Want to help shape the future of programmable data engines?  
Check out [CONTRIBUTING.md](./CONTRIBUTING.md) to get involved.  

---

## 👨‍💻 Author

**hexaFn** is created and maintained by [hexaTune LLC](https://github.com/hTuneSys).

---

## License

This project is licensed under the [MIT License](./LICENSE).  
© 2025 hexaTune LLC. All rights reserved.

The license file is provided in full at the root of this repository.  
All source files include [SPDX](https://spdx.dev/) headers and the project is compliant with [REUSE Specification 3.3](https://reuse.software/spec/).