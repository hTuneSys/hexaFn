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

### **REQUIRED**: Module Dependencies

- `hexafn-core`: Core module, all other modules may depend on this
- `hexafn-store`, `hexafn-cast`: May only depend on `hexafn-core`
- `hexafn-run`, `hexafn-trigger`, `hexafn-watch`, `hexafn-bridge`: May only depend on `hexafn-core`, `hexafn-store` and `hexafn-cast`

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

### **REQUIRED**: Code Documentation Standards

- ALL public functions MUST have documentation comments with `///`
- Documentation MUST explain purpose, parameters, return values, and errors
- Examples MUST be included for non-trivial functions
- Documentation MUST follow rustdoc conventions including `# Examples`, `# Errors`, etc.
- Internal functions SHOULD have documentation when complexity warrants it

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

---

## 🚨 MANDATORY LANGUAGE SUPPORT

### **REQUIRED**: Language Requirements

- ALL code, comments, and text in ANY file MUST be written in English only
- ALL variable/function/class names MUST use English words only
- ALL documentation and strings MUST be in English only
- ALL file content regardless of file type MUST NOT contain non-English text
- ALL commit messages and PR descriptions MUST be in English only

### **FORBIDDEN**: Non-English Content

- ❌ NEVER use non-English words in code, comments, or documentation
- ❌ NEVER use non-English variable or function names
- ❌ NEVER include non-English text in any project file (regardless of file type)
- ❌ NEVER submit PRs with non-English descriptions or commit messages
- ❌ NEVER include localized text directly in source files (use translation files/systems instead)

### **REQUIRED**: Chat Conversation Language Flexibility

