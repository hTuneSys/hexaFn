<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# GitHub Copilot Instructions for hexaFn

## ⚡ REQUIRED PROJECT OVERVIEW

**hexaFn** is a modular, event-driven function pipeline framework powered by the **6F Lifecycle Flow**: **Feed → Filter → Format → Function → Forward → Feedback**. This project follows **Hexagonal Architecture (Ports & Adapters)** with **Domain-Driven Design (DDD)** principles using Rust.

**Company**: hexaTune LLC  
**Maintainer**: Husamettin ARABACI (@husamettinarabaci)  
**License**: MIT (SPDX-compliant, REUSE Spec 3.3 compatible)  
**Repository**: <https://github.com/hTuneSys/hexaFn>

---

## 🚨 CRITICAL ARCHITECTURE REQUIREMENTS

### **REQUIRED**: Hexagonal Architecture Structure

Every code change MUST follow this structure within crates:

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

### **REQUIRED**: 6F Lifecycle Flow Integration

ALL features MUST align with one or more of these phases:

1. **Feed**: Ingest from external sources (events, APIs, queues)
2. **Filter**: Pre-condition checks and gating
3. **Format**: Normalize, transform, validate
4. **Function**: Execute logic with user-defined behavior
5. **Forward**: Route results to KV stores, topics, services
6. **Feedback**: Log, trace, trigger, or audit

### **REQUIRED**: Module Boundaries

These modules are REQUIRED and MUST NOT be mixed:

- `hexafn-core`: Core architecture & event flow
- `hexafn-store`: HexaStore (typed KV storage with triggers)
- `hexafn-cast`: HexaCast (pub-sub messaging engine)
- `hexafn-run`: HexaRun (function runtime: WASM, JS, DSL)
- `hexafn-trigger`: HexaTrigger (conditional logic & rule engine)
- `hexafn-watch`: HexaWatch (observability & audit tracing)
- `hexafn-bridge`: HexaBridge (external integrations & webhooks)

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

## 🚨 MANDATORY RUST CODE STANDARDS

### **REQUIRED**: Naming Conventions

- Variables/Functions: `snake_case` (e.g., `user_name`, `fetch_data()`)
- Structs/Enums/Traits: `CamelCase` (e.g., `UserProfile`, `ResponseStatus`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`)
- Files/Modules: `snake_case` (e.g., `data_parser.rs`)

### **REQUIRED**: Code Quality Rules

- ALL code MUST pass `cargo fmt` and `cargo clippy`
- NO `unwrap()` or `expect()` in library code - use proper error handling
- ALL public items MUST have documentation with `///`
- Use `Result<T, E>` for error propagation
- Prefer descriptive custom error types with `thiserror`
- ALL tests MUST be in `#[cfg(test)]` modules

### **REQUIRED**: SPDX Headers

EVERY new file MUST start with:

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT
```

---

## 🚨 MANDATORY LABELING SYSTEM

### **REQUIRED**: Module Labels (apply ONE)

- `module:bridge`: HexaBridge changes
- `module:cast`: HexaCast changes
- `module:core`: Core architecture changes
- `module:docs`: Documentation changes
- `module:run`: HexaRun changes
- `module:store`: HexaStore changes
- `module:trigger`: HexaTrigger changes
- `module:watch`: HexaWatch changes
- `module:cli`: CLI tools changes

### **REQUIRED**: Type Labels (apply ONE)

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

### **REQUIRED**: Priority Labels (apply ONE if needed)

- `priority:high`: Urgent and critical tasks
- `priority:medium`: Normal importance
- `priority:low`: Can wait

---

## 🚨 MANDATORY PR REQUIREMENTS

### **REQUIRED**: PR Title Format

PR titles MUST follow conventional commits format:

```text
<type>: <description>
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

## 🚨 MANDATORY DEVELOPMENT WORKFLOW

### **REQUIRED**: Code Changes Process

1. Branch from `develop` with semantic name
2. Follow hexagonal architecture structure
3. Add SPDX headers to new files
4. Write tests for new functionality
5. Run `cargo fmt` and `cargo clippy`
6. Create PR with semantic title
7. Apply appropriate labels
8. Request review from maintainers
9. Address all review comments
10. Merge after CI passes and approval

### **REQUIRED**: Testing Standards

- ALL new features MUST have unit tests
- Integration tests MUST be in `integration-tests/` directory
- Performance tests MUST be in `benchmarks/` directory
- Test functions MUST use clear naming: `test_<functionality>()`

### **REQUIRED**: Documentation Standards

- ALL public APIs MUST be documented with `///`
- README files MUST be present in each crate
- Architecture changes MUST update relevant docs in `docs/`
- Examples MUST be provided for new features

---

## 🚨 MANDATORY CI/CD REQUIREMENTS

### **REQUIRED**: Automated Checks

ALL PRs MUST pass these checks:

- **Formatting**: `cargo fmt --check`
- **Linting**: `cargo clippy`
- **Testing**: `cargo test`
- **REUSE Compliance**: REUSE spec validation
- **Branch Naming**: Semantic branch name validation
- **Commit Linting**: Conventional commits validation
- **PR Title**: Semantic PR title validation

### **REQUIRED**: REUSE Compliance

- ALL files MUST have SPDX headers
- License information MUST be in `LICENSES/` directory
- REUSE.toml MUST be maintained for exceptions
- CI MUST validate REUSE compliance on every PR

---

## 🚨 MANDATORY SECURITY & COMPLIANCE

### **REQUIRED**: Security Practices

- NO hardcoded secrets or credentials
- ALL dependencies MUST be regularly updated via Dependabot
- Security vulnerabilities MUST be reported to <info@hexafn.com>
- ALL external inputs MUST be validated
- Error messages MUST NOT leak sensitive information

### **REQUIRED**: Code Ownership

- Default owner: @husamettinarabaci
- Module-specific owners defined in CODEOWNERS
- ALL changes require owner approval for merge

---

## 🚨 FORBIDDEN ACTIONS

### **NEVER** do these

- ❌ Push directly to `main` or `release/*` branches
- ❌ Use commit types not in the allowed list
- ❌ Skip SPDX headers in new files
- ❌ Use `unwrap()` or `expect()` in library code
- ❌ Mix concerns across modules or domains
- ❌ Submit PRs without tests for new functionality
- ❌ Ignore CI failures or clippy warnings
- ❌ Create branches without semantic prefixes
- ❌ Skip code formatting with `cargo fmt`
- ❌ Leave merged branches undeleted
- ❌ Submit PRs targeting `main` directly
- ❌ Use non-conventional commit messages
- ❌ Violate hexagonal architecture boundaries

---

## 🚨 PROJECT-SPECIFIC CONTEXT

### **REQUIRED**: Business Domain Knowledge

- hexaFn is for automation, serverless actions, messaging systems
- Target users: Backend architects, realtime hackers, automation engineers
- NOT a: Database, monolith, batch processor, CMS, frontend framework
- Focus: Event-driven, real-time, programmable data pipelines

### **REQUIRED**: Technical Context

- Runtime: Rust with async/await
- Architecture: Hexagonal + DDD + Event-Driven
- Testing: Unit + Integration + Benchmarks
- Deployment: Crates ecosystem + CLI tools
- Dependencies: Minimize and justify all external crates

### **REQUIRED**: Quality Standards

- Documentation-first engineering approach
- All features must be composable and testable
- Performance and memory efficiency are critical
- Developer experience and API clarity are priorities
- Full compatibility with REUSE specification 3.3

---

**These instructions are REQUIRED and NON-NEGOTIABLE for all contributions to hexaFn.**
