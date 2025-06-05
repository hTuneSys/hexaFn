<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# GitHub Copilot Instructions for hexaFn

## ⚡ PROJECT OVERVIEW

**hexaFn** is a modular, event-driven function pipeline framework powered by the **6F Lifecycle Flow**: **Feed → Filter → Format → Function → Forward → Feedback**. This project follows **Hexagonal Architecture (Ports & Adapters)** with **Domain-Driven Design (DDD)** principles.

**Company**: hexaTune LLC  
**Maintainer**: Husamettin ARABACI (@husamettinarabaci)  
**License**: MIT (SPDX-compliant, REUSE Spec 3.3 compatible)  
**Repository**: <https://github.com/hTuneSys/hexaFn>

---

## 🚨 CRITICAL ARCHITECTURE REQUIREMENTS

### Hexagonal Architecture Structure (for all modules EXCEPT hexafn-core)

```plaintext
crates/<domain>/src/
├── domain/              # Core business rules (entities, value objects, domain services)
│   ├── entities/        # Aggregates and core business objects with identity
│   ├── value_objects/   # Immutable value types without identity
│   ├── events/          # Domain events for cross-domain communication
│   ├── services/        # Domain services containing business logic
│   └── contracts/       # Domain contracts and interfaces
├── application/         # Use cases, application services, port definitions
│   ├── commands/        # State-changing operations (CQRS Commands)
│   ├── queries/         # Read operations (CQRS Queries)
│   ├── ports/           # Interface definitions (dependency inversion)
│   └── services/        # Application orchestration services
├── infrastructure/      # Concrete implementations of output ports
│   ├── persistence/     # Database adapters, repositories
│   ├── messaging/       # Event publishing, message queue adapters
│   └── external/        # External API clients, third-party integrations
└── lib.rs               # Crate entry point (re-exports and public API)
```

### 6F Lifecycle Flow Integration

1. **Feed**: Ingest from external sources (events, APIs, queues)
2. **Filter**: Pre-condition checks and gating
3. **Format**: Normalize, transform, validate
4. **Function**: Execute logic with user-defined behavior
5. **Forward**: Route results to KV stores, topics, services
6. **Feedback**: Log, trace, trigger, or audit

### Module Boundaries

- `hexafn-core`: Core architecture & event flow ( Shared Kernel )
- `hexafn-store`: HexaStore (typed KV storage with triggers)
- `hexafn-cast`: HexaCast (pub-sub messaging engine)
- `hexafn-run`: HexaRun (function runtime: WASM, JS, DSL)
- `hexafn-trigger`: HexaTrigger (conditional logic & rule engine)
- `hexafn-watch`: HexaWatch (observability & audit tracing)
- `hexafn-bridge`: HexaBridge (external integrations & webhooks)

---

## 🚨 MANDATORY DOCUMENTATION REFERENCE

- [`docs/TODO_LIST.md`](../docs/TODO_LIST.md) - **MANDATORY** for task tracking and sprint planning
- [`docs/DATA_MODEL_CORE.md`](../docs/DATA_MODEL_CORE.md) - **MANDATORY** Data Core model and domain concepts
- [`docs/DATA_MODEL_RUN.md`](../docs/DATA_MODEL_RUN.md) - **MANDATORY** Data Run model and domain concepts
- [`docs/DATA_MODEL_CAST.md`](../docs/DATA_MODEL_CAST.md) - **MANDATORY** Data Cast model and domain concepts
- [`docs/DATA_MODEL_BRIDGE.md`](../docs/DATA_MODEL_BRIDGE.md) - **MANDATORY** Data Bridge model and domain concepts
- [`docs/DATA_MODEL_TRIGGER.md`](../docs/DATA_MODEL_TRIGGER.md) - **MANDATORY** Data Trigger model and domain concepts
- [`docs/DATA_MODEL_STORE.md`](../docs/DATA_MODEL_STORE.md) - **MANDATORY** Data Store model and domain concepts
- [`docs/DATA_MODEL_WATCH.md`](../docs/DATA_MODEL_WATCH.md) - **MANDATORY** Data Watch model and domain concepts
- [`docs/DATA_FLOW.md`](../docs/DATA_FLOW.md) - **MANDATORY** Data flow patterns
- [`docs/DATA_FLOW_DETAIL.md`](../docs/DATA_FLOW_DETAIL.md) - **MANDATORY** Detailed component architecture
- [`docs/BRANCH_STRATEGY.md`](../docs/BRANCH_STRATEGY.md) - **MANDATORY** for branch naming and workflow
- [`docs/COMMIT_STRATEGY.md`](../docs/COMMIT_STRATEGY.md) - **MANDATORY** for commit conventions
- [`docs/PR_STRATEGY.md`](../docs/PR_STRATEGY.md) - **MANDATORY** for pull request process
- [`docs/LABELLING_STRATEGY.md`](../docs/LABELLING_STRATEGY.md) - **MANDATORY** for issue/PR labeling

