// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

/// Trait for external integration contracts.
pub trait Integration {
    fn name(&self) -> &str;
}