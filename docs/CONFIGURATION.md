<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# CONFIGURATION.md

This document explains how configuration is managed across the hexaFn project. It includes environment variables, default settings, runtime parameters, and CI/CD setup.

---

## 1. Overview

hexaFn uses environment variables and structured config files to control runtime behavior, development flags, and integrations. All configurations are centralized to ensure transparency and easy modification.

---

## 2. Environment Variables

Environment-specific settings can be defined in a `.env` file or exported manually in the shell.

| Variable           | Description                          | Default        |
|--------------------|--------------------------------------|----------------|
| `HEXA_ENV`         | Application mode (`dev`, `prod`)     | `dev`          |
| `HEXA_PORT`        | Port used by the local server        | `3000`         |
| `HEXA_DEBUG`       | Enable debug logs (`true`/`false`)   | `true`         |

> Ensure `.env` files are excluded from version control via `.gitignore`.

---

## 3. Runtime Configuration

You can customize behavior at runtime using CLI arguments:

```bash
cargo run -- --env prod --debug false --port 8080
```

---

## 4. Default Values

| Parameter       | Description                         | Default |
|------------------|-------------------------------------|---------|
| `env`            | Environment mode                    | `dev`   |
| `port`           | Server port                         | `3000`  |
| `debug`          | Enable logging                      | `true`  |

---

## 5. Build & CI Configuration

- **GitHub Actions**: Configured via `.github/workflows/*.yml`  
  - Includes linting, tests, and formatting checks
- **Rust Toolchain**: Managed using `rust-toolchain.toml`
- **Formatting**: Enforced using `rustfmt.toml`
- **Lints**: Defined in `Clippy.toml` or inline via attributes

---

## 6. Safe Configuration Updates

- Always use branch-specific `.env` variants when testing features.
- Validate changes using `cargo check` and CI pipelines before merging.
- Do not expose secrets or API keys in the repo.

---

## 7. Example `.env`

```env
HEXA_ENV=prod
HEXA_PORT=8080
HEXA_DEBUG=false
```

---

## 8. Useful Commands

```bash
# Load environment and run
source .env && cargo run

# Test with custom config
HEXA_ENV=test HEXA_PORT=8888 cargo test
```

---

This guide helps contributors safely configure and run the project in various environments.