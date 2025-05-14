<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# Contributing to hexaFn

Thank you for considering contributing to **hexaFn**!  
We welcome pull requests, feature proposals, documentation improvements, and plugin contributions.

---

## 📌 How to Contribute

### 1. 🧠 Understand the Vision
hexaFn is a modular, programmable event engine built on the **6F Lifecycle Flow**:  
**Feed → Filter → Format → Function → Forward → Feedback**

Before contributing, please read our [README](./README.md) to get familiar with the architecture and core modules.

---

### 2. 🛠 Code Contributions

- Fork this repository
- Create a feature branch (`git checkout -b feat/my-feature`)
- Make your changes with clear commits
- Run available tests (`cargo test`, etc.)
- Submit a pull request with a clear title and description

> Contributions that include tests and documentation will be prioritized.

---

### 3. 🧩 Plugin Contributions

We love modularity!  
If you're building a plugin (custom function, middleware, or bridge):

- Place it in `plugins/` or `extensions/` with a clear directory name
- Add a README in your plugin folder
- Explain:
  - What it does
  - What inputs it expects
  - Any dependencies or usage examples

---

### 4. 📝 Documentation Contributions

Found a typo, unclear part, or missing explanation?  
Feel free to:

- Suggest edits to the README
- Add a markdown doc under `/docs/`
- Improve code comments in Rust or DSL functions

---

### 5. 🤝 Code Style & Commit Guidelines

- Use `snake_case` for file names and `camelCase` or `PascalCase` for types/functions as per Rust standards
- Use clear and descriptive commit messages:
  - ✅ `feat: add trigger chaining logic`
  - ✅ `fix: handle null in input formatter`
- Keep PRs focused and small (1 feature or fix per PR)

---

## 1️⃣ Branch & Merge Policy

- Never push directly to `main`
- All changes must go through pull requests (PR)
- PRs require approval by project owners before merge
- Merge method: **squash and merge** preferred

---

## 2️⃣ Branch Naming Convention

Use the following pattern:
```
<type>/<module>/<short-description>
```
**Examples:**
- `feat/run/wasm-runtime`
- `fix/store/null-check`

Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`, `ci`, `build`, `style` 

---

## 3️⃣ Pull Request Guidelines

- Use the provided PR template
- Include related issue (e.g. `Closes #42`)
- Check off affected modules and checklist
- Ensure `cargo fmt`, `cargo test`, and `clippy` pass before submitting

---

## 4️⃣ Issue Guidelines

- Use the `bug report` or `feature request` templates
- Include logs, screenshots, or reproduction steps when possible

---

## 5️⃣ Commit Message Format (Semantic Release)

We use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to automate versioning and changelogs.

| Prefix | Meaning            | Version Impact     |
|--------|--------------------|--------------------|
| feat:  | Add a new feature  | minor version bump |
| fix:   | Fix a bug          | patch version bump |
| perf:  | Performance change | patch version bump |
| docs:  | Docs only          | no version change  |
| chore: | Build/config/meta  | no version change  |
| BREAKING CHANGE: | In footer | major version bump 🔥 |

### **Examples:**

#### ➕ Add a New Feature (minor)
```
feat(run): support WASM sandbox execution
```

#### 🐛 Fix a Bug (patch)
```
fix(store): resolve panic when key is empty
```

#### 📈 Improve Performance (patch)
```
perf(trigger): reduce evaluation loop overhead
```

#### 📚 Documentation Only (no bump)
```
docs: clarify HexaBridge usage in README
```

#### 🔧 Tooling Change (no bump)
```
chore(ci): switch to GitHub Actions for build
```

#### 💥 Breaking Change (major)
```
feat(store): introduce versioned keys

BREAKING CHANGE: old keys will no longer be recognized by the runtime.
```

---

## 6️⃣ Tests & CI/CD

- CI checks must pass before PR is reviewed
- Test coverage should not drop
- Use `cargo tarpaulin` to validate test coverage

---

## 7️⃣ Fork & PR Flow

1. Fork the repository
2. Create your feature branch from `main`
3. Commit using conventional format
4. Push to your fork and open a PR

We recommend tagging your PR with labels such as `type:feature`, `module:store`, etc.

---

## 📬 Contact & Support

For questions, feedback, or feature discussions, reach us at:  
📧 **info@hexafn.com**

You can also open a [GitHub Issue](https://github.com/hTuneSys/hexaFn/issues) to suggest new features or report bugs.

---

## ✅ Ground Rules

- All contributions must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
- Code must be formatted with `cargo fmt`
- All PRs must be reviewed and pass CI
- `main` is **protected** and used only for releases
- Contributions are made against the `develop` branch

---

## 🔄 Branch Strategy (Summary)

| Branch        | Role                                                                 |
|---------------|----------------------------------------------------------------------|
| `main`        | Production branch – semantic-release runs here                       |
| `develop`     | Active integration branch – PR base                                  |
| `feature/*`   | New features – merge into `develop`                                  |
| `fix/*`       | Bugfix branches – merge into `develop`                               |
| `release/*`   | QA-tested, stable pre-release – merged to `main` after approval      |
| `hotfix/*`    | Urgent fixes from `main` – immediately patched + backmerged          |

➡️ For full strategy see [`docs/branch-strategy.md`](./docs/branch-strategy.md)

---

## 🧪 How to Submit a PR

1. Fork the repository
2. Create your branch from `develop` using correct naming (`feature/`, `fix/`, etc.)
3. Make sure your changes pass CI and are formatted
4. Submit a pull request to `develop`

PRs must:
- Be clearly described
- Reference related issues (e.g., `Closes #42`)
- Use correct semantic commit format
- Tick module boxes in the PR template

---

## 🧾 License

By contributing, you agree that your contributions will be licensed under the same license as the project: **MIT License**.

---

Thanks again for helping us build **hexaFn** —  
Together we turn events into logic.
