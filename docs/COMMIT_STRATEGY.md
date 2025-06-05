<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# COMMIT_STRATEGY.md

## 🔖 Commit Message Strategy for hexaFn

To ensure consistent and readable commit messages across the project, **hexaFn** enforces the use of a standardized commit format and a predefined set of commit types.

---

## ✅ Commit Format

Each commit message must follow this format:

```text
<type>(optional-scope): <short summary>
```

---

## 🧩 Supported Commit Types with Examples

### 1. **feat** — A new feature for the user or system

```text
feat: add dark mode support
feat(auth): implement token refresh logic
feat(ui): create responsive navbar component
```

### 2. **fix** — A bug fix that impacts user behavior

```text
fix: resolve crash on empty input
fix(api): correct 500 error on login
fix(auth): fix token expiration handling
```

### 3. **docs** — Changes to documentation only

```text
docs: update README with setup instructions
docs(contributing): add issue template section
docs(api): fix typo in endpoint descriptions
```

### 4. **style** — Code style changes (formatting, white-space, etc.)

```text
style: reformat code with Prettier
style(auth): fix indentation
style: remove unnecessary semicolons
```

### 5. **refactor** — Code changes that neither fix a bug nor add a feature

```text
refactor: simplify state management logic
refactor(db): reorganize model definitions
refactor: extract common utility functions
```

### 6. **perf** — A code change that improves performance

```text
perf(api): cache frequently accessed data
perf: reduce image load time by optimizing assets
perf(ui): memoize expensive component renders
```

### 7. **test** — Adding or updating tests

```text
test: add unit tests for user service
test(auth): improve test coverage for login flow
test: mock API for integration tests
```

### 8. **chore** — Routine tasks like build process, dependencies

```text
chore: update project dependencies
chore(lint): add eslint config
chore: clean up temporary files
```

### 9. **ci** — CI/CD configuration and scripts

```text
ci: add GitHub Actions workflow
ci: fix broken CI script
ci: integrate code coverage reporting
```

### 10. **build** — Changes to build system or dependencies

```text
build: switch to Vite from Webpack
build: update TypeScript compiler settings
build: configure output directory for deployment
```

### 11. **release** — Versioning or release-related changes

```text
release: bump version to 1.2.0
release: update changelog for new release
release: prepare hotfix 1.2.1
```

### 12. **hotfix** — Urgent fixes, often related to production issues

```text
hotfix: fix production crash on launch
hotfix(auth): patch login issue in prod
hotfix(api): temporary rollback for breaking endpoint
```

---

## 🚫 What to Avoid

- Do **not** use generic commit messages like "update" or "fix bug".
- Do **not** push without proper commit type and summary.
- Do **not** include long descriptions in the title.

---

## 🛠 Tools & Automation

This strategy is supported by tools such as:

- **commitlint** for linting commit messages.
- **Husky** to enforce commit rules pre-commit.
- **Semantic Release** (optional) for automated versioning based on commit messages.

---

## 💡 Tips

- Use imperative mood: "add" not "added" or "adds".
- Keep summary under 72 characters.
- Reference issues when necessary (e.g., `fix: resolve #42`).

---
