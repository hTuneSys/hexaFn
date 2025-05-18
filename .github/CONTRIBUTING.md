<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# Contributing to hexaFn

Thank you for your interest in contributing to **hexaFn**!  
This document outlines how to get involved, contribute code or ideas, and follow our development process.

---

## 📚 Before You Start

Please review the following core documents:

- [Getting Started](./docs/getting-started.md)
- [Architecture](./docs/architecture.md)
- [Branch Strategy](./docs/branch-strategy.md)
- [Project Board Workflow](./docs/project-board.md)

Also, check the [README](./README.md) and [FAQ](./FAQ.md) for general project info.

---

## 🧩 Contribution Types

- **Code:** Features, bugfixes, improvements to modules or core
- **Plugins:** External integrations or lifecycle extensions
- **Docs:** Fixing typos, improving explanations, or new guides
- **Issues & Ideas:** File an issue, open a discussion

---

## 🔀 Branch Strategy

Please follow our [Branching Guide](./docs/branch-strategy.md).  
Key rules:
- Use `feature/xyz`, `fix/bug-123`, `docs/readme-update` formats
- Always branch from `develop`, never from `main`
- Only maintainers merge to `main` after release approval

---

## 🧪 PR Flow & Project Board

All contributions are tracked on the [hexaFn Kanban Board](./docs/project-board.md):

1. Pick a task from 📥 Inbox or 📝 To Do
2. Create a PR targeting `develop`
3. Your PR moves through:
   - 🚧 In Progress
   - 🔎 In Review
   - ✅ Done

---

## ✏️ Commit & PR Formatting

We use **[Conventional Commits](https://www.conventionalcommits.org/)**:
- `feat: add trigger matching system`
- `fix: prevent crash when KV not found`
- `docs: update lifecycle diagram`
- `refactor: modularize filter logic`

PR titles **must** follow the same format and pass automated title linting.

---

## 🏷 Label System

| Label              | Purpose                                 |
|--------------------|------------------------------------------|
| `good first issue` | Safe for new contributors                |
| `bug`              | Unexpected behavior or error             |
| `enhancement`      | Feature request or improvement           |
| `breaking`         | API-breaking or behavioral change        |
| `infra`            | DevOps / build / GitHub Actions changes  |
| `docs`             | Documentation-only change                |
| `security`         | Vulnerability or security concern        |

---

## 🔁 CI/CD & Releases

- All PRs must pass CI tests and formatting checks
- `main` is protected; `develop` is the integration branch
- Merges to `main` trigger semantic-release for changelog & versioning

---

## 🤝 Communication

- Use [GitHub Discussions](https://github.com/hTuneSys/hexaFn/discussions)
- For sensitive issues, contact: info@hexafn.com
- Be kind, clear, and collaborative – we follow our [Code of Conduct](./CODE_OF_CONDUCT.md)

We’re excited to build with you 🚀
