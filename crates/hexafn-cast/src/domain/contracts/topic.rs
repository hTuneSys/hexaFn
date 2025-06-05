// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

/// Trait for a pub/sub topic contract.
pub trait Topic {
    fn name(&self) -> &str;
}