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

## 📬 Contact & Support

For questions, feedback, or feature discussions, reach us at:  
📧 **info@hexafn.com**

You can also open a [GitHub Issue](https://github.com/hTuneSys/hexaFn/issues) to suggest new features or report bugs.

---

## 🧾 License

By contributing, you agree that your contributions will be licensed under the same license as the project: **MIT License**.

---

Thanks again for helping us build **hexaFn** —  
Together we turn events into logic.
