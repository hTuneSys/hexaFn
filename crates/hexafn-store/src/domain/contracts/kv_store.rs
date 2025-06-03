// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

/// Trait for a generic key-value store contract.
pub trait KvStore {
    fn get(&self, namespace: &str, key: &str) -> Option<Vec<u8>>;
    fn put(&mut self, namespace: &str, key: &str, value: Vec<u8>);
    fn delete(&mut self, namespace: &str, key: &str);
}