- For direct chat conversations with users, GitHub Copilot MAY respond in the user's preferred language
- This exception ONLY applies to interactive chat conversations and verbal explanations in the chat panel
- ALL code generation, documentation files, PR descriptions, commit messages, and other project artifacts MUST still be in English only
- When explaining concepts or providing guidance in a non-English language, GitHub Copilot MUST clarify that any code or documentation contributions must follow the English-only requirement
- When sharing code snippets, terminal commands, or content intended for copying into files or editors, ALL comments and explanatory text within those snippets MUST also be in English only, even when the surrounding conversation is in another language
- The distinction between conversation (which may be in user's language) and actionable code/content (which must be in English) MUST be clear

### **PERMITTED**: Multi-language User Support

- ✅ Permitted: Responding to user questions in their native language during chat conversations
- ✅ Permitted: Explaining code functionality in user's preferred language
- ✅ Permitted: Providing troubleshooting guidance in user's preferred language
- ✅ Permitted: Answering project questions in user's preferred language

### **STILL FORBIDDEN**: Non-English Project Content

- ❌ NEVER generate non-English code or comments, regardless of conversation language
- ❌ NEVER create non-English documentation files, even if explaining in another language
- ❌ NEVER suggest non-English variable/function names
- ❌ NEVER propose non-English commit messages or PR descriptions
- ❌ NEVER include non-English text in code blocks, terminal commands, or content intended for copying

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

### **REQUIRED**: REUSE Compliance

- ALL files MUST have SPDX headers
- License information MUST be in `LICENSES/` directory
- REUSE.toml MUST be maintained for exceptions
- CI MUST validate REUSE compliance on every PR

### **REQUIRED**: GitHub Actions Integration

ALL code changes MUST pass these automated workflows:

- **Auto Assign**: [`auto-assign.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/auto-assign.yml) - Automatic PR assignment
- **Branch Name Check**: [`branch-name-check.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/branch-name-check.yml) - Semantic branch validation
- **CI**: [`ci.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/ci.yml) - Continuous Integration and Deployment
- **Commit Lint**: [`commitlint.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/commitlint.yml) - Conventional commits check
- **Auto Labeling**: [`labeler.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/labeler.yml) - Automatic label assignment
- **Main Pr Protection**: [`main-pr-protect.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/main-pr-protect.yml) - Main PR protection
- **Pr Title Check**: [`pr-title-check.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/pr-title-check.yml) - PR title validation
- **Release Pr Protection**: [`release-pr-protect.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/release-pr-protect.yml) - Release PR protection
- **Release**: [`release.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/release.yml) - Release workflow
- **Reuse Compliance**: [`reuse-lint.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/reuse-lint.yml) - REUSE compliance check
- **Stale Management**: [`stale.yml`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/stale.yml) - Issue/PR lifecycle management

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

## 🚨 MANDATORY DOCUMENTATION REFERENCE

#### **REQUIRED**: Project Knowledge Documents**

- [`docs/TODO_LIST.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/TODO_LIST.md) - **MANDATORY** for task tracking and sprint planning
- All files under [`docs/`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/) - **MANDATORY** for project context and architecture
- All files under [`.github/`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/) - **MANDATORY** for workflow and contribution guidelines
- [`README.md`](https://github.com/hTuneSys/hexaFn/blob/develop/README.md) - **MANDATORY** for project overview and introduction

### **REQUIRED**: Initial Document Analysis

When starting to assist with any hexaFn code task, GitHub Copilot MUST:

1. First analyze the content of `docs/TODO_LIST.md` to understand current tasks
2. Review relevant documentation in `docs/` directory related to the task
3. Check `.github/` workflow files and templates for process requirements
4. Reference the `README.md` for project context and 6F Lifecycle understanding

### **FORBIDDEN**: Working Without Documentation Context

❌ NEVER provide code suggestions without first analyzing:
- The task specifics in `docs/TODO_LIST.md`
- Related architecture docs in `docs/`
- Workflow requirements in `.github/`
- Project overview in `README.md`

### **REQUIRED**: Task ID Reference

When implementing features or fixes:
- Each PR MUST reference a task ID from `docs/TODO_LIST.md`
- Implementation MUST align with task description
- PR description MUST include `Closes #<ID>` or `Addresses #<ID>`
- Copilot MUST verify task alignment before suggesting implementation

### **REQUIRED**: Core Documentation Knowledge

GitHub Copilot MUST be familiar with and reference these documentation files when generating code or providing guidance:

#### **Critical Architecture Documents**

- [`docs/HEXAGONAL_ARCHITECTURE_GUIDE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/HEXAGONAL_ARCHITECTURE_GUIDE.md) - **MANDATORY** for all architectural decisions
- [`docs/ARCHITECTURE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/ARCHITECTURE.md) - **MANDATORY** System design principles and patterns
- [`docs/PROJECT_STRUCTURE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/PROJECT_STRUCTURE.md) - **MANDATORY** Workspace organization rules
- [`docs/DATA_FLOW_EXAMPLE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/DATA_FLOW_EXAMPLE.md) - **MANDATORY** Data flow patterns
- [`docs/DATA_FLOW_DETAIL_EXAMPLE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/DATA_FLOW_DETAIL_EXAMPLE.md) - **MANDATORY** Detailed component architecture
- [`README.md`](https://github.com/hTuneSys/hexaFn/blob/develop/README.md) - **MANDATORY** Project overview and 6F Lifecycle Flow

#### **Development Workflow Documents**

- [`docs/DEVELOPMENT_GUIDE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/DEVELOPMENT_GUIDE.md) - **MANDATORY** for development setup
- [`docs/BRANCH_STRATEGY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/BRANCH_STRATEGY.md) - **MANDATORY** for branch naming and workflow
- [`docs/COMMIT_STRATEGY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/COMMIT_STRATEGY.md) - **MANDATORY** for commit conventions
- [`docs/PR_STRATEGY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/PR_STRATEGY.md) - **MANDATORY** for pull request process
- [`docs/LABELLING_STRATEGY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/LABELLING_STRATEGY.md) - **MANDATORY** for issue/PR labeling

#### **Quality & Standards Documents**

- [`docs/STYLE_GUIDE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/STYLE_GUIDE.md) - **MANDATORY** for code formatting and style
- [`docs/CONTRIBUTING.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/CONTRIBUTING.md) - **MANDATORY** for contribution guidelines
- [`docs/GETTING_STARTED.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/GETTING_STARTED.md) - **MANDATORY** New developer onboarding
- [`docs/RUST_PORTS_ADAPTERS_EXAMPLE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/RUST_PORTS_ADAPTERS_EXAMPLE.md) - **MANDATORY** Rust implementation patterns

#### **GitHub Configuration Files**

- [`.github/PULL_REQUEST_TEMPLATE.md`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/PULL_REQUEST_TEMPLATE.md) - **MANDATORY** PR template structure
- [`.github/ISSUE_TEMPLATE/`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/ISSUE_TEMPLATE/) - **MANDATORY** issue template formats
- [`.github/workflows/`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/workflows/) - **MANDATORY** CI/CD pipeline knowledge
- [`.github/CODEOWNERS`](https://github.com/hTuneSys/hexaFn/blob/develop/.github/CODEOWNERS) - **MANDATORY** Code ownership and review requirements

#### **Project Management Documents**

- [`docs/ROADMAP.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/ROADMAP.md) - **MANDATORY** Project direction and milestones
- [`docs/MILESTONES.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/MILESTONES.md) - **MANDATORY** Specific milestone definitions
- [`docs/USE_CASES.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/USE_CASES.md) - **MANDATORY** Business context and scenarios
- [`docs/FAQ.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/FAQ.md) - **MANDATORY** Common questions and solutions
- [`docs/PROJECT_BOARD.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/PROJECT_BOARD.md) - **MANDATORY** Project management structure

#### **Configuration & Setup Documents**

- [`docs/CONFIGURATION.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/CONFIGURATION.md) - **MANDATORY** for environment setup
- [`Cargo.toml`](https://github.com/hTuneSys/hexaFn/blob/develop/Cargo.toml) - **MANDATORY** workspace dependencies
- [`REUSE.toml`](https://github.com/hTuneSys/hexaFn/blob/develop/REUSE.toml) - **MANDATORY** licensing compliance
- [`package.json`](https://github.com/hTuneSys/hexaFn/blob/develop/package.json) - **MANDATORY** Node.js tooling dependencies

#### **Compliance & Legal Documents**

- [`docs/SECURITY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/SECURITY.md) - **MANDATORY** security policies
- [`docs/CODE_OF_CONDUCT.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/CODE_OF_CONDUCT.md) - **MANDATORY** Community standards
- [`LICENSE`](https://github.com/hTuneSys/hexaFn/blob/develop/LICENSE) - **MANDATORY** MIT license text
- [`CHANGELOG.md`](https://github.com/hTuneSys/hexaFn/blob/develop/CHANGELOG.md) - **MANDATORY** Version history

#### **Community & Support Documents**

- [`docs/COMMUNITY.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/COMMUNITY.md) - **MANDATORY** Community guidelines
- [`docs/SUPPORT.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/SUPPORT.md) - **MANDATORY** Support procedures
- [`docs/CONTACT.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/CONTACT.md) - **MANDATORY** Contact information
- [`docs/BRANDING.md`](https://github.com/hTuneSys/hexaFn/blob/develop/docs/BRANDING.md) - **MANDATORY** Brand identity guidelines

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

- ✅ **Check HEXAGONAL_ARCHITECTURE_GUIDE.md** for structural decisions
- ✅ **Verify BRANCH_STRATEGY.md** for naming conventions  
- ✅ **Validate COMMIT_STRATEGY.md** for commit formats
- ✅ **Confirm STYLE_GUIDE.md** for code formatting
- ✅ **Reference USE_CASES.md** for business context
- ✅ **Follow PR_STRATEGY.md** for pull request guidance
- ✅ **Apply LABELLING_STRATEGY.md** for issue categorization
- ✅ **Review BRANDING.md** for brand identity guidelines
- ✅ **Consult COMMUNITY.md** for community interaction standards
- ✅ **Reference SUPPORT.md** for support procedures
- ✅ **Examine DATA_FLOW_DETAIL_EXAMPLE.md** for data flow patterns
- ✅ **Incorporate PROJECT_BOARD.md** for project management context
- ✅ **Study RUST_PORTS_ADAPTERS_EXAMPLE.md** for implementation patterns
- ✅ **Consider CONTACT.md** for communication channels

### **MANDATORY**: Documentation Update Notifications

When suggesting changes that might affect documentation:

- 🔔 **ALWAYS remind** to update relevant documentation
- 🔔 **Identify** which docs need updates
- 🔔 **Suggest** documentation changes alongside code changes
- 🔔 **Maintain** documentation-code consistency
- 🔔 **Ensure** cross-references remain valid across documentation
- 🔔 **Verify** that examples reflect current implementation patterns

---

## 🚨 DOCUMENTATION-DRIVEN DEVELOPMENT RULES

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

## 🚨 MANDATORY HEXAFN-SPECIFIC PATTERNS

### **REQUIRED**: 6F Lifecycle Implementation Standards

Each 6F phase MUST follow these implementation patterns:

- **Feed**: MUST implement `FeedInput<T>` trait with async support
- **Filter**: MUST use `FilterPredicate<T, E>` with Result-based validation  
- **Format**: MUST implement `Transformer<Input, Output>` trait
- **Function**: MUST use `FunctionRunner<Context, Result>` interface
- **Forward**: MUST implement `ForwardTarget<T>` with retry logic
- **Feedback**: MUST integrate with HexaWatch tracing system

### **REQUIRED**: 6F Pipeline Construction

ALL pipelines MUST use this builder pattern:

```rust
use hexafn_core::pipeline::Pipeline;

let pipeline = Pipeline::new()
    .feed(source)
    .filter(predicate) 
    .format(transformer)
    .function(runner)
    .forward(targets)
    .feedback(observer)
    .build()?;
```

### **FORBIDDEN**: Direct Phase Bypass

- ❌ NEVER skip phases in pipeline  
- ❌ NEVER implement phases outside their designated modules  
- ❌ NEVER bypass 6F lifecycle validation

---

## 🚨 MANDATORY DOMAIN EVENT PATTERNS

### **REQUIRED**: Domain Event Implementation

ALL domain events MUST follow this structure:

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

use hexafn_core::events::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerExecutedEvent {
    pub aggregate_id: TriggerId,
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub payload: TriggerExecutedPayload,
    pub version: u64,
}

impl DomainEvent for TriggerExecutedEvent {
    fn event_type(&self) -> &'static str { 
        "trigger.executed" 
    }
    
    fn aggregate_id(&self) -> &str { 
        &self.aggregate_id.0 
    }
    
    fn correlation_id(&self) -> &str {
        &self.event_id.0
    }
}
```

### **REQUIRED**: Cross-Module Communication

- Events MUST be published through HexaCast
- Event handlers MUST be in application layer  
- Domain layer MUST only generate events, not handle them
- Event names MUST follow pattern: `<module>.<action>`

### **FORBIDDEN**: Domain Event Anti-patterns

- ❌ NEVER handle events in domain layer  
- ❌ NEVER bypass event publishing through HexaCast  
- ❌ NEVER create circular event dependencies

---

## 🚨 MANDATORY ASYNC/PERFORMANCE PATTERNS

### **REQUIRED**: Async Implementation Standards

ALL async code MUST follow these patterns:

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

use tokio::{spawn, select, time::timeout};
use std::sync::Arc;

// Correct async pattern
async fn execute_pipeline(pipeline: Arc<Pipeline>) -> Result<Output, PipelineError> {
    let result = timeout(
        Duration::from_secs(30), 
        pipeline.execute()
    ).await??;
    
    Ok(result)
}

// Correct cancellation pattern
async fn cancellable_operation() -> Result<(), Error> {
    select! {
        result = long_running_task() => result,
        _ = shutdown_signal() => {
            info!("Operation cancelled by shutdown signal");
            Ok(())
        }
    }
}
```

### **REQUIRED**: Performance Constraints

- Pipeline processing MUST complete within 100ms for simple operations
- Memory usage MUST NOT exceed 50MB for standalone operations  
- Concurrent operations MUST be bounded (max 1000 concurrent pipelines)
- Error propagation MUST NOT block pipeline processing
- ALL async operations MUST have timeouts

### **REQUIRED**: Concurrency Patterns

- Use `tokio::spawn` for independent tasks
- Use `tokio::select!` for cancellation patterns
- Use `Arc<>` for shared state, `tokio::sync::Mutex<>` for mutable shared state
- Use `tokio::sync::mpsc` for bounded channels
- Implement graceful shutdown with `tokio::signal`

### **FORBIDDEN**: Performance Anti-patterns

- ❌ NEVER use `std::thread::sleep` in async code  
- ❌ NEVER block async runtime with CPU-intensive operations  
- ❌ NEVER use unbounded channels or queues  
- ❌ NEVER ignore timeout configurations  
- ❌ NEVER use `std::sync::Mutex` in async code

---

## 🚨 MANDATORY ERROR HANDLING PATTERNS

### **REQUIRED**: Error Type Hierarchy

ALL errors MUST follow this hierarchy:

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

use thiserror::Error;

// Module-level errors
#[derive(Error, Debug)]
pub enum TriggerError {
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Execution error: {source}")]
    Execution { 
        #[from] 
        source: ExecutionError 
    },
    
    #[error("Infrastructure error")]
    Infrastructure(#[from] InfrastructureError),
    
    #[error("Configuration error: {config}")]
    Configuration { config: String },
}

// Domain-level errors  
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Business rule violation: {rule}")]
    BusinessRuleViolation { rule: String },
    
    #[error("Aggregate not found: {id}")]
    AggregateNotFound { id: String },
    
    #[error("Invariant violation: {invariant}")]
    InvariantViolation { invariant: String },
}

// Application-level errors
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Use case failed: {use_case}")]
    UseCaseFailed { use_case: String },
    
    #[error("Authorization failed: {reason}")]
    AuthorizationFailed { reason: String },
    
    #[error("Domain error")]
    Domain(#[from] DomainError),
}
```

### **REQUIRED**: Error Context

- ALL errors MUST include structured context
- Error messages MUST be user-friendly (no debug info)
- Errors MUST be traced with correlation IDs
- Error chains MUST preserve original context
- Recovery strategies MUST be documented

### **REQUIRED**: Error Propagation

```rust
// Correct error propagation pattern
pub async fn execute_trigger(
    &self, 
    id: TriggerId
) -> Result<TriggerResult, ApplicationError> {
    let trigger = self.repository
        .find_by_id(id)
        .await
        .map_err(|e| ApplicationError::UseCaseFailed { 
            use_case: "find_trigger".to_string() 
        })?
        .ok_or_else(|| ApplicationError::Domain(
            DomainError::AggregateNotFound { 
                id: id.to_string() 
            }
        ))?;
    
    trigger.execute()
        .map_err(|e| ApplicationError::Domain(e.into()))
}
```

### **FORBIDDEN**: Error Anti-patterns

- ❌ NEVER use `unwrap()` or `expect()` in production code  
- ❌ NEVER ignore errors with `let _ =`  
- ❌ NEVER expose internal error details to external APIs  
- ❌ NEVER create generic "Something went wrong" errors

---

## 🚨 MANDATORY TESTING PATTERNS

### **REQUIRED**: Domain Layer Testing

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_creation_with_valid_config() {
        // Given: Valid trigger configuration
        let config = TriggerConfig::new("test-trigger", "timer:5s");
        
        // When: Creating trigger
        let result = Trigger::new(config);
        
        // Then: Trigger is created successfully
        assert!(result.is_ok());
        let trigger = result.unwrap();
        assert_eq!(trigger.name().value(), "test-trigger");
        assert!(trigger.is_active());
    }
    
    #[test]
    fn test_trigger_business_rule_validation() {
        // Given: Invalid configuration
        let config = TriggerConfig::new("", "invalid-timer");
        
        // When: Creating trigger
        let result = Trigger::new(config);
        
        // Then: Business rule violation
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::BusinessRuleViolation { .. }));
    }
}
```

### **REQUIRED**: Application Layer Testing (with mocks)

```rust
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_trigger_execution_service_success() {
        // Given: Mock repository
        let mut mock_repo = MockTriggerRepository::new();
        let trigger_id = TriggerId::new("test-trigger");
        let trigger = Trigger::new(TriggerConfig::new("test", "timer:1s")).unwrap();
        
        mock_repo
            .expect_find_by_id()
            .with(eq(trigger_id.clone()))
            .return_once(move |_| Ok(Some(trigger)));
            
        // Given: Mock event publisher
        let mut mock_publisher = MockEventPublisher::new();
        mock_publisher
            .expect_publish()
            .returning(|_| Ok(()));
            
        // When: Service execution
        let service = TriggerService::new(
            Arc::new(mock_repo),
            Arc::new(mock_publisher)
        );
        let result = service.execute_trigger(trigger_id).await;
        
        // Then: Execution succeeds
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_trigger_execution_service_not_found() {
        // Given: Mock repository returns None
        let mut mock_repo = MockTriggerRepository::new();
        let trigger_id = TriggerId::new("nonexistent");
        
        mock_repo
            .expect_find_by_id()
            .with(eq(trigger_id.clone()))
            .return_once(|_| Ok(None));
            
        let mock_publisher = MockEventPublisher::new();
        
        // When: Service execution
        let service = TriggerService::new(
            Arc::new(mock_repo),
            Arc::new(mock_publisher)
        );
        let result = service.execute_trigger(trigger_id).await;
        
        // Then: Returns not found error
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(), 
            ApplicationError::Domain(DomainError::AggregateNotFound { .. })
        ));
    }
}
```

### **REQUIRED**: Integration Testing

```rust
// integration-tests/trigger_pipeline_test.rs
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

use hexafn_trigger::*;
use hexafn_core::pipeline::*;

#[tokio::test]
async fn test_complete_trigger_pipeline_flow() {
    // Given: Complete 6F pipeline setup
    let pipeline = Pipeline::new()
        .feed(TimerFeed::new("5s"))
        .filter(AlwaysPassFilter::new())
        .format(JsonFormatter::new())
        .function(EchoFunction::new())
        .forward(LogForwarder::new())
        .feedback(TraceCollector::new())
        .build()
        .expect("Pipeline construction failed");
    
    // When: Pipeline execution
    let result = pipeline.execute().await;
    
    // Then: Pipeline completes successfully
    assert!(result.is_ok());
    
    // And: All phases executed
    let execution_log = result.unwrap();
    assert_eq!(execution_log.phases_executed(), 6);
    assert!(execution_log.contains_phase("feed"));
    assert!(execution_log.contains_phase("feedback"));
}

#[tokio::test]
async fn test_pipeline_error_propagation() {
    // Given: Pipeline with failing filter
    let pipeline = Pipeline::new()
        .feed(TestFeed::new())
        .filter(AlwaysFailFilter::new())
        .format(JsonFormatter::new())
        .function(EchoFunction::new())
        .forward(LogForwarder::new())
        .feedback(TraceCollector::new())
        .build()
        .expect("Pipeline construction failed");
    
    // When: Pipeline execution
    let result = pipeline.execute().await;
    
    // Then: Pipeline fails at filter stage
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(error, PipelineError::FilterFailed { .. }));
}
```

### **REQUIRED**: Performance Testing

```rust
// benchmarks/pipeline_throughput.rs
// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hexafn_core::pipeline::*;

fn benchmark_simple_pipeline(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("simple_pipeline_execution", |b| {
        b.to_async(&runtime).iter(|| async {
            let pipeline = Pipeline::new()
                .feed(TestFeed::new())
                .filter(AlwaysPassFilter::new())
                .format(NoOpFormatter::new())
                .function(NoOpFunction::new())
                .forward(NoOpForwarder::new())
                .feedback(NoOpCollector::new())
                .build()
                .unwrap();
            
            black_box(pipeline.execute().await.unwrap())
        })
    });
}

criterion_group!(benches, benchmark_simple_pipeline);
criterion_main!(benches);
```

### **FORBIDDEN**: Testing Anti-patterns

- ❌ NEVER test implementation details, only behavior  
- ❌ NEVER write tests without clear Given/When/Then structure  
- ❌ NEVER ignore async test warnings  
- ❌ NEVER test multiple concerns in a single test  
- ❌ NEVER use real external services in unit tests

---

## 🚨 MANDATORY DEVELOPMENT ENVIRONMENT

### **REQUIRED**: Local Development Tools

```bash
# Required tools installation
cargo install cargo-watch cargo-nextest cargo-audit
cargo install --locked trunk       # For WASM builds  
cargo install just                 # Task runner
cargo install cargo-make          # Alternative task runner
cargo install cargo-deny          # Dependency licensing checks

# Pre-commit hooks setup  
pip install pre-commit
pre-commit install
```

### **REQUIRED**: VS Code Extensions

Essential extensions for hexaFn development:

- `rust-analyzer` - Rust language support
- `Even Better TOML` - TOML file support  
- `GitLens` - Git integration
- `GitHub Copilot` - AI code assistance (if available)
- `Error Lens` - Inline error display
- `CodeLLDB` - Rust debugging
- `REST Client` - API testing

### **REQUIRED**: Development Environment Variables

```bash
# .env.development
HEXA_ENV=development
HEXA_LOG_LEVEL=debug
HEXA_STORE_BACKEND=memory
HEXA_CAST_BUFFER_SIZE=1000
HEXA_RUNTIME_TIMEOUT=30s
HEXA_ENABLE_TRACING=true
HEXA_METRICS_PORT=9090

# .env.test  
HEXA_ENV=test
HEXA_LOG_LEVEL=error
HEXA_STORE_BACKEND=memory
HEXA_CAST_BUFFER_SIZE=100
HEXA_RUNTIME_TIMEOUT=5s
HEXA_ENABLE_TRACING=false

# .env.production
HEXA_ENV=production
HEXA_LOG_LEVEL=info
HEXA_STORE_BACKEND=rocksdb
HEXA_CAST_BUFFER_SIZE=10000
HEXA_RUNTIME_TIMEOUT=60s
HEXA_ENABLE_TRACING=true
HEXA_METRICS_PORT=9090
```

### **REQUIRED**: Development Workflow Commands

```bash
# Development workflow
just dev-setup          # Initial setup
just build-all          # Build all crates
just test-all           # Run all tests  
just lint               # Run clippy and fmt
just bench              # Run benchmarks
just integration-test   # Run integration tests

# Watch mode for development
cargo watch -x check -x test -x run

# Advanced testing
cargo nextest run       # Faster test runner
cargo audit             # Security audit
cargo deny check        # License compliance
```

### **REQUIRED**: IDE Configuration

```json
// .vscode/settings.json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.allTargets": false,
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    },
    "files.watcherExclude": {
        "**/target/**": true
    }
}

// .vscode/launch.json for debugging
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch", 
            "name": "Debug hexaFn",
            "cargo": {
                "args": ["build", "--bin", "hexafn"],
                "filter": {
                    "name": "hexafn",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### **REQUIRED**: Git Hooks Configuration

```bash
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all --
        language: system
        types: [rust]
        
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        
      - id: reuse-lint
        name: REUSE compliance check
        entry: reuse lint
        language: system
        pass_filenames: false
```

### **FORBIDDEN**: Development Anti-patterns

- ❌ NEVER commit without running pre-commit hooks  
- ❌ NEVER skip cargo fmt before committing  
- ❌ NEVER ignore clippy warnings  
- ❌ NEVER commit with failing tests  
- ❌ NEVER use release mode for development  
- ❌ NEVER commit .env files with secrets

---

## 🚨 MANDATORY FORBIDDEN ACTIONS

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

**This documentation ecosystem is the FOUNDATION of hexaFn development. ALL Copilot assistance MUST respect and reference these established standards.**
