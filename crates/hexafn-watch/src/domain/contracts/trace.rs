// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

/// Trait for distributed tracing.
pub trait Trace {
    fn start_span(&self, name: &str);
}