---

## 🚨 MANDATORY COMMIT & BRANCH RULES

### **REQUIRED**: Commit Message Format

```text
<type>(optional-scope): <short summary>
```

### **REQUIRED**: Allowed Commit Types ONLY

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only changes
- `style`: Code style, formatting (no logic change)
- `refactor`: Code refactoring (no feature change)
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `ci`: Changes to CI/CD configuration
- `build`: Build system or dependencies
- `chore`: Maintenance or tooling
- `release`: Versioning, changelog updates
- `hotfix`: Emergency fix for critical issues

### **REQUIRED**: Branch Naming Convention

Branch names MUST follow: `<type>/<description>`

If there is a related issue ID, it should be appended at the end: `<type>/<description>-<issueID>`

**Allowed prefixes ONLY**:

- `feat/`: New features
- `fix/`: Bug fixes
- `chore/`: Routine tasks, maintenance
- `refactor/`: Code refactoring
- `test/`: Test additions and modifications
- `docs/`: Documentation improvements
- `ci/`: CI/CD and automation scripts
- `perf/`: Performance improvements
- `build/`: Build-related changes
- `hotfix/`: Emergency fixes
- `style/`: Code style and formatting
- `develop`: Integration branch (protected)

### **REQUIRED**: Branch Flow

- ALL PRs MUST target `develop` branch (NOT `main`)
- `main` is production-only and protected
- `release/*` branches are for QA and protected
- `hotfix/*` branches target `main` directly

---

## 🚨 MANDATORY LABELING SYSTEM

### **REQUIRED**: Module Labels

**Allowed Labels ONLY**:

- `module:bridge`: HexaBridge changes
- `module:cast`: HexaCast changes
- `module:core`: Core architecture changes
- `module:docs`: Documentation changes
- `module:run`: HexaRun changes
- `module:store`: HexaStore changes
- `module:trigger`: HexaTrigger changes
- `module:watch`: HexaWatch changes
- `module:cli`: CLI tools changes

### **REQUIRED**: Type Labels

**Allowed Labels ONLY**:

- `type:feature`: New feature or capability
- `type:bug`: Defect or malfunction
- `type:doc`: Documentation content
- `type:enhancement`: Enhancement of existing functionality
- `type:refactor`: Code restructuring with no behavior change
- `type:test`: Test writing or coverage
- `type:ci`: CI/CD workflows and automation
- `type:infra`: Infrastructure, build system
- `type:security`: Vulnerability or security concern
- `type:compliance`: Standards, audits, certification checks
- `type:release`: Versioning, packaging, publishing
- `type:legal`: Legal compliance (DCO/CLA)

### **REQUIRED**: Priority Labels

**Allowed Labels ONLY**:

- `priority:high`: Urgent and critical tasks
- `priority:medium`: Normal importance
- `priority:low`: Can wait

---

## 🚨 MANDATORY PR REQUIREMENTS

### **REQUIRED**: PR Title Format

PR titles MUST follow conventional commits format:

```text
<type>(optional-scope): <description>
```

### **REQUIRED**: PR Checklist

EVERY PR MUST have:

- [ ] Valid semantic title with allowed type
- [ ] Related issue linked (if applicable)
- [ ] Description explains context and purpose
- [ ] No unrelated changes or mixed concerns
- [ ] All CI checks pass (format, lint, test)
- [ ] Appropriate labels applied
- [ ] SPDX headers in new files

### **REQUIRED**: Branch Protection

- PRs targeting `main` or `release/*` MUST have maintainer review
- ALL PRs MUST pass CI status checks
- Branch must be up-to-date before merge
- Merged branches MUST be deleted immediately

---

## 🚨 MANDATORY SPDX HEADERS

### **REQUIRED**: SPDX Headers for All File Types

EVERY new file MUST start with appropriate SPDX headers:

#### **Rust Files (.rs)**

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT
```

#### **TOML Files (.toml)**

```toml
# SPDX-FileCopyrightText: 2025 Husamettin ARABACI
# SPDX-License-Identifier: MIT
```

#### **Markdown Files (.md)**

```markdown
<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->
```

#### **YAML Files (.yml, .yaml)**

```yaml
# SPDX-FileCopyrightText: 2025 Husamettin ARABACI
# SPDX-License-Identifier: MIT
```

#### **JSON Files (.json)**

```json
{
  "_comment": "SPDX-FileCopyrightText: 2025 Husamettin ARABACI",
  "_license": "SPDX-License-Identifier: MIT"
}
```

### **FORBIDDEN**: Missing SPDX Headers

- ❌ NEVER create files without proper SPDX headers
- ❌ NEVER use incorrect comment syntax for file type
- ❌ NEVER skip license identification
