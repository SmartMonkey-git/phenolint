mod doc_string;
mod utils;

use crate::doc_string::{check_rule_docs_format, extract_doc_string};
use crate::utils::extract_rule_id;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, Span as Span2};
use quote::quote;
use syn::{Item, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn register_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let doc_string = extract_doc_string(&input);
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    check_rule_docs_format(&doc_string, &rule_id);
    let struct_name = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        _ => panic!("register_rule can only be applied to structs"),
    };

    let upper_snake_case_struct_name = struct_name.to_string().to_case(Case::Snake).to_uppercase();
    let upper_snake_case_struct = Ident2::new(&upper_snake_case_struct_name, Span2::call_site());

    let expanded = quote! {
        #input

        impl LintRule for #struct_name {
            const RULE_ID: &'static str = #rule_id;
        }

        static #upper_snake_case_struct: OnceLock<Arc<Result<BoxedRuleCheck<<#struct_name as RuleCheck>::CheckType>, FromContextError>>> = OnceLock::new();

        inventory::submit! {
            LintingPolicy::<<#struct_name as RuleCheck>::CheckType> {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    Arc::clone(#upper_snake_case_struct.get_or_init(|| {
                        Arc::new(#struct_name::from_context(context))
                    }))
                },
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
            PatchRegistration {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    #name::from_context(context)
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
            ReportRegistration {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    #name::from_context(context)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
