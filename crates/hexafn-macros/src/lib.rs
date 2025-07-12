// SPDX-FileCopyrightText: 2025 Husamettin ARABACI
// SPDX-License-Identifier: MIT

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(hexafn_formatter)]
pub fn derive_hexafn_formatter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl DataFormatter for #name {
            fn transform(&self, input: RawData) -> Result<FormattedData, String> {
                // Default: just wrap as JSON
                Ok(FormattedData {
                    structured_data: serde_json::json!({"data": String::from_utf8_lossy(&input.content)}),
                    schema: None,
                    format: DataFormat::Json,
                    validation_errors: vec![],
                    metadata: input.metadata.clone(),
                })
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(hexafn_function)]
pub fn derive_hexafn_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl FunctionRuntime for #name {
            fn execute(&self, context: FunctionContext) -> Result<ExecutionResult, String> {
                let mut outputs = std::collections::HashMap::new();
                outputs.insert("result".to_string(), "ok".to_string());
                Ok(ExecutionResult::new(
                    ExecutionStatus::Success,
                    outputs,
                    None,
                    std::time::Duration::from_millis(1),
                    0,
                ))
            }
        }
    };
    expanded.into()
}
