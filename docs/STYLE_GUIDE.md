<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# STYLE_GUIDE.md

This style guide ensures consistency across the codebase by outlining formatting rules, naming conventions, and Rust-specific practices used in the hexaFn project.

---

## 🧠 General Principles

- Prioritize **readability** and **maintainability**
- Follow **idiomatic Rust** patterns and conventions
- Keep functions **short**, **pure**, and **focused**
- Document public items with clear and concise comments

---

## 📦 File & Module Organization

- Each module resides in its own file
- File names use `snake_case` (e.g., `data_parser.rs`)
- Group related modules into folders
- Avoid deep nesting where possible

---

## 🧱 Naming Conventions

| Item           | Convention     | Example              |
|----------------|----------------|----------------------|
| Variables      | `snake_case`   | `user_name`          |
| Functions      | `snake_case`   | `fetch_data()`       |
| Structs        | `CamelCase`    | `UserProfile`        |
| Enums          | `CamelCase`    | `ResponseStatus`     |
| Traits         | `CamelCase`    | `Serializable`       |
| Constants      | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES`   |

---

## 🧼 Formatting

- Use `rustfmt` with the default configuration
- Run `cargo fmt` before pushing code
- Never manually align comments or long lines

---

## 🧪 Testing Conventions

- Test modules reside under the same file, inside `#[cfg(test)]`
- Name test functions clearly: `test_successful_login()`
- Use `assert_eq!`, `assert!`, or result-based matching

---

## 🪓 Error Handling

- Use `Result<T, E>` and proper error propagation
- Prefer descriptive custom error types with `thiserror`
- Avoid panics in library code

---

## 💡 Comments & Docs

- Use `///` for public documentation
- Use `//` for inline developer comments
- Avoid commented-out code

---

## 🔍 Linting

- Enforce with `cargo clippy`
- Fix all warnings unless explicitly justified
- Common lints to enable:
  - `clippy::unwrap_used`
  - `clippy::expect_used`
  - `clippy::wildcard_imports`

---

Consistent style improves collaboration and makes onboarding easier. Stick to this guide for clean, professional code
