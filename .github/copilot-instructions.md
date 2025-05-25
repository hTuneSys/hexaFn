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

### **REQUIRED**: GitHub Actions Integration

ALL code changes MUST pass these automated workflows:

- **Auto-assign**: [`auto-assign.yml`](.github/workflows/auto-assign.yml) - Automatic PR assignment
- **Branch protection**: [`branch-name-check.yml`](.github/workflows/branch-name-check.yml) - Semantic branch validation
- **Commit validation**: [`commitlint.yml`](.github/workflows/commitlint.yml) - Conventional commits check
- **Auto-labeling**: [`labeler.yml`](.github/workflows/labeler.yml) - Automatic label assignment
- **Stale management**: [`stale.yml`](.github/workflows/stale.yml) - Issue/PR lifecycle management

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

## 📚 MANDATORY DOCUMENTATION REFERENCE

### **REQUIRED**: Core Documentation Knowledge

GitHub Copilot MUST be familiar with and reference these documentation files when generating code or providing guidance:

#### **Critical Architecture Documents**

- [`docs/HEXAGONAL_ARCHITECTURE_GUIDE.md`](../docs/HEXAGONAL_ARCHITECTURE_GUIDE.md) - **MANDATORY** for all architectural decisions
- [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) - System design principles and patterns
- [`docs/PROJECT_STRUCTURE.md`](../docs/PROJECT_STRUCTURE.md) - Workspace organization rules
- [`README.md`](../../README.md) - Project overview and 6F Lifecycle Flow

#### **Development Workflow Documents**

- [`docs/DEVELOPMENT_GUIDE.md`](../docs/DEVELOPMENT_GUIDE.md) - **MANDATORY** for development setup
- [`docs/BRANCH_STRATEGY.md`](../docs/BRANCH_STRATEGY.md) - **MANDATORY** for branch naming and workflow
- [`docs/COMMIT_STRATEGY.md`](../docs/COMMIT_STRATEGY.md) - **MANDATORY** for commit conventions
- [`docs/PR_STRATEGY.md`](../docs/PR_STRATEGY.md) - **MANDATORY** for pull request process
- [`docs/LABELLING_STRATEGY.md`](../docs/LABELLING_STRATEGY.md) - **MANDATORY** for issue/PR labeling

#### **Quality & Standards Documents**

- [`docs/STYLE_GUIDE.md`](../docs/STYLE_GUIDE.md) - **MANDATORY** for code formatting and style
- [`docs/CONTRIBUTING.md`](../docs/CONTRIBUTING.md) - **MANDATORY** for contribution guidelines
- [`.github/CONTRIBUTING.md`](CONTRIBUTING.md) - GitHub-specific contribution rules
- [`docs/GETTING_STARTED.md`](../docs/GETTING_STARTED.md) - New developer onboarding

#### **GitHub Configuration Files**

- [`.github/PULL_REQUEST_TEMPLATE.md`](PULL_REQUEST_TEMPLATE.md) - **MANDATORY** PR template structure
- [`.github/ISSUE_TEMPLATE/`](ISSUE_TEMPLATE/) - **MANDATORY** issue template formats
- [`.github/workflows/`](workflows/) - **MANDATORY** CI/CD pipeline knowledge
- [`.github/CODEOWNERS`](CODEOWNERS) - Code ownership and review requirements

#### **Project Management Documents**

- [`docs/ROADMAP.md`](../docs/ROADMAP.md) - Project direction and milestones
- [`docs/MILESTONES.md`](../docs/MILESTONES.md) - Specific milestone definitions
- [`docs/USE_CASES.md`](../docs/USE_CASES.md) - Business context and scenarios
- [`docs/FAQ.md`](../docs/FAQ.md) - Common questions and solutions

#### **Configuration & Setup Documents**

- [`docs/CONFIGURATION.md`](../docs/CONFIGURATION.md) - **MANDATORY** for environment setup
- [`Cargo.toml`](../../Cargo.toml) - **MANDATORY** workspace dependencies
- [`REUSE.toml`](../../REUSE.toml) - **MANDATORY** licensing compliance
- [`package.json`](../../package.json) - Node.js tooling dependencies

#### **Compliance & Legal Documents**

- [`docs/SECURITY.md`](../docs/SECURITY.md) - **MANDATORY** security policies
- [`docs/CODE_OF_CONDUCT.md`](../docs/CODE_OF_CONDUCT.md) - Community standards
- [`LICENSE`](../../LICENSE) - MIT license text
- [`CHANGELOG.md`](../../CHANGELOG.md) - Version history

### **CRITICAL IMPORTANCE**: Documentation-First Engineering

🚨 **ALL GitHub Copilot responses MUST:**

1. **Reference relevant documentation** when providing guidance
2. **Align with documented standards** in all suggestions
3. **Cross-reference multiple docs** for comprehensive answers
4. **Maintain consistency** with established patterns
5. **Update suggestions** when documentation conflicts arise

### **FORBIDDEN**: Documentation Ignorance

❌ **NEVER provide suggestions that:**

- Contradict documented standards
- Ignore established workflows
- Skip required processes
- Violate architectural principles
- Bypass security policies

### **REQUIRED**: Documentation Cross-Reference

When generating code or guidance, Copilot MUST:

✅ **Check HEXAGONAL_ARCHITECTURE_GUIDE.md** for structural decisions
✅ **Verify BRANCH_STRATEGY.md** for naming conventions  
✅ **Validate COMMIT_STRATEGY.md** for commit formats
✅ **Confirm STYLE_GUIDE.md** for code formatting
✅ **Reference USE_CASES.md** for business context
✅ **Follow PR_STRATEGY.md** for pull request guidance
✅ **Apply LABELLING_STRATEGY.md** for issue categorization

### **MANDATORY**: Documentation Update Notifications

When suggesting changes that might affect documentation:

🔔 **ALWAYS remind** to update relevant documentation
🔔 **Identify** which docs need updates
🔔 **Suggest** documentation changes alongside code changes
🔔 **Maintain** documentation-code consistency

---

## 🎯 DOCUMENTATION-DRIVEN DEVELOPMENT RULES

### **REQUIRED**: Documentation Precedence

1. **Documentation defines behavior** - Code implements it
2. **When in doubt, follow the docs** - Don't guess or assume
3. **Documentation updates** MUST accompany significant changes
4. **Cross-references** MUST be maintained between related docs
5. **Examples** in documentation MUST be kept up-to-date

### **REQUIRED**: GitHub Copilot Behavior

GitHub Copilot responses MUST:

- 📖 **Quote relevant documentation** sections when applicable
- 🔗 **Provide documentation links** for further reading  
- ⚖️ **Balance documentation compliance** with practical solutions
- 🔄 **Suggest documentation updates** when gaps are identified
- 📋 **Reference templates and examples** from documentation

---

**This documentation ecosystem is the FOUNDATION of hexaFn development. ALL Copilot assistance MUST respect and reference these established standards.**
