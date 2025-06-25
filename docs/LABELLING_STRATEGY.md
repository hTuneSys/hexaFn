<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# LABELLING_STRATEGY.md

This document defines the complete, system-approved label taxonomy used in the hexaFn GitHub repository. These labels help organize issues and pull requests for better triage, automation, and collaboration.

---

## 🧭 Meta Labels

- `default:unclassified` – No specific category matched this PR or file path

---

## 🧑‍💻 Contribution Labels

- `good first issue` – Good entry point for contributors
- `help wanted` – Contributions welcome

---

## 🧩 Module Labels

- `module:core` – Core architecture & event flow
- `module:bridge` – HexaBridge (external integrations)
- `module:trigger` – HexaTrigger (rule engine)
- `module:run` – HexaRun (function runtime)
- `module:store` – HexaStore (key-value engine)
- `module:cast` – HexaCast (pub-sub engine)
- `module:watch` – HexaWatch (logging & tracing)
- `module:docs` – Documentation tasks
- `module:cli` – CLI tools

---

## 📊 Priority Labels

- `priority:high` – Urgent and critical tasks
- `priority:medium` – Normal importance
- `priority:low` – Can wait

---

## 🔁 Status Labels

- `status:blocked` – Blocked by another issue or PR
- `status:needs-review` – Awaiting code review
- `status:needs-spec` – Needs specification or detail
- `status:waiting-feedback` – Awaiting external feedback

---

## 🏷️ Type Labels

- `type:bug` – A defect or malfunction
- `type:ci` – CI/CD workflows and automation tasks
- `type:compliance` – Standards, audits, and certification checks (e.g., OpenSSF)
- `type:doc` – Documentation content
- `type:enhancement` – Enhancement of existing functionality
- `type:feature` – New feature or capability
- `type:infra` – Infrastructure, CI/CD, build system
- `type:legal` – Legal compliance tasks like DCO/CLA
- `type:refactor` – Code restructuring with no behavior change
- `type:release` – Tasks related to versioning, packaging, and publishing
- `type:security` – Vulnerability or security concern
- `type:test` – Test writing or coverage

---

## ✅ Best Practices

- Use exactly one `type:` label per issue/PR
- Use at most one `priority:` label
- Assign `status:` labels to track board movement
- Apply `module:` labels to indicate affected areas
- `default:unclassified` should be removed once other labels are assigned

---

This standardized labeling system enables GitHub automation, PR triage, and progress tracking across the hexaFn project.
