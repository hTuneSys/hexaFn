<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# 📚 Branching Strategy for hexaFn

This project follows a structured, scalable branching model inspired by GitHub Flow, Gitflow, and real-world enterprise project practices. All contributors are expected to adhere to the rules defined here for consistent development and stable releases.

---

## 🔱 Branch Layers

| Branch             | Purpose                                               |
|--------------------|-------------------------------------------------------|
| `main`             | Production branch. All releases are tagged from here. |
| `release/x.y`      | Pre-release stabilization and QA                     |
| `develop`          | Ongoing integration of all approved feature/fix PRs  |
| `feature/xyz`      | New feature development per issue                    |
| `fix/bug-id`       | Bugfix branches with isolated fixes                  |
| `hotfix/critical`  | Emergency fixes applied directly on top of `main`    |

---

## 🔄 Pull Request Flow

### 🔁 From contributors:
- All PRs must be based on `develop`
- Use `feature/your-feature`, `fix/your-bug` naming convention
- PR title must follow semantic commit format (`feat:`, `fix:`, etc.)

### ✅ Merge Rules:
- `feature/*` → `develop` (after review & CI pass)
- `fix/*` → `develop`
- `develop` → `release/x.y` (planned at sprint end)
- `release/x.y` → `main` (after test + approval)
- `hotfix/*` → `main` → `develop`

---

## 🔐 Branch Protection Rules

| Branch       | Protection                            |
|--------------|----------------------------------------|
| `main`       | ✅ Required PR, review, and status checks |
| `release/*`  | ✅ Only maintainers can merge          |
| `develop`    | ⚠️ PR required, at least 1 review      |
| others       | 🔓 No restriction, delete after merge  |

---

## 🗓️ Merge Schedule

| Action                      | Frequency            |
|-----------------------------|----------------------|
| `feature/*` → `develop`     | As soon as ready     |
| `develop` → `release/x.y`   | Weekly or per sprint |
| `release/x.y` → `main`      | After QA/approval    |
| `hotfix/*` → `main`         | Immediately if needed|

---

## 🧹 Cleanup Policy

- Merged `feature/` and `fix/` branches must be deleted after merge
- `release/` branches are deleted after tagging
- `hotfix/` branches are merged and deleted immediately

---

## 💡 Notes

- `semantic-release` runs **only on `main`**
- PR templates and commit conventions are strictly enforced
- Contributors must fork and target `develop` in their PRs

For questions, open a [discussion](https://github.com/hTuneSys/hexaFn/discussions)

---

> Maintained by @husamettinarabaci • hexaTune LLC
