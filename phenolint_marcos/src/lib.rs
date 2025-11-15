mod utils;

use crate::utils::extract_rule_id;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn register_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let struct_name = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        _ => panic!("lint_rule can only be applied to structs"),
    };

    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    let expanded = quote! {
        #input

        impl LintRule for #struct_name {
            const RULE_ID: &'static str = #rule_id;
        }

        inventory::submit! {
            LintingPolicy::<<#struct_name as RuleCheck>::CheckType> {
                rule_id: #rule_id,
                factory: |context: &LinterContext| #struct_name::from_context(context),
            }
        }

    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input
        impl RulePatch for #name {
            const RULE_ID: &'static str = #rule_id;
        }

        ::inventory::submit! {
            crate::patches::patch_registration::PatchRegistration {
                rule_id: #rule_id,
                register: |registry| {
                    registry.register(#rule_id, #name);
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_report(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input
        impl RuleReport for #name {
            const RULE_ID: &'static str = #rule_id;
        }

        ::inventory::submit! {
            crate::report::report_registration::ReportRegistration {
                rule_id: #rule_id,
                register: |registry| {
                    registry.register(#rule_id, #name);
                }
            }
        }
    };

    TokenStream::from(expanded)
}
